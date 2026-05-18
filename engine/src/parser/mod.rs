use std::{collections::HashMap, rc::Rc};

use thiserror::Error;

use crate::{
    err::Result,
    expr::Expr,
    i_tab::{ITab, Id},
    parser::{file_pos::FilePos, lexer::Lexer, token::Token},
};

mod file_pos;
mod lexer;
mod token;

#[derive(Debug, Error)]
pub enum ParseErrorKind {
    #[error("Unexpected token `{0:?}`.")]
    UnexpectedToken(Token),
    #[error("Expected identifier.")]
    ExpectedIdentifier,
}

/// Error produced by parser.
#[derive(Debug, Error)]
#[error("{pos}: {kind}")]
pub struct ParseError {
    #[source]
    pub kind: ParseErrorKind,
    pub pos: FilePos,
}

/// Parse the given input stream into expressions. Update the top level
/// definitions and identifier table.
pub fn parse<I: Iterator<Item = Result<char>>>(
    itab: &mut ITab,
    input: I,
    defs: &mut HashMap<Id, Rc<Expr>>,
) -> Result<Vec<Rc<Expr>>> {
    itab.reset_scopes();
    let mut parser = Parser::new(itab, Lexer::new(input), defs);
    let res = parser.parse();
    parser.itab.reset_scopes();
    res?;
    Ok(parser.exprs)
}

#[derive(Debug)]
struct Parser<'a, I> {
    pub itab: &'a mut ITab,
    pub defs: &'a mut HashMap<Id, Rc<Expr>>,
    pub exprs: Vec<Rc<Expr>>,
    lexer: Lexer<I>,
    cur: Token,
}

impl<'a, I: Iterator<Item = Result<char>>> Parser<'a, I> {
    /// Create new parser.
    pub fn new(
        itab: &'a mut ITab,
        lexer: Lexer<I>,
        defs: &'a mut HashMap<Id, Rc<Expr>>,
    ) -> Self {
        Self {
            itab,
            defs,
            exprs: vec![],
            lexer,
            cur: Token::Eof,
        }
    }

    /// Parse the input.
    pub fn parse(&mut self) -> Result<()> {
        self.next()?;
        while self.cur != Token::Eof {
            self.read_stmt()?;
        }
        Ok(())
    }

    fn read_stmt(&mut self) -> Result<()> {
        match self.cur {
            Token::Semicol => _ = self.next()?,
            Token::Let => self.read_let()?,
            _ => {
                let expr = self.read_expr()?;
                self.exprs.push(expr.into());
            }
        }
        Ok(())
    }

    fn read_let(&mut self) -> Result<()> {
        self.next()?;
        self.expect(Token::Ident)?;
        let name = self.lexer.last_id().to_string();
        self.next()?;

        self.expect(Token::Eq)?;
        self.skip(Token::Eq)?;
        let expr = self.read_expr()?;

        let id = self.itab.insert(&name);
        self.defs.insert(id, expr.into());

        Ok(())
    }

    fn read_expr(&mut self) -> Result<Expr> {
        let mut res = self.read_item()?;
        while !matches!(
            self.cur,
            Token::Let | Token::Eof | Token::Right | Token::Semicol
        ) {
            res = Expr::Apply(res.into(), self.read_item()?.into());
        }
        Ok(res)
    }

    fn read_item(&mut self) -> Result<Expr> {
        match self.cur {
            Token::Lambda => self.read_lambda(),
            Token::Left => self.read_bracket(),
            Token::Ident => self.read_existing_ident().map(Expr::Ident),
            _ => Err(self.err_unexpected_token().into()),
        }
    }

    fn read_lambda(&mut self) -> Result<Expr> {
        self.next()?;
        self.itab.push_scope();

        let mut idents = vec![];
        while self.cur == Token::Ident {
            idents.push(self.read_new_ident()?);
        }
        if idents.is_empty() {
            return Err(self.err_expected_identifier().into());
        }

        self.skip(Token::Dot)?;
        let mut res = self.read_expr()?;
        self.itab.pop_scope();

        for i in idents.into_iter().rev() {
            res = Expr::Lambda(i, res.into());
        }
        Ok(res)
    }

    fn read_bracket(&mut self) -> Result<Expr> {
        self.next()?;
        let res = self.read_expr()?;
        self.skip(Token::Right)?;
        Ok(res)
    }

    fn read_existing_ident(&mut self) -> Result<Id> {
        let id = self.itab.get(self.lexer.last_id());
        self.next()?;
        Ok(id)
    }

    fn read_new_ident(&mut self) -> Result<Id> {
        let id = self.itab.insert(self.lexer.last_id());
        self.next()?;
        Ok(id)
    }

    fn skip(&mut self, t: Token) -> Result<Token> {
        self.expect(t)?;
        self.next()
    }

    fn next(&mut self) -> Result<Token> {
        self.cur = self.lexer.next()?;
        Ok(self.cur)
    }

    fn expect(&self, t: Token) -> Result<()> {
        if self.cur != t {
            Err(self.err_unexpected_token().into())
        } else {
            Ok(())
        }
    }

    fn err_expected_identifier(&self) -> ParseError {
        self.err(ParseErrorKind::ExpectedIdentifier)
    }

    fn err_unexpected_token(&self) -> ParseError {
        self.err(ParseErrorKind::UnexpectedToken(self.cur))
    }

    fn err(&self, kind: ParseErrorKind) -> ParseError {
        ParseError {
            kind,
            pos: self.lexer.last_pos(),
        }
    }
}
