// mod ty;

use crate::{
    error::{self, Error},
    op::{Expr, Operand, Operation, OperationId, Symbol},
    parser::Ast,
};
use std::collections::HashMap;

pub struct Interp {
    aliases: HashMap<Symbol, Operand>,
    operations: HashMap<OperationId, Operation>,
}

impl Default for Interp {
    fn default() -> Self {
        Self {
            aliases: HashMap::default(),
            operations: HashMap::from_iter([

            ]),
        }
    }
}

impl Interp {
    pub fn aliases(&self) -> impl Iterator<Item = (&Symbol, &Operand)> {
        self.aliases.iter()
    }

    pub fn eval_ast(&mut self, ast: Ast) -> Result<(), Error> {
        for expr in ast.exprs {
            self.eval_expr(expr)?;
        }

        Ok(())
    }

    pub fn eval_expr(&mut self, mut expr: Expr) -> Result<(), Error> {
        self.subst_aliases(&mut expr.operands);

        if let Some(operation) = self.operations.get(&expr.operation_id) {
            let operands = expr.operands
                .get(0..operation.operand_cnt)
                // TODO: Handle this error.
                .unwrap();

            (operation.execute)(operands)
        } else {
            // TODO: Replace range with actual span range.
            Err(Error::invalid(error::Class::OperationId, 0..1))
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
