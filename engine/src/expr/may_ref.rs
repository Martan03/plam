use crate::expr::{Expr, ExprId};

/// Possibly unset reference to or direct expression.
#[derive(Debug, Clone, Default)]
pub enum MayRef {
    #[default]
    None,
    Value(Expr),
    Ref(ExprId),
}
