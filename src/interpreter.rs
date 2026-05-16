use std::{collections::HashMap, rc::Rc};

use crate::{expr::Expr, i_tab::Id};

pub struct Interpreter {
    top: HashMap<Id, Rc<Expr>>,
}

impl Interpreter {
    pub fn new(top: HashMap<Id, Rc<Expr>>) -> Self {
        Self { top }
    }

    pub fn eval(&self, mut expr: Rc<Expr>) -> Rc<Expr> {
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
                    if let Some(e) = self.eval_apply(l.clone(), r.clone()) {
                        e
                    } else {
                        return expr;
                    }
                }
                Expr::Lambda(_, _) | Expr::Counter(_) | Expr::Increment => {
                    return expr;
                }
            };
        }
    }

    fn eval_ident(&self, id: Id) -> Option<Rc<Expr>> {
        self.get(id)
    }

    fn eval_apply(&self, l: Rc<Expr>, r: Rc<Expr>) -> Option<Rc<Expr>> {
        let l = self.eval(l);
        match &*l {
            Expr::Lambda(_, _) => Some(self.eval_apply_lambda(l, r)),
            Expr::Increment => self.eval_apply_increment(r),
            _ => None,
        }
    }

    fn eval_apply_lambda(&self, mut l: Rc<Expr>, r: Rc<Expr>) -> Rc<Expr> {
        let Expr::Lambda(id, body) = Rc::make_mut(&mut l) else {
            panic!();
        };
        body.replace(*id, r);
        body.clone()
    }

    fn eval_apply_increment(&self, r: Rc<Expr>) -> Option<Rc<Expr>> {
        let r = self.eval(r);
        if let Expr::Counter(cnt) = &*r {
            Some(Rc::new(Expr::Counter(*cnt + 1)))
        } else {
            None
        }
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
            Expr::Counter(_) | Expr::Increment => {}
            _ => match Rc::make_mut(self) {
                Expr::Ident(_) | Expr::Counter(_) | Expr::Increment => {
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
