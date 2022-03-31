use crate::b::Token;
use std::fmt;

/// A parser error.
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Subject {
    /// An [expression](crate::parser::Expr).
    Expr,
    /// An [operand](crate::parser::Operand).
    Operand,
    Operation,
    /// A [symbol](crate::parser::Symbol).
    Symbol,
    /// A [lexical token](crate::lexer::Token).
    Token(Option<Token>),
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expr => write!(f, "expression"),
            Self::Operand => write!(f, "operand"),
            Self::Operation => write!(f, "operation"),
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
