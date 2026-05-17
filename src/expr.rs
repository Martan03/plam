use std::{fmt::Write, rc::Rc};

use crate::i_tab::{ITab, Id};

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Id),
    Apply(Rc<Expr>, Rc<Expr>),
    Lambda(Id, Rc<Expr>),
    // Special builtins for better output.
    Counter(usize),
    Increment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LastType {
    Root,
    Lambda,
    ApplyLeft,
    ApplyRight,
}

impl Expr {
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
        }
    }
}
