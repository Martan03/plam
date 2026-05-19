use std::mem;

use crate::expr::{Expr, ExprId, WeakExprId, may_ref::MayRef};

#[derive(Debug)]
pub struct Expression {
    pub id: WeakExprId,
    pub expr: MayRef,
}

impl Expression {
    pub fn new(id: &ExprId, expr: Expr) -> Self {
        Self {
            id: id.downgrade(),
            expr: MayRef::Value(expr),
        }
    }

    pub fn free(&mut self, freed: &mut Vec<usize>) {
        match mem::take(&mut self.expr) {
            MayRef::Ref(r) => {
                let ri = *r.0;
                if r.strong_cnt() == 1 {
                    freed.push(ri);
                }
            }
            MayRef::Value(Expr::Apply(l, r)) => {
                let li = *l.0;
                if l.strong_cnt() == 1 {
                    freed.push(li);
                }
                let ri = *l.0;
                if r.strong_cnt() == 1 {
                    freed.push(ri);
                }
            }
            MayRef::Value(Expr::Lambda(_, e)) => {
                let ei = *e.0;
                if e.strong_cnt() == 1 {
                    freed.push(ei);
                }
            }
            _ => {}
        }
    }
}
