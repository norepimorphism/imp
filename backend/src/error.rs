//! A unified error interface.

use std::fmt;

/// An error.
#[derive(Debug)]
pub struct Error {
    /// The kind of error.
    pub kind: Kind,
    /// The class causing the error.
    pub class: Class,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.class)
    }
}

impl Error {
    pub fn expected(class: Class) -> Self {
        Self { kind: Kind::Expected, class }
    }

    pub fn invalid(class: Class) -> Self {
        Self { kind: Kind::Invalid, class }
    }
}

/// A kind of error.
#[derive(Debug)]
pub enum Kind {
    /// Something expected is missing.
    Expected,
    /// Something is not valid.
    Invalid,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expected => {
                write!(f, "expected")
            }
            Self::Invalid => {
                write!(f, "invalid")
            }
        }
    }
}

#[derive(Debug)]
pub enum Class {
    /// A textual character.
    Char,
    /// An [expression](crate::parser::Expr).
    Expr,
    /// An [operand](crate::parser::Operand).
    Operand,
    /// An [operation](crate::parser::Operation).
    Operation,
    /// A [string literal](crate::parser::StrLit).
    StrLit,
    /// A [symbol](crate::parser::Symbol).
    Symbol,
    /// A lexical [token](crate::lexer::Token).
    Token,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Char => "character",
                Self::Expr => "expression",
                Self::Operand => "operand",
                Self::Operation => "operation",
                Self::StrLit => "string literal",
                Self::Symbol => "symbol",
                Self::Token =>"token",
            }
        )

    }
}
