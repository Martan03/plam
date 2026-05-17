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
        self.last_pos.clone()
    }

    pub fn next(&mut self) -> Result<Token> {
        self.buf.clear();
        self.skip_whitespace()?;

        self.last_pos = self.pos.clone();

        if let Some(s) = self.read_special()? {
            self.cur = None;
            return Ok(s);
        }

        match self.cur()? {
            None => Ok(Token::Eof),
            _ => {
                let res = self.chunk()?;
                if self.buf == "//" {
                    self.line_comment()?;
                    self.next()
                } else if self.buf == "/*" {
                    self.block_comment()?;
                    self.next()
                } else {
                    Ok(res)
                }
            }
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

    fn line_comment(&mut self) -> Result<()> {
        while matches!(self.cur()?, Some(c) if c != '\n') {
            self.cur = None;
        }
        Ok(())
    }

    fn block_comment(&mut self) -> Result<()> {
        let mut nest = 1;
        while self.cur()?.is_some() {
            if self.cur()? == Some('*') {
                self.cur = None;
                if self.cur()? == Some('/') {
                    self.cur = None;
                    nest -= 1;
                    if nest == 0 {
                        return Ok(());
                    }
                }
            } else if self.cur()? == Some('/') {
                self.cur = None;
                if self.cur()? == Some('*') {
                    nest += 1;
                }
            } else {
                self.cur = None;
            }
        }
        Ok(())
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
