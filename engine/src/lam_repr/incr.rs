use crate::{
    expr::{ExprId, ExprTree},
    i_tab::ITab,
};

/// `\a f x.a f (f x)`
#[derive(Debug, Clone)]
pub struct Incr(ExprId);

impl Incr {
    /// Create new increment function.
    pub fn new(et: &mut ExprTree, itab: &mut ITab) -> Self {
        itab.push_scope();
        let a = itab.insert("a");
        let f = itab.insert("f");
        let x = itab.insert("x");
        itab.pop_scope();

        let fe = et.ident(f);
        let xe = et.ident(x);
        let ae = et.ident(a);
        let fx = et.apply(fe.clone(), xe); // f x
        let body = et.apply_many(ae, [fe, fx]); // a f (f x)
        let lambda = et.lambda_many([a, f, x], body);
        Self(lambda)
    }

    /// Increment the given number by one.
    pub fn incr(&self, et: &mut ExprTree, a: ExprId) -> ExprId {
        et.apply(self.0.clone(), a)
    }
}

impl From<Incr> for ExprId {
    fn from(value: Incr) -> Self {
        value.0
    }
}
