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

pub fn eval_ast(ast: Expr) -> Result<Output, Span<Error>> {
    eval_expr(ast).map(|it| Output::Text(it.to_string()))
}

fn eval_expr(expr: Expr) -> Result<Operand, Span<Error>> {
    let operation = get_operation(expr.operation)?;
    let operands = construct_raw_operands(&operation.inner, expr.operands)?;
    let _ = check_operand_count(&operation, operands.as_slice())?;

    execute_operation(&operation, operands.as_slice())
}

fn get_operation(operation: Span<parser::Operation>) -> Result<Span<&'static Operation>, Span<Error>> {
    OPERATIONS
        .get(operation.inner.name.as_str())
        .map(|it| operation.clone().map(|_| it))
        .ok_or_else(|| operation.map(|operation| Error::UnknownOperation {
            name: operation.name.to_string(),
        }))
}

fn construct_raw_operands(
    operation: &Operation,
    operands: Vec<Span<parser::Operand>>,
) -> Result<Vec<RawOperand>, Span<Error>> {
    operands
        .into_iter()
        // Recursively evaluate subexpressions (see [`Operand::from`]).
        .map(|operand| operand.map(|inner| Operand::from(inner)))
        .enumerate()
        .map(|(idx, operand)| {
            if idx >= operation.sig.len() {
                return Err(operand.map(|_| Error::ExtraOperand));
            }

            let _ = type_check_operand(&operand, operation.sig[idx])?;

            Ok(operand.inner.raw())
        })
        .collect()
}

fn type_check_operand(operand: &Span<Operand>, operand_kind: operand::Kind) -> Result<(), Span<Error>> {
    if kind_is_valid(&operand.inner, operand_kind) {
        Ok(())
    } else {
        Err(Span::new(Error::UnexpectedOperandKind, operand.range.clone()))
    }
}

fn kind_is_valid(actual_operand: &Operand, expected_kind: operand::Kind) -> bool {
    actual_operand.kind() == expected_kind
}

fn check_operand_count(operation: &Span<&Operation>, operands: &[RawOperand]) -> Result<(), Span<Error>> {
    if operands.len() < operation.inner.sig.len() {
        Err(Span::new(Error::MissingOperand, operation.range.clone()))
    } else {
        Ok(())
    }
}

/// Execute the operation with its operands.
fn execute_operation(operation: &Span<&Operation>, operands: &[RawOperand]) -> Result<Operand, Span<Error>> {
    (operation.inner.exe)(operands)
        .map_err(|e| Span::new(Error::Operation(e), operation.range.clone()))
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExtraOperand => {
                write!(f, "extra operand")
            }
            Self::MissingOperand => {
                write!(f, "missing operand")
            }
            Self::Operation(e) => {
                e.fmt(f)
            }
            Self::UnexpectedOperandKind => {
                write!(f, "unexpected operand kind")
            }
            Self::UnknownOperation { name } => {
                write!(f, "unknown operation \"{}\"", name)
            }
        }
    }
}
