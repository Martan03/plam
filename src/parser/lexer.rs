use crate::{err::Result, parser::token::Token};

#[derive(Debug)]
pub struct Lexer<I> {
    input: I,
    cur: Option<char>,
    buf: String,
}

impl<I: Iterator<Item = Result<char>>> Lexer<I> {
    pub fn new(input: I) -> Self {
        Self {
            input,
            cur: None,
            buf: String::new(),
        }
    }

    pub fn last_id(&mut self) -> &str {
        &self.buf
    }

    pub fn next(&mut self) -> Result<Token> {
        self.buf.clear();
        self.skip_whitespace()?;

        if let Some(s) = self.read_special()? {
            self.cur = None;
            return Ok(s);
        }

        match self.cur()? {
            None => Ok(Token::Eof),
            _ => self.chunk(),
        }
    }

    fn read_special(&mut self) -> Result<Option<Token>> {
        match self.cur()? {
            Some('\\') => Ok(Some(Token::Lambda)),
            Some('=') => Ok(Some(Token::Eq)),
            Some('.') => Ok(Some(Token::Dot)),
            Some('(') => Ok(Some(Token::Left)),
            Some(')') => Ok(Some(Token::Right)),
            Some(';') => Ok(Some(Token::Semicol)),
            _ => Ok(None),
        }
    }

    fn chunk(&mut self) -> Result<Token> {
        while let Some(c) = self.cur()?
            && !c.is_ascii_whitespace()
            && !special(c)
        {
            self.buf.push(c);
            self.cur = None;
        }

        if self.buf == "let" {
            Ok(Token::Let)
        } else {
            Ok(Token::Ident)
        }
    }

    fn skip_whitespace(&mut self) -> Result<()> {
        while matches!(self.cur()?, Some(c) if c.is_ascii_whitespace()) {
            self.cur = None;
        }
        Ok(())
    }

    fn cur(&mut self) -> Result<Option<char>> {
        if self.cur.is_none() {
            self.cur = self.input.next().transpose()?;
        }
        Ok(self.cur)
    }
}

fn special(c: char) -> bool {
    matches!(c, '\\' | '=' | '.' | '(' | ')' | ';')
}
