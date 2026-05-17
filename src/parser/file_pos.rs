use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FilePos {
    pub line: usize,
    pub col: usize,
}

impl FilePos {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
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
        self.col = 1;
    }

    pub fn step_col(&mut self) {
        self.col += 1;
    }
}

impl Default for FilePos {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

impl Display for FilePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "??:{}:{}", self.line, self.col)
    }
}
