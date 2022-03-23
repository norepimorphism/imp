#![feature(iter_advance_by, let_else)]

pub mod error;
pub mod interp;
pub mod lexer;
pub mod parser;

use error::Error;
use parser::Ast;

// mod stream;

pub fn process(input: &str) -> Result<Ast, Error> {
    let tokens = lexer::lex(input)?;
    // println!("{:?}", tokens);
    let ast = parser::parse(tokens.into_iter())?;
    println!("{}", ast);

    Ok(ast)
}
