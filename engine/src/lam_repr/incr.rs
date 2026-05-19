use std::rc::Rc;

use crate::{expr::Expr, i_tab::ITab};

/// `\a f x.a f (f x)`
#[derive(Debug, Clone)]
pub struct Incr(Rc<Expr>);

impl Incr {
    /// Create new increment function.
    pub fn new(itab: &mut ITab) -> Self {
        itab.push_scope();
        let a = itab.insert("a");
        let f = itab.insert("f");
        let x = itab.insert("x");
        itab.pop_scope();

        let fe = Rc::new(Expr::Ident(f));
        let fx = Rc::new(Expr::Apply(fe.clone(), Expr::Ident(x).into())); // f x
        let body = Expr::apply(Expr::Ident(a), [fe, fx]); // a f (f x)
        let lambda = Expr::lambda([a, f, x], body);
        Self(lambda)
    }

    /// Increment the given number by one.
    pub fn incr(&self, a: impl Into<Rc<Expr>>) -> Rc<Expr> {
        Expr::Apply(self.0.clone(), a.into()).into()
    }
}

impl From<Incr> for Rc<Expr> {
    fn from(value: Incr) -> Self {
        value.0
    }
}
