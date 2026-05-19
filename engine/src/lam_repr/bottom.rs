use crate::{
    expr::{ExprId, ExprTree},
    lam_repr::{First, y_comb::YComb},
};

/// `Y \f _.f`
#[derive(Debug, Clone)]
pub struct Bottom(ExprId);

impl Bottom {
    /// Create new bottom.
    pub fn new(et: &mut ExprTree, y: YComb, first: First) -> Self {
        Self(et.apply(y, first))
    }
}

impl From<Bottom> for ExprId {
    fn from(value: Bottom) -> Self {
        value.0
    }
}
