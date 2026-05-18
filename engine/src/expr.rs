use std::{fmt::Write, rc::Rc};

use crate::i_tab::{ITab, Id};

/// Expressions that can occur in interpreted lambda calculus.
#[derive(Debug, Clone)]
pub enum Expr {
    /// Identifier.
    Ident(Id),
    /// Application of expression to argument.
    Apply(Rc<Expr>, Rc<Expr>),
    /// Definition of lambda function.
    Lambda(Id, Rc<Expr>),
    // Special builtins for better output.
    /// Internal counter type.
    Counter(usize),
    /// Builtin function that can increment counter.
    Increment,
    /// Builtin function that can create string from counter.
    Char,
    /// Internal string type.
    String(Vec<u8>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LastType {
    Root,
    Lambda,
    ApplyLeft,
    ApplyRight,
}

impl Expr {
    /// Format the expression into a valid lambda calculus string when given
    /// table of identifiers.
    pub fn to_string(&self, itab: &ITab, res: &mut String) {
        self.to_string_inner(itab, res, LastType::Root);
    }

    fn to_string_inner(&self, itab: &ITab, res: &mut String, last: LastType) {
        match self {
            Expr::Ident(id) => *res += &itab.name_of(*id),
            Expr::Apply(l, r) => {
                let brackets = last == LastType::ApplyRight;
                if brackets {
                    res.push('(');
                }
                l.to_string_inner(itab, res, LastType::ApplyLeft);
                res.push(' ');
                r.to_string_inner(itab, res, LastType::ApplyRight);
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
                expr.to_string_inner(itab, res, LastType::Lambda);
                if brackets {
                    res.push(')');
                }
            }
            Expr::Counter(cnt) => _ = write!(res, ":{cnt}:"),
            Expr::Increment => *res += "$increment",
            Expr::Char => *res += "$char",
            Expr::String(s) => *res += &String::from_utf8_lossy(s),
        }
    }
}
