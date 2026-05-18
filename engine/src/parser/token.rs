/// Enumeration of recognized tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// The keyword `let`.
    Let,
    /// The operator `=`.
    Eq,
    /// The operator `\`.
    Lambda,
    /// The operator `.`.
    Dot,
    /// Left bracket `(`.
    Left,
    /// Right bracket `)`.
    Right,
    /// Identifier.
    Ident,
    /// Semicolon `;`.
    Semicol,
    /// End Of File.
    Eof,
}
