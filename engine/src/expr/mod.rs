use std::fmt::Write;

use crate::i_tab::{ITab, Id};

mod expr_id;
mod expr_tree;
mod expression;
mod may_ref;

pub use self::{expr_id::*, expr_tree::*};

/// Expressions that can occur in interpreted lambda calculus.
#[derive(Debug, Clone)]
pub enum Expr {
    /// Identifier.
    Ident(Id),
    /// Application of expression to argument.
    Apply(ExprId, ExprId),
    /// Definition of lambda function.
    Lambda(Id, ExprId),
    // Special builtins for better output.
    /// Internal counter type.
    Counter(usize),
    /// Builtin function that can increment counter.
    Increment,
    /// Builtin function that can create string from counter.
    Char,
    /// Internal string type.
    String(Vec<u8>),
    /// Stdin
    Stdin(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LastType {
    Root,
    Lambda,
    ApplyLeft,
    ApplyRight,
}

impl ExprTree {
    /// Allocate new identifier expression.
    pub fn ident(&mut self, id: Id) -> ExprId {
        self.insert(Expr::Ident(id))
    }

    /// Allocate new apply expression.
    pub fn apply(
        &mut self,
        left: impl Into<ExprId>,
        right: impl Into<ExprId>,
    ) -> ExprId {
        self.insert(Expr::Apply(left.into(), right.into()))
    }

    /// Allocate new lambda expression.
    pub fn lambda(&mut self, id: Id, body: impl Into<ExprId>) -> ExprId {
        self.insert(Expr::Lambda(id, body.into()))
    }

    /// Allocate new counter expression.
    pub fn counter(&mut self, cnt: usize) -> ExprId {
        self.insert(Expr::Counter(cnt))
    }

    /// Allocate new increment expression.
    pub fn increment(&mut self) -> ExprId {
        self.insert(Expr::Increment)
    }

    /// Allocate new char expression.
    pub fn char(&mut self) -> ExprId {
        self.insert(Expr::Char)
    }

    /// Allocate new stdin expression.
    pub fn stdin(&mut self, pos: usize) -> ExprId {
        self.insert(Expr::Stdin(pos))
    }

    /// Apply multiple arguments to an expression.
    pub fn apply_many<E: Into<ExprId>>(
        &mut self,
        left: impl Into<ExprId>,
        right: impl IntoIterator<Item = E>,
    ) -> ExprId {
        let mut res = left.into();
        for r in right {
            res = self.apply(res, r);
        }
        res
    }

    /// Create lambda with multiple arguments.
    pub fn lambda_many<I: DoubleEndedIterator<Item = Id>>(
        &mut self,
        ids: impl IntoIterator<Item = Id, IntoIter = I>,
        body: impl Into<ExprId>,
    ) -> ExprId {
        let mut res = body.into();
        for id in ids.into_iter().rev() {
            res = self.lambda(id, res);
        }
        res
    }

    /// Format the expression into a valid lambda calculus string when given
    /// table of identifiers.
    pub fn to_string(&self, expr: &ExprId, itab: &ITab, res: &mut String) {
        self.to_string_inner(expr, itab, res, LastType::Root);
    }

    fn to_string_inner(
        &self,
        expr: &ExprId,
        itab: &ITab,
        res: &mut String,
        last: LastType,
    ) {
        match &self[expr] {
            Expr::Ident(id) => *res += &itab.name_of(*id),
            Expr::Apply(l, r) => {
                let brackets = last == LastType::ApplyRight;
                if brackets {
                    res.push('(');
                }
                self.to_string_inner(l, itab, res, LastType::ApplyLeft);
                res.push(' ');
                self.to_string_inner(r, itab, res, LastType::ApplyRight);
                if brackets {
                    res.push(')');
                }
            }
            Expr::Lambda(id, expr) => {
                let brackets =
                    matches!(last, LastType::ApplyLeft | LastType::ApplyRight);
                if brackets {
                    res.push('(');
                }
                res.push('\\');
                *res += &itab.name_of(*id);
                res.push('.');
                self.to_string_inner(expr, itab, res, LastType::Lambda);
                if brackets {
                    res.push(')');
                }
            }
            Expr::Counter(cnt) => _ = write!(res, ":{cnt}:"),
            Expr::Increment => *res += "$increment",
            Expr::Char => *res += "$char",
            Expr::String(s) => *res += &String::from_utf8_lossy(s),
            Expr::Stdin(0) => *res += "$stdin",
            Expr::Stdin(n) => _ = write!(res, "$stdin<{n}>"),
        }
    }
}
