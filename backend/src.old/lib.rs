//! Interactive Mathematical Processor (IMP).
//!
//! IMP is a library for solving various mathematical problems notated in the IMP Language (IMPL).
//! This crate is the IMP backend, which provides a [`process`] function that fully evaluates a
//! string of IMPL code.
//!
//! # IMPL
//!
//! IMPL uses S-expressions for function application and LaTeX symbols for function and constant
//! names. Outer parentheses are optional, and comments are prefixed with a semicolon and terminated
//! by a line feed. It looks like this:
//!
//! ```latex
//! ; Gives the area of 'cos(x^2)' bounded by the X-axis.
//! \int (\cos (\pow x 2))\,dx
//! ```
//!
//! ## Formal Grammar
//!
//! ```abnf
//! program = *expression
//! expression = *"(" operation *operand *")"
//! operation = 1*ALPHA / operator
//! operator = "+" / "-" / "*" / "/"
//! operand = expression / number
//! number = 1*DIGIT
//! ```
//!
//! # Pipeline
//!
//! IMP follows a traditional interpreter model:
//! 1. The lexer tokenizes source code, stripping comments and whitespace in the process.
//! 2. The parser assigns meaning to these tokens by grouping them into expressions.
//! 3. These expressions are transformed across multiple intermediate passes.
//! 3. The interpreter evaluates each expression.

pub mod error;
pub mod interp;
pub mod lexer;
pub mod parser;
// pub mod pass;

pub use error::Error;
pub use interp::Interp;

use error::Span;
use lexer::Token;
use parser::Expr;

/// Processes IMPL code through an interpreter.
///
/// This function is recommended for most use cases of IMP; it abstracts away the interpretation
/// pipeline. Callbacks are available to inspect the processor state at different points in time.
pub fn process(
    interp: &mut Interp,
    input: &str,
    cb: Callbacks,
) -> Result<(), Span<Error>> {
    // First, IMPL code is broken into lexical tokens.
    let tokens = lexer::lex(input).map_err(|e| e.map(Error::Lexer))?;
    cb.post_lex.map(|f| f(tokens.as_slice()));

    // Second, those tokens are parsed into an Abstract Syntax Tree (AST).
    let mut ast = parser::parse(tokens.into_iter()).map_err(|e| e.map(Error::Parser))?;
    cb.post_parse.map(|f| f(&ast));

    // Third, the AST is desugared through multiple transformations called 'passes'.
    // let ast = ast.map(|mut it| {
    //     pass::resolve_pseudo_operations(&mut it);
    //     it
    // });
    cb.post_desugar.map(|f| f(&ast));

    // Fourth, and finally, the AST is evaluated by the interpreter.
    // interp.eval_expr(ast).map_err(|e| e.map(Error::Interp))

    Ok(())
}

/// Callbacks for [`process`].
pub struct Callbacks {
    /// Access the tokens after lexing.
    pub post_lex: Option<fn(&[Span<Token>])>,
    /// Access the AST after parsing.
    pub post_parse: Option<fn(&Expr)>,
    /// Access the AST after it is desugared.
    pub post_desugar: Option<fn(&Expr)>,
}
