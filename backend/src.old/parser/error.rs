//! A unified error interface.

use crate::lexer::Token;
use std::fmt;

/// A parser error.
#[derive(Debug)]
pub struct Error {
    /// The cause of the error.
    cause: Cause,
    subject: Subject,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.cause, self.subject)
    }
}

impl Error {
    pub fn expected(subject: Subject) -> Self {
        Self {
            cause: Cause::Expected,
            subject,
        }
    }

    pub fn invalid(subject: Subject) -> Self {
        Self {
            cause: Cause::Invalid,
            subject,
        }
    }

    pub fn cause(&self) -> Cause {
        self.cause
    }

    pub fn subject(&self) -> &Subject {
        &self.subject
    }
}

/// The cause of an [error](Error).
#[derive(Clone, Copy, Debug)]
pub enum Cause {
    /// Something expected is missing.
    Expected,
    /// Something is not valid.
    Invalid,
}

impl fmt::Display for Cause {
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
pub enum Subject {
    /// A textual character.
    Char,
    /// An [expression](crate::parser::Expr).
    Expr,
    /// An [operand](crate::parser::Operand).
    Operand,
    /// An [operation ID](crate::parser::OperationId).
    OperationId,
    /// A [rational number](crate::parser::Rational)
    Rational,
    /// A [string literal](crate::parser::StrLit).
    StrLit,
    /// A [symbol](crate::parser::Symbol).
    Symbol,
    /// A [lexical token](crate::lexer::Token).
    Token(Option<Token>),
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char => write!(f, "character"),
            Self::Expr => write!(f, "expression"),
            Self::Operand => write!(f, "operand"),
            Self::OperationId => write!(f, "operation"),
            Self::Rational => write!(f, "rational number"),
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
