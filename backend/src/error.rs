use crate::{
    lexer::Token,
    parser::{Expr, Operand, Operation, Ratio, StrLit, Symbol},
};
use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
    pub class: Class,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.class)
    }
}

#[derive(Debug)]
pub enum Kind {
    Expected,
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
    Char(Option<char>),
    Expr(Option<Expr>),
    Operand(Option<Operand>),
    Operation(Option<Operation>),
    Ratio(Option<Ratio>),
    StrLit(Option<StrLit>),
    Symbol(Option<Symbol>),
    Token(Option<Token>),
}

macro_rules! match_class {
    ($match:expr, $formatter:expr, $( ( $class:tt , $desc:literal ) $(,)? )*) => {
        match $match {
            $(
                Self::$class(it) => {
                    write!($formatter, $desc)?;
                    if let Some(it) = it {
                        write!($formatter, " {}", it)?;
                    }

                    Ok(())
                }
            )*
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match_class!(
            self,
            f,
            (Char, "character"),
            (Expr, "expression"),
            (Operand, "operand"),
            (Operation, "operation"),
            (Ratio, "rational number"),
            (StrLit, "string literal"),
            (Symbol, "symbol"),
            (Token, "token"),
        )
    }
}
