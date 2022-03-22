use crate::parser::{Ast, Expr};

pub fn interpret_ast(ast: Ast) {
    for expr in ast.exprs {
        interpret_expr(expr);
    }
}

pub fn interpret_expr(expr: Expr) {
    match expr.operation.name.as_str() {
        "add" => (),
        _ => (),
    }
}
