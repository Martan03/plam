use crate::{
    expr::{ExprId, ExprTree},
    i_tab::ITab,
};

/// `\a b.a`
#[derive(Debug, Clone)]
pub struct First(ExprId);

impl First {
    /// Create new picker for first.
    pub fn new(et: &mut ExprTree, itab: &mut ITab) -> Self {
        itab.push_scope();
        let a = itab.insert("a");
        let b = itab.insert("b");
        itab.pop_scope();
        let ae = et.ident(a);
        let lambda = et.lambda_many([a, b], ae);
        Self(lambda)
    }
}

impl From<First> for ExprId {
    fn from(value: First) -> Self {
        value.0
    }
}
