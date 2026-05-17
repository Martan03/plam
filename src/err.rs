use std::{borrow::Cow, fmt::Display, path::Path};

use pareg::ArgError;
use thiserror::Error;

use crate::parser::ParseError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ErrKind {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Pareg(#[from] ArgError),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrKind,
    pub msg: Cow<'static, str>,
}

impl Error {
    pub fn new(
        kind: impl Into<ErrKind>,
        msg: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            kind: kind.into(),
            msg: msg.into(),
        }
    }

    pub fn with_path(mut self, path: impl AsRef<Path>) -> Self {
        if let ErrKind::Parse(pe) = &mut self.kind {
            pe.pos.path = Some(path.as_ref().into());
        }
        self
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.msg.is_empty() {
            write!(f, "{}", self.kind)
        } else if self.msg.ends_with(": ") {
            write!(f, "{}{}", self.msg, self.kind)
        } else if let Some(msg) = self.msg.strip_suffix(".") {
            write!(f, "{msg}: {}", self.kind)
        } else if self.msg.ends_with(":") {
            write!(f, "{} {}", self.msg, self.kind)
        } else {
            write!(f, "{}: {}", self.msg, self.kind)
        }
    }
}

impl<T: Into<ErrKind>> From<T> for Error {
    fn from(value: T) -> Self {
        Self::new(value, "")
    }
}
