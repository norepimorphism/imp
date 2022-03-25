//! A unified error interface.

use crate::lexer::Token;
use std::{fmt, ops::Range};

/// An error.
#[derive(Debug)]
pub struct Error {
    /// The kind.
    pub kind: Kind,
    /// The class causing the error.
    pub class: Class,
    pub range: Range<usize>,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.class)
    }
}

impl Error {
    pub fn expected(class: Class, range: Range<usize>) -> Self {
        Self { kind: Kind::Expected, class, range }
    }

    pub fn invalid(class: Class, range: Range<usize>) -> Self {
        Self { kind: Kind::Invalid, class, range }
    }
}

/// A kind of [error](Error).
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

#[derive(Clone, Debug)]
pub enum Class {
    /// A textual character.
    Char,
    /// An [expression](crate::parser::Expr).
    Expr,
    /// An [operand](crate::parser::Operand).
    Operand,
    /// An [operation ID](crate::op::Id).
    OperationId,
    /// A [string literal](crate::parser::StrLit).
    StrLit,
    /// A [symbol](crate::parser::Symbol).
    Symbol,
    /// A [lexical token](crate::lexer::Token).
    Token(Option<Token>),
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char => write!(f, "character"),
            Self::Expr => write!(f, "expression"),
            Self::Operand => write!(f, "operand"),
            Self::OperationId => write!(f, "operation"),
            Self::StrLit => write!(f, "string literal"),
            Self::Symbol => write!(f, "symbol"),
            Self::Token(maybe) => {
                write!(f, "token")?;
                if let Some(it) = maybe {
                    write!(f, " {}", it)?;
                }

                Ok(())
            }
        }
    }
}
