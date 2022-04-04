//! Interactive Mathematical Processor (IMP).
//!
//! IMP is a library for solving various mathematical problems notated in the IMP Language (IMPL).
//! This crate is the IMP backend, which provides a [`process`] function that evaluates one or more
//! lines of IMPL code.
//!
//! # IMPL
//!
//! IMPL uses S-expressions for function application and LaTeX symbols for function and constant
//! names. Outer parentheses are optional, and comments are prefixed with a semicolon and terminated
//! by a line feed.
//!
//! ## Formal Grammar
//!
//! The formal grammar of IMPL is notated here in Augmented Backus–Naur form:
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
//! Evaluating IMPL code is a five-stage pipeline; each stage recieves input from the previous stage
//! and produces input for the next stage. In chronological order:
//!
//! a. The lexer breaks source code into a sequence of lexical tokens, stripping comments and
//!    whitespace in the process.
//! b. The token sequence is desugared.
//! c. The parser assigns meaning to each token by grouping them into expressions and, ultimately,
//!    an Abstract Syntax Tree (AST).
//! d. The interpreter evaluates the AST and produces either a textual or graphical result.
//!
//! # Project Layout
//!
//! Each stage is implemented in a module identified by the stage's letter (refer to section
//! [Pipeline](#pipeline)). Each module contains some or all of:
//!
//! * a public function named `process`;
//! * a public type named `Output`; and
//! * a public type named `Error`.

#![feature(let_else)]

pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod span;

use rayon::prelude::*;
use span::Span;
use std::fmt;

/// Callbacks for [`process`].
pub struct Callbacks {
    pub a: Option<fn(&a::Output)>,
    pub b: Option<fn(&b::Output)>,
    pub c: Option<fn(&c::Output)>,
}

pub fn process(
    impl_code: &str,
    cb: Callbacks,
) -> Result<Vec<d::Output>, Span<Error>> {
    let output = a::process(impl_code).map_err(|e| e.map(Error::A))?;
    if let Some(a) = cb.a {
        a(&output);
    }

    if output.tokens.is_empty() {
        return Ok(Vec::new());
    }

    let output = b::process(output);
    if let Some(b) = cb.b {
        b(&output);
    }

    let output = c::process(output).map_err(|e| e.map(Error::C))?;
    if let Some(c) = cb.c {
        c(&output);
    }

    Ok(output.ast.into_par_iter().map(|expr| d::process(expr.inner).unwrap()).collect())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    A(a::Error),
    C(c::Error),
    D(d::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(e) => e.fmt(f),
            Self::C(e) => e.fmt(f),
            Self::D(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Span<Error> {}

impl fmt::Display for Span<Error> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
