use std::{collections::HashMap, io::BufRead, rc::Rc};

use crate::{expr::Expr, i_tab::Id, lam_repr::StdinList};

/// Interpreter capable of interpreting lambda code.
pub struct Interpreter<R> {
    top: HashMap<Id, Rc<Expr>>,
    stdin: StdinList<R>,
}

impl<R: BufRead> Interpreter<R> {
    /// Create new interpreter.
    pub fn new(top: HashMap<Id, Rc<Expr>>, stdin: StdinList<R>) -> Self {
        Self { top, stdin }
    }

    /// Evaluate the given expression. If `expand` is true it will be evaluated
    /// to the furthest expanded version even if it doesn't produce single
    /// value.
    pub fn eval(&mut self, mut expr: Rc<Expr>, expand: bool) -> Rc<Expr> {
        loop {
            expr = match &*expr {
                Expr::Ident(id) => {
                    if let Some(e) = self.eval_ident(*id) {
                        e
                    } else {
                        return expr;
                    }
                }
                Expr::Apply(l, r) => {
                    match self.eval_apply(l.clone(), r.clone()) {
                        Ok(r) => r,
                        Err(e) if expand => return e,
                        _ => return expr,
                    }
                }
                Expr::Stdin(n) => self.stdin.get(*n),
                Expr::Lambda(_, _)
                | Expr::Counter(_)
                | Expr::Increment
                | Expr::Char
                | Expr::String(_) => {
                    return expr;
                }
            };
        }
    }

    fn eval_ident(&self, id: Id) -> Option<Rc<Expr>> {
        self.get(id)
    }

    fn eval_apply(
        &mut self,
        l: Rc<Expr>,
        r: Rc<Expr>,
    ) -> Result<Rc<Expr>, Rc<Expr>> {
        let l = self.eval(l, true);
        match &*l {
            Expr::Lambda(_, _) => Ok(self.eval_apply_lambda(l, r)),
            Expr::Increment => self.eval_apply_increment(r),
            Expr::Char => self.eval_apply_char(r),
            Expr::String(_) => self.eval_apply_string(l, r),
            _ => Err(Rc::new(Expr::Apply(l, r))),
        }
    }

    fn eval_apply_lambda(&self, mut l: Rc<Expr>, r: Rc<Expr>) -> Rc<Expr> {
        let Expr::Lambda(id, body) = Rc::make_mut(&mut l) else {
            panic!();
        };
        body.replace(*id, r);
        body.clone()
    }

    fn eval_apply_increment(
        &mut self,
        r: Rc<Expr>,
    ) -> Result<Rc<Expr>, Rc<Expr>> {
        let r = self.eval(r, true);
        if let Expr::Counter(cnt) = &*r {
            Ok(Rc::new(Expr::Counter(*cnt + 1)))
        } else {
            Err(Rc::new(Expr::Apply(Rc::new(Expr::Increment), r)))
        }
    }

    fn eval_apply_char(&mut self, r: Rc<Expr>) -> Result<Rc<Expr>, Rc<Expr>> {
        let r = self.eval(r, true);
        if let Expr::Counter(cnt) = &*r {
            Ok(Rc::new(Expr::String(vec![*cnt as u8])))
        } else {
            Err(Rc::new(Expr::Apply(Rc::new(Expr::Char), r)))
        }
    }

    fn eval_apply_string(
        &mut self,
        mut l: Rc<Expr>,
        r: Rc<Expr>,
    ) -> Result<Rc<Expr>, Rc<Expr>> {
        let r = self.eval(r, true);
        let Expr::String(r) = &*r else {
            return Err(Rc::new(Expr::Apply(l, r)));
        };
        let Expr::String(v) = Rc::make_mut(&mut l) else {
            panic!();
        };
        v.extend_from_slice(r);
        Ok(l)
    }

    fn get(&self, id: Id) -> Option<Rc<Expr>> {
        self.top.get(&id).cloned()
    }
}

impl Expr {
    pub fn replace(self: &mut Rc<Self>, id: Id, expr: Rc<Expr>) {
        match &**self {
            Expr::Ident(i) => {
                if *i == id {
                    *self = expr
                }
            }
            Expr::Lambda(i, expr) if *i == id => {}
            Expr::Counter(_)
            | Expr::Stdin(_)
            | Expr::Increment
            | Expr::Char
            | Expr::String(_) => {}
            _ => match Rc::make_mut(self) {
                Expr::Ident(_)
                | Expr::Counter(_)
                | Expr::Increment
                | Expr::Char
                | Expr::Stdin(_)
                | Expr::String(_) => {
                    unreachable!()
                }
                Expr::Apply(l, r) => {
                    l.replace(id, expr.clone());
                    r.replace(id, expr);
                }
                Expr::Lambda(_, e) => e.replace(id, expr),
            },
        }
    }
}
