mod arith;
mod calc;
mod trig;

use super::operand::{self, Operand};
use ahash::RandomState;
use std::collections::HashMap;

pub struct Operation {
    pub sig: &'static [operand::Kind],
    pub exe: fn(&[operand::Raw]) -> Result<Operand, Error>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {}

#[static_init::dynamic]
pub static OPERATIONS: HashMap<&'static str, Operation, RandomState> = HashMap::from_iter([
    // ("simplify", todo!()),
    // ("solve", todo!()),
    ("add", arith::ADD),
    ("sub", arith::SUB),
    ("mul", arith::MUL),
    ("div", arith::DIV),
    // ("sum", arith::SUM),
    // ("prod", arith::PROD),
    // ("int", calc::INT),
    ("sin", trig::SIN),
    ("cos", trig::COS),
    ("tan", trig::TAN),
    ("arcsin", trig::ARCSIN),
    ("arccos", trig::ARCCOS),
    ("arctan", trig::ARCTAN),
    // ("deg", trig::DEG),
    // ("rad", trig::RAD),
]);
