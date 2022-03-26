mod operation;

use crate::{
    error::{self, Error},
    parser::{Expr, Operand, OperationId, Symbol},
    parser::Ast,
};
use std::collections::HashMap;

pub struct Interp {
    aliases: HashMap<Symbol, Operand>,
    operations: HashMap<OperationId, Operation>,
}

impl Default for Interp {
    fn default() -> Self {
        let operations = [
            ("add.rational", operation::ADD_RATIONAL),
        ]
        .map(|(name, operation)| (
            OperationId { name: name.to_string() },
            operation,
        ));

        Self {
            aliases: HashMap::default(),
            operations: HashMap::from_iter(operations),
        }
    }
}

impl Interp {
    pub fn aliases(&self) -> impl Iterator<Item = (&Symbol, &Operand)> {
        self.aliases.iter()
    }

    pub fn eval_ast(&mut self, ast: Ast) -> Result<Vec<Operand>, Error> {
        ast.exprs
            .into_iter()
            .map(|expr| self.eval_expr(expr))
            .collect()
    }

    pub fn eval_expr(&mut self, mut expr: Expr) -> Result<Operand, Error> {
        self.subst_aliases(&mut expr.operands);

        if let Some(operation) = self.operations.get(&expr.operation_id) {
            Self::eval_operation_with_operands(operation, expr.operands)
        } else {
            // TODO: Replace range with actual span range.
            Err(Error::invalid(error::Class::OperationId, 0..1))
        }
    }

    fn eval_operation_with_operands(operation: &Operation, mut operands: Vec<Operand>) -> Result<Operand, Error> {
        let expected_operand_cnt = operation.operand_cnt;
        let actual_operand_cnt = operands.len();

        if actual_operand_cnt < expected_operand_cnt {
            return Err(todo!());
        }

        // TODO: This isn't sound.
        let execute_cnt = 1 + (actual_operand_cnt - expected_operand_cnt);

        let mut last_result = None;

        for i in 0..execute_cnt {
            if let Some(last_result) = last_result {
                operands[i] = last_result;
            }

            let operands = &operands[i..(i + expected_operand_cnt)];
            last_result = Some((operation.execute)(operands));
        }

        Ok(last_result.unwrap())
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

    // fn eval_let(&mut self, mut operands: impl Iterator<Item = Operand>) -> Result<(), Error> {
    //     let alias = operands.next_or(error::Class::Operand)?;
    //     let Operand::Symbol(alias) = alias else {
    //         return Err(Error::expected(error::Class::Symbol));
    //     };

    //     let value = operands
    //         .next()
    //         .ok_or_else(|| Error::expected(error::Class::Operand))?;

    //     let _ = self.aliases.insert(alias, value);

    //     Ok(())
    // }
}

struct Operation {
    operand_cnt: usize,
    execute: fn(&[Operand]) -> Operand,
}
