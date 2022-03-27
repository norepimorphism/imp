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

use lexer::Token;
use parser::Expr;

pub fn process(interp: &mut Interp, input: &str, cb: Callbacks) -> Result<parser::Operand, Error> {
    let tokens = lexer::lex(input)?;
    (cb.post_lex)(tokens.as_slice());

    let mut ast = parser::parse(tokens.into_iter())?;
    (cb.post_parse)(&ast);

    pass::resolve_pseudo_operations(&mut ast);
    (cb.post_resolve)(&ast);

    interp.eval_expr(ast)
}

pub struct Callbacks {
    pub post_lex: fn(&[Span<Token>]),
    pub post_parse: fn(&Expr),
    pub post_resolve: fn(&Expr),
}
