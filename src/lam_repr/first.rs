use std::rc::Rc;

use crate::{expr::Expr, i_tab::ITab};

/// `\a b.a`
#[derive(Debug, Clone)]
pub struct First(Rc<Expr>);

impl First {
    /// Create new picker for first.
    pub fn new(itab: &mut ITab) -> Self {
        itab.push_scope();
        let a = itab.insert("a");
        let b = itab.insert("b");
        itab.pop_scope();
        let lambda = Expr::lambda([a, b], Expr::Ident(a));
        Self(lambda)
    }
}

impl From<First> for Rc<Expr> {
    fn from(value: First) -> Self {
        value.0
    }
}
