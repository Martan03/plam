use std::collections::HashMap;

use thiserror::Error;

use crate::{
    err::Result,
    expr::{ExprId, ExprTree},
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
    et: &mut ExprTree,
    input: I,
    defs: &mut HashMap<Id, ExprId>,
) -> Result<Vec<ExprId>> {
    itab.reset_scopes();
    let mut parser = Parser::new(itab, et, Lexer::new(input), defs);
    let res = parser.parse();
    parser.itab.reset_scopes();
    res?;
    Ok(parser.exprs)
}

#[derive(Debug)]
struct Parser<'a, I> {
    pub itab: &'a mut ITab,
    pub et: &'a mut ExprTree,
    pub defs: &'a mut HashMap<Id, ExprId>,
    pub exprs: Vec<ExprId>,
    lexer: Lexer<I>,
    cur: Token,
}

impl<'a, I: Iterator<Item = Result<char>>> Parser<'a, I> {
    /// Create new parser.
    pub fn new(
        itab: &'a mut ITab,
        et: &'a mut ExprTree,
        lexer: Lexer<I>,
        defs: &'a mut HashMap<Id, ExprId>,
    ) -> Self {
        Self {
            itab,
            et,
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
                self.exprs.push(expr);
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
        self.defs.insert(id, expr);

        Ok(())
    }

    fn read_expr(&mut self) -> Result<ExprId> {
        let mut res = self.read_item()?;
        while !matches!(
            self.cur,
            Token::Let | Token::Eof | Token::Right | Token::Semicol
        ) {
            let itm = self.read_item()?;
            res = self.et.apply(res, itm);
        }
        Ok(res)
    }

    fn read_item(&mut self) -> Result<ExprId> {
        match self.cur {
            Token::Lambda => self.read_lambda(),
            Token::Left => self.read_bracket(),
            Token::Ident => {
                self.read_existing_ident().map(|i| self.et.ident(i))
            }
            _ => Err(self.err_unexpected_token().into()),
        }
    }

    fn read_lambda(&mut self) -> Result<ExprId> {
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
            res = self.et.lambda(i, res);
        }
        Ok(res)
    }

    fn read_bracket(&mut self) -> Result<ExprId> {
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
