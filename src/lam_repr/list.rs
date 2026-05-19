use std::rc::Rc;

use crate::{
    expr::Expr,
    i_tab::ITab,
    lam_repr::{Bottom, False, Triple, True},
};

/// List representation. This doesn't corespond to single lambda.
#[derive(Debug)]
pub struct List {
    empty: Rc<Expr>,
    push: Rc<Expr>,
}

impl List {
    /// Create new list constructor.
    pub fn new(
        triple: Triple,
        tru: True,
        fals: False,
        bottom: Bottom,
        itab: &mut ITab,
    ) -> Self {
        itab.push_scope();
        let h = itab.insert("h");
        let t = itab.insert("t");
        itab.pop_scope();

        let empty = triple.create(fals, bottom.clone(), bottom);

        let push_body = triple.create(tru, Expr::Ident(h), Expr::Ident(t));
        let push = Expr::lambda([h, t], push_body);

        Self { empty, push }
    }

    /// Create empty list `triple false _ _`
    pub fn empty(&self) -> Rc<Expr> {
        self.empty.clone()
    }

    /// Push the given item to the front of the given list
    pub fn push_to(
        &self,
        item: impl Into<Rc<Expr>>,
        list: impl Into<Rc<Expr>>,
    ) -> Rc<Expr> {
        Expr::apply(self.push.clone(), [item.into(), list.into()])
    }
}
