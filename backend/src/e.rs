//! The IMPL interpreter.

mod opn;

use crate::c::{self, Expr};
use opn::{OPERATIONS, Operand, Operation};
use std::{collections::HashMap, fmt};

pub fn process(interp: &mut Interp, expr: Expr) -> Result<Output, Error> {
    Ok(Output::Text(interp.eval_expr(expr).unwrap().to_string()))
}

#[derive(Default)]
pub struct Interp {
    aliases: HashMap<String, Operand>,
}

impl Interp {
    fn eval_expr(&self, expr: Expr) -> Result<Operand, opn::Error> {
        let operation = OPERATIONS.get(&expr.operation.inner.name.as_str()).unwrap();
        let operands = expr.operands
            .into_iter()
            .map(|operand| {
                match operand.inner {
                    c::Operand::Expr(expr) => self.eval_expr(expr).unwrap(),
                    c::Operand::Rational(it) => Operand::Rational(it),
                    c::Operand::StrLit(it) => Operand::StrLit(it),
                    c::Operand::Symbol(it) => Operand::Symbol(it),
                }
            })
            .collect::<Vec<Operand>>();

        (operation.exe)(operands.as_slice())
    }
}

pub enum Output {
    Text(String),
    Graphic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
