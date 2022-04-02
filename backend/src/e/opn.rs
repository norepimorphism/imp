mod arith;
mod calc;
mod trig;

macro_rules! expect_operand {
    ($pattern:pat, $operands:expr, $idx:expr) => {
        let $pattern = &$operands[$idx] else { return Err(Error::Expected); };
    };
}

use ahash::{RandomState};
use expect_operand;
use crate::c::{Rational, StrLit, Symbol};
use std::{collections::HashMap, fmt};

pub struct Operation {
    pub exe: fn(&[Operand]) -> Result<Operand, Error>,
}

pub enum Operand {
    Rational(Rational),
    StrLit(StrLit),
    Symbol(Symbol),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rational(it) => write!(f, "{}", it.val),
            Self::StrLit(it) => write!(f, "{}", it.content),
            Self::Symbol(it) => write!(f, "\\{}", it.name),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Expected,
}

#[static_init::dynamic]
pub static OPERATIONS: HashMap<String, Operation, RandomState> = HashMap::from_iter([
    ("add".to_string(), arith::ADD),
    ("sub".to_string(), arith::SUB),
    ("mul".to_string(), arith::MUL),
    ("div".to_string(), arith::DIV),
    // ("sum", arith::SUM),
    // ("prod", arith::PROD),
    // ("int", calc::INT),
    // ("sin", trig::SIN),
    // ("cos", trig::COS),
    // ("tan", trig::TAN),
    // ("arcsin", trig::ARCSIN),
    // ("arccos", trig::ARCCOS),
    // ("arctan", trig::ARCTAN),
    // ("deg", trig::DEG),
    // ("rad", trig::RAD),
]);
