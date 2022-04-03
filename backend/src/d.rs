//! The IMPL interpreter.

mod operand;
mod operation;

use crate::c::Expr;
use operand::Operand;
use operation::OPERATIONS;
use std::fmt;

pub fn process(expr: Expr) -> Result<Output, Error> {
    Ok(Output::Text(eval_expr(expr).unwrap().to_string()))
}

fn eval_expr(expr: Expr) -> Result<Operand, Error> {
    let operation = OPERATIONS
        .get(&expr.operation.inner.name.as_str())
        .ok_or_else(|| Error::UnknownOperation { name: expr.operation.inner.name.clone() })?;

    let operands = expr.operands
        .into_iter()
        // Recursively evaluate subexpressions (see [`Operand::from`]).
        .map(|operand| operand.map(Operand::from))
        .enumerate()
        .map(|(idx, operand)| {
            if idx >= operation.sig.len() {
                return Err(Error::ExtraOperand);
            }

            let expected_type = operation.sig[idx];

            if !operand.inner.is_type_valid(expected_type) {
                return Err(Error::UnexpectedOperandType);
            }

            Ok(operand::Raw::new(operand.inner))
        })
        .collect::<Result<Vec<operand::Raw>, Error>>()?;

    if operands.len() < operation.sig.len() {
        return Err(Error::MissingOperand);
    }

    // Execute the operation with its operands.
    (operation.exe)(operands.as_slice()).map_err(Error::Operation)
}

pub enum Output {
    Text(String),
    Graphic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    ExtraOperand,
    MissingOperand,
    Operation(operation::Error),
    UnexpectedOperandType,
    UnknownOperation { name: String },
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
