use crate::{
    parser::{Expr, Operand, OperationId},
};

pub fn resolve_pseudo_operations(expr: &mut Expr) {
    for operand in expr.operands.iter_mut() {
        if let Operand::Expr(expr) = operand {
            resolve_pseudo_operations(expr);
        }
    }

    if is_pseudo_operation(&expr.operation_id) {
        if let Some(first_operand) = expr.operands.first() {
            let ext = match first_operand {
                Operand::Rational(_) => "rational",
                _ => todo!(),
            };

            expr.operation_id.name.push('.');
            expr.operation_id.name.push_str(ext);
        }
    }
}

fn is_pseudo_operation(id: &OperationId) -> bool {
    matches!(id.name.as_str(), "add" | "sub" | "mul" | "div")
}
