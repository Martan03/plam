use std::{collections::HashMap, io::BufRead};

use rand::RngExt;

use crate::{
    expr::{Expr, ExprId, ExprTree},
    i_tab::Id,
    lam_repr::StdinList,
};

/// Interpreter capable of interpreting lambda code.
pub struct Interpreter<'a, R> {
    pub et: &'a mut ExprTree,
    top: HashMap<Id, ExprId>,
    stdin: StdinList<R>,
    pub cache_limit: usize,
    apply_cache: HashMap<(ExprId, ExprId), ExprId>,
}

impl<'a, R: BufRead> Interpreter<'a, R> {
    /// Create new interpreter.
    pub fn new(
        et: &'a mut ExprTree,
        top: HashMap<Id, ExprId>,
        stdin: StdinList<R>,
    ) -> Self {
        Self {
            et,
            top,
            stdin,
            apply_cache: HashMap::new(),
            cache_limit: 0,
        }
    }

    /// Evaluate the given expression. If `expand` is true it will be evaluated
    /// to the furthest expanded version even if it doesn't produce single
    /// value.
    pub fn eval(&mut self, expr: &ExprId, expand: bool) {
        loop {
            let orig = expand.then(|| self.et[expr].clone());
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
                        if let Some(o) = orig {
                            self.et[expr] = o;
                        }
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
        self.eval(&l, true);
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
                self.et.replace(out, &body, id, &r);
                if self.cache_limit != 0 {
                    if self.apply_cache.len() >= self.cache_limit {
                        self.free_cache();
                    }
                    self.apply_cache.insert(lr, out.clone());
                }
                true
            }
            Expr::Increment => {
                self.eval(&r, true);
                if let Expr::Counter(cnt) = self.et[&r] {
                    self.et[out] = Expr::Counter(cnt + 1);
                    true
                } else {
                    false
                }
            }
            Expr::Char => {
                self.eval(&r, true);
                if let Expr::Counter(cnt) = self.et[&r] {
                    self.et[out] = Expr::String(vec![cnt as u8]);
                    true
                } else {
                    false
                }
            }
            Expr::String(_) => {
                self.eval(&r, true);
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

impl ReplaceResult {
    pub fn get_expr(self, et: &mut ExprTree, expr: &ExprId) -> Option<ExprId> {
        match self {
            ReplaceResult::Body => None,
            ReplaceResult::Expr => Some(expr.clone()),
            ReplaceResult::New(expr) => Some(et.insert(expr)),
        }
    }
}

impl ExprTree {
    pub fn replace(
        &mut self,
        out: &ExprId,
        body: &ExprId,
        id: Id,
        expr: &ExprId,
    ) {
        match self.replace_inner(body, id, expr) {
            ReplaceResult::Body => self.reference(out, body.clone()),
            ReplaceResult::Expr => self.reference(out, expr.clone()),
            ReplaceResult::New(expr) => self[out] = expr,
        }
    }

    fn replace_inner(
        &mut self,
        body: &ExprId,
        id: Id,
        expr: &ExprId,
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
                    if let Some(e) = self
                        .replace_inner(&body, id, expr)
                        .get_expr(self, expr)
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
                let l = self.replace_inner(&le, id, expr).get_expr(self, expr);
                let r = self.replace_inner(&re, id, expr).get_expr(self, expr);
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
}
