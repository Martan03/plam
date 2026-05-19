use std::rc::Rc;

use crate::{
    expr::Expr,
    lam_repr::{First, y_comb::YComb},
};

/// `Y \f _.f`
#[derive(Debug, Clone)]
pub struct Bottom(Rc<Expr>);

impl Bottom {
    /// Create new bottom.
    pub fn new(y: YComb, first: First) -> Self {
        Self(Expr::Apply(y.into(), first.into()).into())
    }
}

impl From<Bottom> for Rc<Expr> {
    fn from(value: Bottom) -> Self {
        value.0
    }
}
