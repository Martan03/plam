use crate::{
    expr::{ExprId, ExprTree},
    i_tab::ITab,
};

/// `\a b c f.f a b c`
#[derive(Clone, Debug)]
pub struct Triple(ExprId);

impl Triple {
    /// Create new triple constructor.
    pub fn new(et: &mut ExprTree, itab: &mut ITab) -> Self {
        itab.push_scope();
        let a = itab.insert("a");
        let b = itab.insert("b");
        let c = itab.insert("c");
        let f = itab.insert("f");
        itab.pop_scope();
        let fe = et.ident(f);
        let ae = et.ident(a);
        let be = et.ident(b);
        let ce = et.ident(c);
        let body = et.apply_many(fe, [ae, be, ce]);
        let lambda = et.lambda_many([a, b, c, f], body);
        Self(lambda)
    }

    /// `\f.f a b c`
    pub fn create(
        &self,
        et: &mut ExprTree,
        a: impl Into<ExprId>,
        b: impl Into<ExprId>,
        c: impl Into<ExprId>,
    ) -> ExprId {
        et.apply_many(self.0.clone(), [a.into(), b.into(), c.into()])
    }
}

impl From<Triple> for ExprId {
    fn from(value: Triple) -> Self {
        value.0
    }
}
