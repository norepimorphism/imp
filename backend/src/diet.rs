// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{lexer, span::Span};
use std::fmt;

pub fn process(mut input: lexer::Output) -> Output {
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
fn enclose_in_parens(tokens: &mut Vec<Span<lexer::Token>>) {
    if !matches!(
        tokens.first(),
        Some(Span {
            inner: lexer::Token::LParen,
            range: _
        })
    ) {
        // An outer left parenthesis wasn't found.

        // The length of this span is 0 so that it will never be shown in an error; this is
        // important because the user never actually typed this parenthesis, so such a span would
        // highlight the wrong character.
        tokens.insert(0, Span::new(lexer::Token::LParen, 0..0));

        let end = tokens
            .last()
            // This `expect` is OK because we know that `tokens` isn't empty; `tokens.first()`
            // matched with a `Some(_)` pattern, after all.
            .expect("`tokens` should not be empty")
            .range
            .end;
        tokens.push(Span::new(lexer::Token::RParen, end..end));
    }
}

/// Desugars operators into operations.
fn resolve_operators(tokens: Vec<Span<lexer::Token>>) -> Vec<Span<Token>> {
    tokens
        .into_iter()
        .map(|token| {
            token.map(|token| match token {
                lexer::Token::LParen => Token::LParen,
                lexer::Token::RParen => Token::RParen,
                lexer::Token::LBrace => Token::LBrace,
                lexer::Token::RBrace => Token::RBrace,
                lexer::Token::Plus => Token::Symbol("add".to_string()),
                lexer::Token::Minus => Token::Symbol("sub".to_string()),
                lexer::Token::Star => Token::Symbol("mul".to_string()),
                lexer::Token::Slash => Token::Symbol("div".to_string()),
                lexer::Token::Dollar => Token::Dollar,
                lexer::Token::Hash => Token::Hash,
                lexer::Token::Rational(it) => Token::Rational(it),
                lexer::Token::StrLit(it) => Token::StrLit(it),
                lexer::Token::Symbol(it) => Token::Symbol(it),
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
