use crate::{
    expr::{ExprId, ExprTree},
    i_tab::ITab,
};

/// \f.f ((\x.f (x x)) \x.f (x x))
pub struct YComb(ExprId);

impl YComb {
    /// Create new Y combinator.
    pub fn new(et: &mut ExprTree, itab: &mut ITab) -> Self {
        itab.push_scope();
        let f = itab.insert("f");
        let x = itab.insert("x");
        itab.pop_scope();

        let xe = et.ident(x);
        let fe = et.ident(f);

        let xx = et.apply(xe.clone(), xe); // x x
        let fxx = et.apply(fe.clone(), xx); // f (x x)
        let dup = et.lambda(x, fxx); // \x.f (x x)
        let dup = et.apply(dup.clone(), dup); // (\x.f (x x)) \x.f (x x)
        let body = et.apply(fe, dup); // f ((\x.f (x x)) \x.f (x x))
        let lambda = et.lambda(f, body);
        Self(lambda)
    }
}

impl From<YComb> for ExprId {
    fn from(value: YComb) -> Self {
        value.0
    }
}
