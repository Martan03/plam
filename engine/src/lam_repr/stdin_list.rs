use std::{io::BufRead, rc::Rc};

use termal::eprintacln;

use crate::{
    err::Result,
    expr::Expr,
    lam_repr::{Bottom, List, PeanoChars},
};

/// Structure formulating list from stdin data.
#[derive(Debug)]
pub struct StdinList<R> {
    reader: R,
    chars: PeanoChars,
    list: List,
    bottom: Bottom,
    data: Vec<u8>,
}

impl<R: BufRead> StdinList<R> {
    /// Create new stdin list.
    pub fn new(
        reader: R,
        chars: PeanoChars,
        list: List,
        bottom: Bottom,
    ) -> Self {
        Self {
            reader,
            chars,
            list,
            bottom,
            data: vec![],
        }
    }

    /// Get character from stdin at the given position.
    pub fn get(&mut self, pos: usize) -> Rc<Expr> {
        if let Err(e) = self.read_enough(pos) {
            eprintacln!("{'m}warning: {'_}stdin read failed: {e}");
            self.list.push_to(self.bottom.clone(), self.list.empty())
        } else if self.data.len() <= pos {
            self.list.empty()
        } else {
            let chr = self.chars.get(self.data[pos]);
            self.list.push_to(chr, Expr::Stdin(pos + 1))
        }
    }

    fn read_enough(&mut self, pos: usize) -> Result<()> {
        while self.data.len() <= pos {
            let b = self.reader.fill_buf()?;
            if b.is_empty() {
                return Ok(());
            }
            self.data.extend_from_slice(b);
            let len = b.len();
            self.reader.consume(len);
        }
        Ok(())
    }
}
