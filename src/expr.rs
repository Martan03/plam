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

impl Expr {
    pub fn to_string(&self, itab: &ITab, res: &mut String) {
        match self {
            Expr::Ident(id) => *res += &itab.name_of(*id),
            Expr::Apply(l, r) => {
                res.push('(');
                l.to_string(itab, res);
                res.push(' ');
                r.to_string(itab, res);
                res.push(')');
            }
            Expr::Lambda(id, expr) => {
                *res += "(\\";
                *res += &itab.name_of(*id);
                res.push('.');
                expr.to_string(itab, res);
                res.push(')');
            }
            Expr::Counter(cnt) => _ = write!(res, ":{cnt}:"),
            Expr::Increment => *res += "$increment",
        }
    }
}
