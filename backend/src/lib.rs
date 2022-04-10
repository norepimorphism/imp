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

#![feature(let_else)]

pub mod lexer;
pub mod parser;
pub mod evaluator;
pub mod span;

use rayon::prelude::*;
use span::Span;
use std::fmt;

/// Callbacks for [`process`].
pub struct Callbacks {
    pub inspect_lexer_output: Option<fn(&lexer::Output)>,
    pub inspect_parser_output: Option<fn(&parser::Output)>,
}

pub fn process(impl_code: &str, cb: Callbacks) -> Result<Vec<evaluator::Output>, Span<Error>> {
    let output = lexer::lex(impl_code).map_err(|e| e.map(Error::Lexer))?;
    if let Some(cb) = cb.inspect_lexer_output {
        cb(&output);
    }

    let output = parser::parse(output).map_err(|e| e.map(Error::Parser))?;
    if let Some(cb) = cb.inspect_parser_output {
        cb(&output);
    }

    output
        .ast
        .into_par_iter()
        .map(|expr| evaluator::eval_ast(expr.inner).map_err(|e| e.map(Error::Evaluator)))
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Lexer(lexer::Error),
    Parser(parser::Error),
    Evaluator(evaluator::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lexer(e) => e.fmt(f),
            Self::Parser(e) => e.fmt(f),
            Self::Evaluator(e) => e.fmt(f),
        }
    }
}
