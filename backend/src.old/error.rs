use std::{fmt, ops::Range};

#[derive(Debug)]
pub enum Error {
    Lexer(crate::lexer::Error),
    Parser(crate::parser::Error),
    // Interp(crate::interp::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lexer(e) => e.fmt(f),
            Self::Parser(e) => e.fmt(f),
            // Self::Interp(e) => e.fmt(f),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Span<T> {
    pub inner: T,
    pub range: Range<usize>,
}

impl<T> Span<T> {
    pub fn new(inner: T, range: Range<usize>) -> Self {
        Self { inner, range }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Span<U> {
        Span {
            inner: f(self.inner),
            range: self.range,
        }
    }
}

impl<T, E> Span<Result<T, E>> {
    pub fn transpose(self) -> Result<Self, Self> {
        self.inner
            .map(|t| Self::new(Ok(t), self.range))
            .map_err(|e| Self::new(Err(e), self.range))
    }
}
