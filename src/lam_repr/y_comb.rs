use std::rc::Rc;

use crate::{expr::Expr, i_tab::ITab};

/// \f.f ((\x.f (x x)) \x.f (x x))
pub struct YComb(Rc<Expr>);

impl YComb {
    /// Create new Y combinator.
    pub fn new(itab: &mut ITab) -> Self {
        itab.push_scope();
        let f = itab.insert("f");
        let x = itab.insert("x");
        itab.pop_scope();

        let xe = Rc::new(Expr::Ident(x));
        let fe = Rc::new(Expr::Ident(f));

        let xx = Expr::Apply(xe.clone(), xe).into(); // x x
        let fxx = Expr::Apply(fe.clone(), xx).into(); // f (x x)
        let dup = Rc::new(Expr::Lambda(x, fxx)); // \x.f (x x)
        let dup = Expr::Apply(dup.clone(), dup).into(); // (\x.f (x x)) \x.f (x x)
        let body = Expr::Apply(fe, dup).into(); // f ((\x.f (x x)) \x.f (x x))
        let lambda = Expr::Lambda(f, body);
        Self(lambda.into())
    }
}

impl From<YComb> for Rc<Expr> {
    fn from(value: YComb) -> Self {
        value.0
    }
}
