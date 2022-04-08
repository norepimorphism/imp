// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_use]
mod macros;

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
    ("pi", trig::PI),
    ("sin", trig::SIN),
    ("cos", trig::COS),
    ("tan", trig::TAN),
    // ("arcsin", trig::ARCSIN),
    // ("arccos", trig::ARCCOS),
    // ("arctan", trig::ARCTAN),
    // ("deg", trig::DEG),
    // ("rad", trig::RAD),
]);

mod arith {
    binary_rr_r!(ADD, +);
    binary_rr_r!(SUB, -);
    binary_rr_r!(MUL, *);
    binary_rr_r!(DIV, /);
}

mod calc {

}

mod trig {
    use rust_decimal::{Decimal, MathematicalOps as _};

    nullary_r!(PI, Decimal::PI);
    unary_r_r!(SIN, sin);
    unary_r_r!(COS, cos);
    unary_r_r!(TAN, tan);
}
