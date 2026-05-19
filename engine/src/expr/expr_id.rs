use std::rc::{self, Rc};

/// Unique identificator owning an expression. The value of the underlaying
/// expression may change, but only to more expanded form.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprId(pub(super) Rc<usize>);

/// Unique identificator of expression which doesn't own it.
#[derive(Debug)]
pub struct WeakExprId(rc::Weak<usize>);

impl ExprId {
    /// Create new expression identificator.
    pub(super) fn new(idx: usize) -> Self {
        Self(idx.into())
    }

    /// Downgrade the expression identificator.
    pub(super) fn downgrade(&self) -> WeakExprId {
        WeakExprId(Rc::downgrade(&self.0))
    }

    /// Get the number of users of this expression.
    pub fn strong_cnt(&self) -> usize {
        Rc::strong_count(&self.0)
    }
}

impl WeakExprId {
    /// Get the number of users of this expression.
    pub fn strong_cnt(&self) -> usize {
        self.0.strong_count()
    }
}
