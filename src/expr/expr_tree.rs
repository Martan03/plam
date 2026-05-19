use std::{
    collections::BTreeSet,
    mem,
    ops::{Index, IndexMut},
};

use crate::expr::{Expr, ExprId, expression::Expression, may_ref::MayRef};

/// Automatic memory management for expression.
/// 
/// This is basically simple garbage collector.
#[derive(Debug, Default)]
pub struct ExprTree {
    exprs: Vec<Expression>,
    free: BTreeSet<usize>,
}

impl ExprTree {
    /// Create empty expression tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Remove the given id from the expression tree.
    pub fn take_out(&mut self, id: ExprId) -> Expr {
        // TODO: optimize this to remove the clone in some cases
        self[&id].clone()
    }

    /// Create make the referee reference another expression.
    pub fn reference(&mut self, referee: &ExprId, target: ExprId) {
        let target = self.resolve_opt(target);
        self.exprs[*referee.0].expr = MayRef::Ref(target);
    }

    /// Insert new expression to the tree.
    pub fn insert(&mut self, expr: Expr) -> ExprId {
        self.reserve_1();

        if let Some(idx) = self.free.pop_first() {
            let id = ExprId::new(idx);
            self.exprs[idx] = Expression::new(&id, expr);
            return id;
        }

        let idx = self.exprs.len();
        let id = ExprId::new(idx);
        self.exprs.push(Expression::new(&id, expr));
        id
    }

    fn reserve_1(&mut self) {
        if !self.free.is_empty() {
            return;
        }

        if self.exprs.capacity() > self.exprs.len() {
            return;
        }

        self.resize();
    }

    fn resize(&mut self) {
        let mut freed = vec![];
        for (i, e) in self.exprs.iter_mut().enumerate() {
            if e.id.strong_cnt() == 0 {
                e.free(&mut freed);
                self.free.insert(i);
            }
        }

        let mut buf = vec![];
        while !freed.is_empty() {
            mem::swap(&mut buf, &mut freed);
            freed.clear();
            for i in &buf {
                let i = *i;
                let e = &mut self.exprs[i];
                if e.id.strong_cnt() == 0 {
                    e.free(&mut freed);
                    self.free.insert(i);
                }
            }
        }

        self.exprs.reserve(self.exprs.len());
    }

    fn resolve(&self, id: &ExprId) -> &Expression {
        let exp = &self.exprs[*id.0];
        match &exp.expr {
            MayRef::None => panic!(),
            MayRef::Value(_) => exp,
            MayRef::Ref(expr_id) => self.resolve(expr_id),
        }
    }

    fn resolve_opt(&mut self, id: ExprId) -> ExprId {
        match &self.exprs[*id.0].expr {
            MayRef::None => panic!(),
            MayRef::Value(_) => id.clone(),
            MayRef::Ref(expr_id) => {
                let i = expr_id.clone();
                let i = self.resolve_opt(i);
                self.exprs[*id.0].expr = MayRef::Ref(i.clone());
                i
            }
        }
    }
}

impl Index<&ExprId> for ExprTree {
    type Output = Expr;

    fn index(&self, index: &ExprId) -> &Self::Output {
        let MayRef::Value(e) = &self.resolve(index).expr else {
            panic!()
        };
        e
    }
}

impl IndexMut<&ExprId> for ExprTree {
    fn index_mut(&mut self, index: &ExprId) -> &mut Self::Output {
        let id = self.resolve_opt(index.clone());
        let MayRef::Value(e) = &mut self.exprs[*id.0].expr else {
            panic!()
        };
        e
    }
}
