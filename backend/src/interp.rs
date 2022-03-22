use crate::parser::{Ast, Expr};

pub struct Interp {

}

impl Interp {
    pub fn eval_ast(&mut self, ast: Ast) {
        for expr in ast.exprs {
            self.eval_expr(expr);
        }
    }

    pub fn eval_expr(&mut self, expr: Expr) {
        match expr.operation.name.as_str() {
            "add" => (),
            _ => (),
        }
    }
}
