//!

#![feature(exact_size_is_empty, iter_advance_by, let_else)]

pub mod error;
pub mod interp;
pub mod lexer;
pub mod parser;
pub mod pass;
pub mod span;

pub use error::Error;
pub use interp::Interp;
pub use span::Span;

use parser::Ast;

pub fn process(input: &str) -> Result<Ast, Error> {
    let tokens = lexer::lex(input)?;
    let mut ast = Ast::parse(tokens.into_iter())?;
    do_passes(&mut ast);

    Ok(ast)
}

fn do_passes(ast: &mut Ast) {
    for expr in ast.exprs.iter_mut() {
        pass::resolve_pseudo_operations(expr);
    }
}
