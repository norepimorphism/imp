// mod ty;

use crate::{error::{self, Error}, op::{Expr, Operand, Symbol}, parser::Ast};
use std::collections::HashMap;

#[derive(Default)]
pub struct Interp {
    pub aliases: HashMap<Symbol, Operand>,
}

impl Interp {
    pub fn eval_ast(&mut self, ast: Ast) -> Result<(), Error> {
        for expr in ast.exprs {
            self.eval_expr(expr)?;
        }

        Ok(())
    }

    pub fn eval_expr(&mut self, expr: Expr) -> Result<(), Error> {
        let mut operands = expr.operands;
        self.subst_aliases(&mut operands);
        let operands = operands.into_iter();

        match expr.operation.name.as_str() {
            "let" => self.eval_let(operands),
            _ => {
                Err(Error::invalid(error::Class::Operation, todo!()))
            }
        }
    }

    fn subst_aliases(&self, operands: &mut [Operand]) {
        for operand in operands {
            if let Operand::Symbol(ref symbol) = operand {
                if let Some(value) = self.aliases.get(symbol) {
                    *operand = value.clone();
                }
            }
        }
    }

    fn eval_let(&mut self, mut operands: impl Iterator<Item = Operand>) -> Result<(), Error> {
        let alias = operands.next_or(error::Class::Operand)?;
        let Operand::Symbol(alias) = alias else {
            return Err(Error::expected(error::Class::Symbol));
        };

        let value = operands
            .next()
            .ok_or_else(|| Error::expected(error::Class::Operand))?;

        let _ = self.aliases.insert(alias, value);

        Ok(())
    }
}
