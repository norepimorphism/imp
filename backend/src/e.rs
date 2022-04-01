//! The IMPL interpreter.

use crate::c::{Expr, Operand};
use std::{collections::HashMap, fmt};

pub fn process(interp: &mut Interp, expr: Expr) -> Result<Output, Error> {
    interp.eval_expr(expr)
}

#[derive(Default)]
pub struct Interp {
    aliases: HashMap<String, Operand>,
}

impl Interp {
    fn eval_expr(&self, expr: Expr) -> Result<Output, Error> {
        todo!()
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
