// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The IMPL lexer.
//!
//! The lexer translates IMPL code into lexical tokens. As this is the only component with access to
//! the original IMPL code, the lexer is responsible for preserving links---implemented as [`Span`]s
//! ---between tokens and the source code so that erroneous code snippets may be identified and
//! presented to the user.

mod tokenizer;

use crate::span::Span;
use std::{fmt, iter::Peekable};
use tokenizer::Tokenizer;

/// Translates one or more lines of IMPL code into [a sequence of lexical tokens](Output).
pub fn lex(impl_code: &str) -> Result<Output, Span<Error>> {
    let mut idxed_chars = make_peekable_idxed_chars(impl_code);
    let mut output = Output::default();

    // Iterate over each character/index pair.
    while let Some(first) = idxed_chars.next() {
        // Try to create a lexical token given the first character of the token's source code.
        let token = tokenize(&mut idxed_chars, first.val)
            .map(|tkzed| {
                // Success! `tkzed` contains our token (or not, if the tokenizer explicitly did not
                // produce one---this is the case with, e.g., comments).

                tkzed.maybe_token.map(|token| {
                    // Wrap the token in a span and append it to the final sequence.
                    Span::new(token, (first.idx)..(first.idx + tkzed.len))
                })
            })
            .map_err(|e| {
                // Tokenization failed; this only occurs when the first character is incompatible
                // with all tokenizers. The span is therefore the first character.
                Span::new(e, (first.idx)..(first.idx + 1))
            })?;

        if let Some(token) = token {
            output.tokens.push(token);
        }
    }

    if !output.tokens.is_empty() {
        enclose_tokens_in_parens(&mut output.tokens);
    }

    Ok(output)
}

fn make_peekable_idxed_chars(it: &str) -> Peekable<impl Iterator<Item = IndexedChar> + '_> {
    it.chars()
        .enumerate()
        .map(|(idx, val)| IndexedChar { idx, val })
        .peekable()
}

#[derive(Clone, Copy, Debug)]
struct IndexedChar {
    idx: usize,
    val: char,
}

fn tokenize(
    idxed_chars: &mut Peekable<impl Iterator<Item = IndexedChar>>,
    first_ch: char,
) -> Result<Tokenized, Error> {
    // There are two flavors of tokenizers: multi- and single-character. Multi-character tokenizers
    // form a token from multiple characters, whereas single-character tokenizers require only one
    // character.

    // The multi-character tokenizers will be attempted first with the single-character tokenizer
    // as a fallback.
    find_compat_multi_tokenizer(first_ch)
        .map(|tokenizer| {
            // A compatible multi-character tokenizer was found.
            tokenize_multi(idxed_chars, tokenizer, first_ch)
        })
        .or_else(|| {
            // None of the multi-character tokenizers are compatible; let's try a single-character
            // tokenizer.
            tokenize_single(first_ch).map(|token| Tokenized {
                len: 1,
                maybe_token: Some(token),
            })
        })
        .map(Ok)
        .unwrap_or_else(|| {
            // Neither a compatible multi- nor single-character tokenizer was found.
            Err(Error::Invalid(first_ch))
        })
}

fn find_compat_multi_tokenizer(ch: char) -> Option<&'static Tokenizer> {
    use tokenizer as tkz;

    // Try the tokenizers in this order.
    [
        tkz::SYMBOL,
        tkz::RATIONAL,
        tkz::STR_LIT,
        tkz::WHITESPACE,
        tkz::COMMENT,
    ]
    .iter()
    // Return the first tokenizer that accepts the character.
    .find(|tokenizer| (tokenizer.accepts)("", ch))
}

fn tokenize_multi(
    input: &mut Peekable<impl Iterator<Item = IndexedChar>>,
    tokenizer: &Tokenizer,
    first_ch: char,
) -> Tokenized {
    // The source code corresponding to this token.
    let mut impl_code = first_ch.to_string();

    // Continue processing characters with the selected tokenizer. Use [`Peekable::peek`] so that
    // rejected characters may be processed again through a different tokenizer.
    while let Some(IndexedChar { idx: _, val }) = input.peek().copied() {
        if (tokenizer.accepts)(impl_code.as_str(), val) {
            // The tokenizer accepts the next character; append it to the token code.
            impl_code.push(val);
            // Manually advance the iterator because [`Peekable::peek`] does not.
            let _ = input.next();
        } else {
            // The tokenizer rejected the character. `impl_code` is now complete.
            break;
        }
    }

    Tokenized {
        len: impl_code.len(),
        maybe_token: (tokenizer.tokenize)(impl_code),
    }
}

fn tokenize_single(ch: char) -> Option<Token> {
    match ch {
        '(' => Some(Token::LParen),
        ')' => Some(Token::RParen),
        '+' => Some(Token::Symbol("add".to_string())),
        '-' => Some(Token::Symbol("sub".to_string())),
        '*' => Some(Token::Symbol("add".to_string())),
        '/' => Some(Token::Symbol("div".to_string())),
        '^' => Some(Token::Caret),
        _ => None,
    }
}

/// The output of [`tokenize`].
struct Tokenized {
    /// The length, in characters, of the original IMPL code.
    len: usize,
    /// The token, if one should be included.
    maybe_token: Option<Token>,
}

/// A lexical token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    /// A rational number.
    Rational(String),
    /// A string literal.
    StrLit(String),
    /// A symbol.
    Symbol(String),
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
    /// A caret (`^`).
    Caret,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let it = match self {
            Self::Rational(it) => it.to_string(),
            Self::StrLit(it) => it.to_string(),
            Self::Symbol(it) => format!("{}", it),
            Self::LParen => '('.to_string(),
            Self::RParen => ')'.to_string(),
            Self::LBrace => '{'.to_string(),
            Self::RBrace => '}'.to_string(),
            Self::Dollar => '$'.to_string(),
            Self::Hash => '#'.to_string(),
            Self::Caret => '^'.to_string(),
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

/// Encloses a token sequence in [`Token::LParen`] and [`Token::RParen`].
///
/// Although the IMPL grammar permits omitting outer parentheses, the parser is designed to only
/// recognize expressions which are enclosed in parentheses. Here, we add these parentheses if they
/// were omitted.
fn enclose_tokens_in_parens(tokens: &mut Vec<Span<Token>>) {
    if !tokens_are_enclosed_in_parens(tokens) {
        // An outer left parenthesis wasn't found.

        // The length of this span is 0 so that it will never be shown in an error; this is
        // important because the user never actually typed this parenthesis, so such a span would
        // highlight the wrong character.
        tokens.insert(0, Span::new(Token::LParen, 0..0));

        let end = tokens
            .last()
            // This `expect` is OK because we know that `tokens` isn't empty; `tokens.first()`
            // matched with a `Some(_)` pattern, after all.
            .expect("`tokens` should not be empty")
            .range
            .end;
        tokens.push(Span::new(Token::RParen, end..end));
    }
}

fn tokens_are_enclosed_in_parens(tokens: &Vec<Span<Token>>) -> bool {
    // We assume that if the left parenthesis is present, the right is also.
    matches!(
        tokens.first(),
        Some(Span {
            inner: Token::LParen,
            range: _
        })
    )
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Invalid(char),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(ch) => {
                write!(f, "invalid character '{}'", ch)
            }
        }
    }
}
