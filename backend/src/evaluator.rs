// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The IMPL interpreter.

mod operand;
mod operation;

use crate::{parser::{self, Expr}, span::Span};
use operand::{Operand, RawOperand};
use operation::{OPERATIONS, Operation};
use std::fmt;

pub fn eval_ast(ast: Expr) -> Result<Output, Error> {
    Ok(Output::Text(eval_expr(ast).unwrap().to_string()))
}

fn eval_expr(expr: Expr) -> Result<Operand, Error> {
    let operation = get_operation_with_name(expr.operation.inner.name.as_str())?;
    let operands = construct_raw_operands(operation, expr.operands)?;
    let _ = check_operand_count(operation, operands.as_slice())?;

    execute_operation(operation, operands.as_slice())
}

fn get_operation_with_name(name: &str) -> Result<&Operation, Error> {
    OPERATIONS
        .get(name)
        .ok_or_else(|| Error::UnknownOperation {
            name: name.to_string(),
        })
}

fn construct_raw_operands(operation: &Operation, operands: Vec<Span<parser::Operand>>) -> Result<Vec<RawOperand>, Error> {
    operands
        .into_iter()
        // Recursively evaluate subexpressions (see [`Operand::from`]).
        .map(|operand| Operand::from(operand.inner))
        .enumerate()
        .map(|(idx, operand)| {
            if idx >= operation.sig.len() {
                return Err(Error::ExtraOperand);
            }

            let _ = type_check_operand(&operand, operation.sig[idx])?;

            Ok(operand.raw())
        })
        .collect()
}

fn type_check_operand(operand: &Operand, operand_kind: operand::Kind) -> Result<(), Error> {
    if kind_is_valid(operand, operand_kind) {
        Ok(())
    } else {
        return Err(Error::UnexpectedOperandKind);
    }
}

fn kind_is_valid(actual_operand: &Operand, expected_kind: operand::Kind) -> bool {
    actual_operand.kind() == expected_kind
}

fn check_operand_count(operation: &Operation, operands: &[RawOperand]) -> Result<(), Error> {
    if operands.len() < operation.sig.len() {
        Err(Error::MissingOperand)
    } else {
        Ok(())
    }
}

/// Execute the operation with its operands.
fn execute_operation(operation: &Operation, operands: &[RawOperand]) -> Result<Operand, Error> {
    (operation.exe)(operands).map_err(Error::Operation)
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
    UnexpectedOperandKind,
    UnknownOperation { name: String },
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
