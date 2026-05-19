use crate::expr::{Expr, ExprId};

#[derive(Debug, Clone, Default)]
pub enum MayRef {
    #[default]
    None,
    Value(Expr),
    Ref(ExprId),
}
