use std::io::BufRead;

use termal::eprintacln;

use crate::{
    err::Result,
    expr::{ExprId, ExprTree},
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
    pub fn get(&mut self, et: &mut ExprTree, pos: usize) -> ExprId {
        if let Err(e) = self.read_enough(pos) {
            eprintacln!("{'m}warning: {'_}stdin read failed: {e}");
            self.list
                .push_to(et, self.bottom.clone(), self.list.empty())
        } else if self.data.len() <= pos {
            self.list.empty()
        } else {
            let chr = self.chars.get(self.data[pos]);
            let ns = et.stdin(pos + 1);
            self.list.push_to(et, chr, ns)
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
