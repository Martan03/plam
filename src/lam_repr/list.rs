use crate::{
    expr::{ExprId, ExprTree},
    i_tab::ITab,
    lam_repr::{Bottom, False, Triple, True},
};

/// List representation. This doesn't corespond to single lambda.
#[derive(Debug)]
pub struct List {
    empty: ExprId,
    push: ExprId,
}

impl List {
    /// Create new list constructor.
    pub fn new(
        et: &mut ExprTree,
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

        let empty = triple.create(et, fals, bottom.clone(), bottom);

        let he = et.ident(h);
        let te = et.ident(t);
        let push_body = triple.create(et, tru, he, te);
        let push = et.lambda_many([h, t], push_body);

        Self { empty, push }
    }

    /// Create empty list `triple false _ _`
    pub fn empty(&self) -> ExprId {
        self.empty.clone()
    }

    /// Push the given item to the front of the given list
    pub fn push_to(
        &self,
        et: &mut ExprTree,
        item: impl Into<ExprId>,
        list: impl Into<ExprId>,
    ) -> ExprId {
        et.apply_many(self.push.clone(), [item.into(), list.into()])
    }
}
