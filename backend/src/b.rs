// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{a, span::Span};
use std::fmt;

pub fn process(mut input: a::Output) -> Output {
    enclose_in_parens(&mut input.tokens);

    Output {
        tokens: resolve_operators(input.tokens),
    }
}

/// Encloses a token sequence in [`Token::LParen`] and [`Token::RParen`].
///
/// Although the IMPL grammar permits omitting outer parentheses, the parser is designed to only
/// recognize expressions which are enclosed in parentheses. Here, we add these parentheses if they
/// were omitted.
fn enclose_in_parens(tokens: &mut Vec<Span<a::Token>>) {
    if !matches!(
        tokens.first(),
        Some(Span {
            inner: a::Token::LParen,
            range: _
        })
    ) {
        // An outer left parenthesis wasn't found.

        // The length of this span is 0 so that it will never be shown in an error; this is
        // important because the user never actually typed this parenthesis, so such a span would
        // highlight the wrong character.
        tokens.insert(0, Span::new(a::Token::LParen, 0..0));

        let end = tokens
            .last()
            // This `expect` is OK because we know that `tokens` isn't empty; `tokens.first()`
            // matched with a `Some(_)` pattern, after all.
            .expect("`tokens` should not be empty")
            .range
            .end;
        tokens.push(Span::new(a::Token::RParen, end..end));
    }
}

/// Desugars operators into operations.
fn resolve_operators(tokens: Vec<Span<a::Token>>) -> Vec<Span<Token>> {
    tokens
        .into_iter()
        .map(|token| {
            token.map(|token| match token {
                a::Token::LParen => Token::LParen,
                a::Token::RParen => Token::RParen,
                a::Token::LBrace => Token::LBrace,
                a::Token::RBrace => Token::RBrace,
                a::Token::Plus => Token::Symbol("add".to_string()),
                a::Token::Minus => Token::Symbol("sub".to_string()),
                a::Token::Star => Token::Symbol("mul".to_string()),
                a::Token::Slash => Token::Symbol("div".to_string()),
                a::Token::Dollar => Token::Dollar,
                a::Token::Hash => Token::Hash,
                a::Token::Rational(it) => Token::Rational(it),
                a::Token::StrLit(it) => Token::StrLit(it),
                a::Token::Symbol(it) => Token::Symbol(it),
            })
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    /// A left, or opening, parenthesis (`(`).
    LParen,
    /// A right, or closing, parenthesis (`)`).
    RParen,
    /// A left, or opening, brace (`{`).
    LBrace,
    /// A right, or closing, brace (`}`).
    RBrace,
    /// A dollar sign (`$`).
    Dollar,
    /// A hash sign (`#`).
    Hash,
    /// A rational number.
    Rational(String),
    /// A string literal.
    StrLit(String),
    /// A symbol.
    Symbol(String),
}

// TODO: This was copy-and-pasted from Module A.
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let it = match self {
            Self::LParen => '('.to_string(),
            Self::RParen => ')'.to_string(),
            Self::LBrace => '{'.to_string(),
            Self::RBrace => '}'.to_string(),
            Self::Dollar => '$'.to_string(),
            Self::Hash => '#'.to_string(),
            Self::Rational(it) => it.to_string(),
            Self::StrLit(it) => it.to_string(),
            Self::Symbol(it) => format!("\\{}", it),
        };

        write!(
            f,
            "{0}{1}{0}",
            match it.len() {
                1 => '\'',
                _ => '"',
            },
            it
        )
    }
}

#[derive(Debug, Default)]
pub struct Output {
    pub tokens: Vec<Span<Token>>,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.tokens
                .iter()
                .map(|token| token.inner.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
