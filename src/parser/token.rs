#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Let,     // 'let'
    Eq,      // '='
    Lambda,  // '\'
    Dot,     // '.'
    Left,    // '('
    Right,   // ')'
    Ident,   // identifier
    Semicol, // ';'
    Eof,
}
