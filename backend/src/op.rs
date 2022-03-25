use crate::error::Error;
use std::fmt;

pub struct Operation {
    pub operand_cnt: usize,
    pub execute: fn(operands: &[Operand]) -> Result<(), Error>,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct OperationId {
    /// The name.
    pub name: String,
}

impl fmt::Display for OperationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// An operand.
#[derive(Clone, Debug)]
pub enum Operand {
    /// An expression.
    Expr(Expr),
    /// A number.
    Number(Number),
    /// A string literal.
    StrLit(StrLit),
    /// A symbol.
    Symbol(Symbol),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expr(it) => it.fmt(f),
            Self::Number(it) => it.fmt(f),
            Self::StrLit(it) => it.fmt(f),
            Self::Symbol(it) => it.fmt(f),
        }
    }
}

/// An expression.
#[derive(Clone, Debug, Default)]
pub struct Expr {
    /// The operation ID.
    pub operation_id: OperationId,
    /// The operands.
    pub operands: Vec<Operand>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {})",
            self.operation_id,
            self.operands
                .iter()
                .map(|it| it.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[derive(Clone, Debug)]
pub enum Number {
    Rational(Rational),
    Irrational(Irrational),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rational(it) => it.fmt(f),
            Self::Irrational(it) => it.fmt(f),
        }
    }
}

/// A rational number.
#[derive(Clone, Debug)]
pub struct Rational {
    pub value: f64,
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone, Debug)]
pub enum Irrational {
    E,
    Pi,
}

impl fmt::Display for Irrational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::E => "e",
                Self::Pi => "pi",
            }
        )
    }
}

/// A string literal.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct StrLit {
    pub content: String,
}

impl fmt::Display for StrLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.content)
    }
}

/// A symbol.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Symbol {
    pub name: String,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
