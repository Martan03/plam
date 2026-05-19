use std::rc::{self, Rc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprId(pub(super) Rc<usize>);

#[derive(Debug)]
pub struct WeakExprId(rc::Weak<usize>);

impl ExprId {
    pub(super) fn new(idx: usize) -> Self {
        Self(idx.into())
    }

    pub fn downgrade(&self) -> WeakExprId {
        WeakExprId(Rc::downgrade(&self.0))
    }

    pub fn strong_cnt(&self) -> usize {
        Rc::strong_count(&self.0)
    }
}

impl WeakExprId {
    pub fn strong_cnt(&self) -> usize {
        self.0.strong_count()
    }
}
