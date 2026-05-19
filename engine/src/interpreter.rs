use std::{
    collections::HashMap,
    io::{BufRead, StdinLock},
};

use crate::{
    expr::{Expr, ExprId, ExprTree},
    i_tab::{ITab, Id},
    lam_repr::{
        Bottom, First, Incr, List, PeanoChars, Second, StdinList, Triple,
        YComb,
    },
};

use rand::RngExt;

/// Interpreter capable of interpreting lambda code.
pub struct Interpreter<'a, R> {
    pub et: &'a mut ExprTree,
    pub itab: &'a ITab,
    top: HashMap<Id, ExprId>,
    stdin: StdinList<R>,
    pub cache_limit: usize,
    apply_cache: HashMap<(ExprId, ExprId), ExprId>,
    view_buf: HashMap<ExprId, Option<LazyExpr>>,
    base: Option<ExprId>,
}

pub fn init_interpreter<'a>(
    et: &'a mut ExprTree,
    defs: HashMap<Id, ExprId>,
    itab: &'a mut ITab,
) -> Interpreter<'a, StdinLock<'static>> {
    let y = YComb::new(et, itab);
    let first = First::new(et, itab);
    let triple = Triple::new(et, itab);
    let second = Second::new(et, itab);
    let incr = Incr::new(et, itab);
    let bottom = Bottom::new(et, y, first.clone());
    let list =
        List::new(et, triple, first, second.clone(), bottom.clone(), itab);
    let pean_chars = PeanoChars::new(et, &incr, second);
    let stdin = std::io::stdin().lock();
    let stdin_list = StdinList::new(stdin, pean_chars, list, bottom);
    Interpreter::new(et, defs, stdin_list, itab)
}

impl<'a, R: BufRead> Interpreter<'a, R> {
    /// Create new interpreter.
    pub fn new(
        et: &'a mut ExprTree,
        top: HashMap<Id, ExprId>,
        stdin: StdinList<R>,
        itab: &'a ITab,
    ) -> Self {
        Self {
            et,
            itab,
            top,
            stdin,
            apply_cache: HashMap::new(),
            cache_limit: 10000,
            view_buf: HashMap::new(),
            base: None,
        }
    }

    /// Evaluate the given expression. If `expand` is true it will be evaluated
    /// to the furthest expanded version even if it doesn't produce single
    /// value.
    pub fn eval(&mut self, expr: &ExprId) {
        self.base = None; //Some(expr.clone());
        self.eval_inner(expr);
    }

    fn eval_inner(&mut self, expr: &ExprId) {
        loop {
            if let Some(b) = &self.base {
                let mut s = String::new();
                self.et.to_string(b, self.itab, &mut s);
                println!("{s}");
            }
            match &self.et[expr] {
                Expr::Ident(id) => {
                    if let Some(e) = self.eval_ident(*id) {
                        self.et.reference(expr, e);
                    } else {
                        return;
                    }
                }
                Expr::Apply(l, r) => {
                    if !self.eval_apply(expr, l.clone(), r.clone()) {
                        return;
                    }
                }
                Expr::Stdin(n) => {
                    let n = *n;
                    let res = self.stdin.get(self.et, n);
                    self.et.reference(expr, res);
                }
                Expr::Lambda(_, _)
                | Expr::Counter(_)
                | Expr::Increment
                | Expr::Char
                | Expr::String(_) => {
                    return;
                }
            };
        }
    }

    fn eval_ident(&self, id: Id) -> Option<ExprId> {
        self.get(id)
    }

    fn eval_apply(&mut self, out: &ExprId, l: ExprId, r: ExprId) -> bool {
        self.eval_inner(&l);
        match &self.et[&l] {
            Expr::Lambda(id, body) => {
                let lr = (l.clone(), r.clone());
                if self.cache_limit != 0
                    && let Some(e) = self.apply_cache.get(&lr)
                {
                    self.et.reference(out, e.clone());
                    return true;
                }
                let body = body.clone();
                let id = *id;
                self.et.replace(out, &body, id, &r, &mut self.view_buf);
                if self.cache_limit != 0 {
                    if self.apply_cache.len() >= self.cache_limit {
                        self.free_cache();
                    }
                    self.apply_cache.insert(lr, out.clone());
                }
                true
            }
            Expr::Increment => {
                self.eval_inner(&r);
                if let Expr::Counter(cnt) = self.et[&r] {
                    self.et[out] = Expr::Counter(cnt + 1);
                    true
                } else {
                    false
                }
            }
            Expr::Char => {
                self.eval_inner(&r);
                if let Expr::Counter(cnt) = self.et[&r] {
                    self.et[out] = Expr::String(vec![cnt as u8]);
                    true
                } else {
                    false
                }
            }
            Expr::String(_) => {
                self.eval_inner(&r);
                if !matches!(self.et[&r], Expr::String(_)) {
                    return false;
                }
                self.et[out] = Expr::Char;
                let Expr::String(mut s) = self.et.take_out(l) else {
                    unreachable!();
                };
                let Expr::String(r) = &self.et[&r] else {
                    unreachable!();
                };
                s.extend_from_slice(r);
                self.et[out] = Expr::String(s);
                true
            }
            _ => false,
        }
    }

