use std::rc::Rc;

use crate::{expr::Expr, i_tab::ITab};

/// `\a b.b`
#[derive(Debug, Clone)]
pub struct Second(Rc<Expr>);

impl Second {
    /// Craete new picker for second.
    pub fn new(itab: &mut ITab) -> Self {
        itab.push_scope();
        let a = itab.insert("a");
        let b = itab.insert("b");
        itab.pop_scope();
        let lambda = Expr::lambda([a, b], Expr::Ident(b));
        Self(lambda)
    }
}

impl From<Second> for Rc<Expr> {
    fn from(value: Second) -> Self {
        value.0
    }
}
