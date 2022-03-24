//!

#![feature(iter_advance_by, let_else)]

pub mod error;
// pub mod interp;
pub mod lexer;
pub mod op;
pub mod parser;
pub mod pass;
pub mod span;

pub use error::Error;
// ub use interp::Interp;
pub use span::Span;

use parser::Ast;

pub fn process(input: &str) -> Result<Ast, Error> {
    let tokens = lexer::lex(input)?;
    // println!("{:?}", tokens);
    let ast = Ast::parse(tokens.into_iter())?;
    println!("{}", ast);

    Ok(ast)
}
