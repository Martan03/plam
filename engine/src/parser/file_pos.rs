use std::{fmt::Display, path::Path, rc::Rc};

/// Position within a file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilePos {
    /// Line index starting at 1.
    pub line: usize,
    /// Column index starting at 1. May be 0 for newline characters.
    pub col: usize,
    /// Path to the file if known.
    pub path: Option<Rc<Path>>,
}

impl FilePos {
    /// Create new file position with unknown file.
    pub fn new(line: usize, col: usize) -> Self {
        Self {
            line,
            col,
            path: None,
        }
    }

    /// Move the file position according to the character.
    pub fn step(&mut self, chr: char) {
        if chr == '\n' {
            self.step_line();
        } else {
            self.step_col();
        }
    }

    /// Move the file position one line down.
    pub fn step_line(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    /// Move the file position one column right.
    pub fn step_col(&mut self) {
        self.col += 1;
    }
}

impl Default for FilePos {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

impl Display for FilePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(p) = &self.path {
            write!(f, "{}:{}:{}", p.display(), self.line, self.col)
        } else {
            write!(f, "??:{}:{}", self.line, self.col)
        }
    }
}
