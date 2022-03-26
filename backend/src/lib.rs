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

pub fn process(interp: &mut Interp, input: &str) -> Result<parser::Operand, Error> {
    let tokens = lexer::lex(input)?;
    let mut ast = parser::parse(tokens.into_iter())?;
    pass::prep(&mut ast);

    interp.eval_expr(ast)
}
