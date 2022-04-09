// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
//! Each stage is implemented in a module, each of which contains some or all of:
//!
//! * a public function named `process`;
//! * a public type named `Output`; and
//! * a public type named `Error`.

#![feature(let_else)]

pub mod lexer;
pub mod diet;
pub mod parser;
pub mod interp;
pub mod span;

use rayon::prelude::*;
use span::Span;
use std::fmt;

/// Callbacks for [`process`].
pub struct Callbacks {
    pub lexer: Option<fn(&lexer::Output)>,
    pub diet: Option<fn(&diet::Output)>,
    pub parser: Option<fn(&parser::Output)>,
}

pub fn process(impl_code: &str, cb: Callbacks) -> Result<Vec<interp::Output>, Span<Error>> {
    let output = lexer::process(impl_code).map_err(|e| e.map(Error::Lexer))?;
    if let Some(cb) = cb.lexer {
        cb(&output);
    }

    if output.tokens.is_empty() {
        return Ok(Vec::new());
    }

    let output = diet::process(output);
    if let Some(cb) = cb.diet {
        cb(&output);
    }

    let output = parser::process(output).map_err(|e| e.map(Error::Parser))?;
    if let Some(cb) = cb.parser {
        cb(&output);
    }

    output
        .ast
        .into_par_iter()
        .map(|expr| interp::process(expr.inner).unwrap())
        .map(Ok)
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Lexer(lexer::Error),
    Parser(parser::Error),
    Interp(interp::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lexer(e) => e.fmt(f),
            Self::Parser(e) => e.fmt(f),
            Self::Interp(e) => e.fmt(f),
        }
    }
}
