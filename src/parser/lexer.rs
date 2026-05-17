use crate::{
    err::Result,
    parser::{file_pos::FilePos, token::Token},
};

#[derive(Debug)]
pub struct Lexer<I> {
    input: I,
    pos: FilePos,
    last_pos: FilePos,
    cur: Option<char>,
    buf: String,
}

impl<I: Iterator<Item = Result<char>>> Lexer<I> {
    pub fn new(input: I) -> Self {
        Self {
            input,
            pos: FilePos::default(),
            last_pos: FilePos::default(),
            cur: None,
            buf: String::new(),
        }
    }

    pub fn last_id(&self) -> &str {
        &self.buf
    }

    pub fn last_pos(&self) -> FilePos {
        self.last_pos
    }

    pub fn next(&mut self) -> Result<Token> {
        self.buf.clear();
        self.skip_whitespace()?;

        self.last_pos = self.pos;

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
            if let Some(c) = self.cur {
                self.pos.step(c);
            }
        }
        Ok(self.cur)
    }
}

fn special(c: char) -> bool {
    matches!(c, '\\' | '=' | '.' | '(' | ')' | ';')
}
