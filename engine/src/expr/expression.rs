use std::mem;

use crate::expr::{Expr, ExprId, WeakExprId, may_ref::MayRef};

/// Expression stored in expression tree.
#[derive(Debug)]
pub struct Expression {
    /// Id of the expression.
    pub id: WeakExprId,
    /// The unerlaying expression. Possibly reference.
    pub expr: MayRef,
}

impl Expression {
    /// Create new unreferenced expression.
    pub fn new(id: &ExprId, expr: Expr) -> Self {
        Self {
            id: id.downgrade(),
            expr: MayRef::Value(expr),
        }
    }

    /// Remove this expression and get ids of all previously owned subtrees.
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