    fn get(&self, id: Id) -> Option<ExprId> {
        self.top.get(&id).cloned()
    }

    fn free_cache(&mut self) {
        let mut rng = rand::rng();
        self.apply_cache.retain(|_, _| rng.random());
    }
}

#[derive(Debug)]
pub enum ReplaceResult {
    Body,
    Expr,
    New(Expr),
}

impl ExprTree {
    pub fn replace(
        &mut self,
        out: &ExprId,
        body: &ExprId,
        id: Id,
        expr: &ExprId,
        viewed: &mut HashMap<ExprId, Option<LazyExpr>>,
    ) {
        match self.replace_inner(body, id, expr, viewed) {
            ReplaceResult::Body => self.reference(out, body.clone()),
            ReplaceResult::Expr => self.reference(out, expr.clone()),
            ReplaceResult::New(expr) => self[out] = expr,
        }
        viewed.clear();
    }

    fn replace_inner(
        &mut self,
        body: &ExprId,
        id: Id,
        expr: &ExprId,
        viewed: &mut HashMap<ExprId, Option<LazyExpr>>,
    ) -> ReplaceResult {
        match &self[body] {
            Expr::Ident(i) => {
                if *i == id {
                    ReplaceResult::Expr
                } else {
                    ReplaceResult::Body
                }
            }
            Expr::Lambda(i, body) => {
                let i = *i;
                if i == id {
                    ReplaceResult::Body
                } else {
                    let body = body.clone();
                    if let Some(e) =
                        self.checked_replace(&body, id, expr, viewed)
                    {
                        ReplaceResult::New(Expr::Lambda(i, e))
                    } else {
                        ReplaceResult::Body
                    }
                }
            }
            Expr::Apply(l, r) => {
                let le = l.clone();
                let re = r.clone();
                let l = self.checked_replace(&le, id, expr, viewed);
                let r = self.checked_replace(&re, id, expr, viewed);
                if l.is_none() && r.is_none() {
                    ReplaceResult::Body
                } else {
                    let l = l.unwrap_or(le);
                    let r = r.unwrap_or(re);
                    ReplaceResult::New(Expr::Apply(l, r))
                }
            }
            Expr::Counter(_)
            | Expr::Stdin(_)
            | Expr::Increment
            | Expr::Char
            | Expr::String(_) => ReplaceResult::Body,
        }
    }

    fn checked_replace(
        &mut self,
        body: &ExprId,
        id: Id,
        expr: &ExprId,
        viewed: &mut HashMap<ExprId, Option<LazyExpr>>,
    ) -> Option<ExprId> {
        if let Some(res) = viewed.get_mut(body) {
            return res.as_mut().map(|a| a.get(self));
        }
        viewed.insert(body.clone(), Some(LazyExpr::empty()));
        let res = self.replace_inner(body, id, expr, viewed);
        let res = match res {
            ReplaceResult::Body => None,
            ReplaceResult::Expr => {
                viewed
                    .get_mut(body)
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .set(expr.clone());
                Some(expr.clone())
            }
            ReplaceResult::New(expr) => {
                let d =
                    viewed.get_mut(body).unwrap().as_mut().unwrap().get(self);
                self[&d] = expr;
                Some(d)
            }
        };
        if res.is_none() {
            viewed.insert(body.clone(), None);
        }
        res
    }
}

/// Lazily allocate expression id.
pub struct LazyExpr {
    value: Option<ExprId>,
}

impl LazyExpr {
    /// Create unallocated expression id.
    pub fn empty() -> Self {
        Self { value: None }
    }

    /// Set the expression.
    pub fn set(&mut self, expr: ExprId) {
        self.value = Some(expr);
    }

    /// Get the expression. This may allocate it.
    pub fn get(&mut self, et: &mut ExprTree) -> ExprId {
        if let Some(v) = &self.value {
            v.clone()
        } else {
            // Char is just dummy variant as it is one of the most inert
            // expression types.
            let res = et.char();
            self.value = Some(res.clone());
            res
        }
    }
}
