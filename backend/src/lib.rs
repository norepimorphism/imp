//!

#![feature(iter_advance_by, let_else)]

pub mod error;
pub mod interp;
pub mod lexer;
pub mod op;
pub mod parser;
pub mod pass;
pub mod saved;

pub use error::Error;
pub use interp::Interp;
pub use saved::Saved;

use parser::Ast;

pub fn process(input: &str) -> Result<Ast, Error> {
    let tokens = lexer::lex(input)?;
    // println!("{:?}", tokens);
    let ast = Ast::parse(tokens.into_iter())?;
    println!("{}", ast);

    Ok(ast)
}
