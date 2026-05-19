use std::rc::Rc;

use crate::{expr::Expr, i_tab::ITab};

/// `\a b c f.f a b c`
#[derive(Clone, Debug)]
pub struct Triple(Rc<Expr>);

impl Triple {
    /// Create new triple constructor.
    pub fn new(itab: &mut ITab) -> Self {
        itab.push_scope();
        let a = itab.insert("a");
        let b = itab.insert("b");
        let c = itab.insert("c");
        let f = itab.insert("f");
        itab.pop_scope();
        let body = Expr::apply(
            Expr::Ident(f),
            [Expr::Ident(a), Expr::Ident(b), Expr::Ident(c)],
        );
        let lambda = Expr::lambda([a, b, c, f], body);
        Self(lambda)
    }

    /// `\f.f a b c`
    pub fn create(
        &self,
        a: impl Into<Rc<Expr>>,
        b: impl Into<Rc<Expr>>,
        c: impl Into<Rc<Expr>>,
    ) -> Rc<Expr> {
        Expr::apply(self.0.clone(), [a.into(), b.into(), c.into()])
    }
}

impl From<Triple> for Rc<Expr> {
    fn from(value: Triple) -> Self {
        value.0
    }
}
