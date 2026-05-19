use crate::{
    expr::{ExprId, ExprTree},
    i_tab::ITab,
};

/// `\a b.b`
#[derive(Debug, Clone)]
pub struct Second(ExprId);

impl Second {
    /// Craete new picker for second.
    pub fn new(et: &mut ExprTree, itab: &mut ITab) -> Self {
        itab.push_scope();
        let a = itab.insert("a");
        let b = itab.insert("b");
        itab.pop_scope();
        let be = et.ident(b);
        let lambda = et.lambda_many([a, b], be);
        Self(lambda)
    }
}

impl From<Second> for ExprId {
    fn from(value: Second) -> Self {
        value.0
    }
}
