//!

#![feature(exact_size_is_empty, iter_advance_by, let_else)]

pub mod error;
pub mod interp;
pub mod lexer;
pub mod op;
pub mod parser;
pub mod pass;
pub mod span;

pub use error::Error;
pub use interp::Interp;
pub use span::Span;

use parser::Ast;

pub fn process(input: &str) -> Result<Ast, Error> {
    let tokens = lexer::lex(input)?;
    // println!("{}", tokens.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("\n"));
    let ast = Ast::parse(tokens.into_iter())?;
    //println!("{}", ast);

    Ok(ast)
}
