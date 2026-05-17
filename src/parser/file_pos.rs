use std::{fmt::Display, path::Path, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilePos {
    pub line: usize,
    pub col: usize,
    pub path: Option<Rc<Path>>,
}

impl FilePos {
    pub fn new(line: usize, col: usize) -> Self {
        Self {
            line,
            col,
            path: None,
        }
    }

    pub fn step(&mut self, chr: char) {
        if chr == '\n' {
            self.step_line();
        } else {
            self.step_col();
        }
    }

    pub fn step_line(&mut self) {
        self.line += 1;
        self.col = 0;
    }

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
