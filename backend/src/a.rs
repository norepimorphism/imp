//! The IMPL lexer.
//!
//! The lexer translates IMPL code into lexical tokens. As this is the only component with access to
//! the original IMPL code, the lexer is responsible for preserving links between tokens and
//! the source code so that erroneous code snippets may be identified and presented to the user.
//! Specifically, these links are implemented as [`Span`]s.

mod tkz;

use crate::span::Span;
use std::{fmt, iter::Peekable};
use tkz::Tokenizer;

/// Translates one or more lines of IMPL code into [a sequence of lexical tokens](Output).
pub fn process(impl_code: &str) -> Result<Output, Span<Error>> {
    // Create a peekable iterator of character and index pairs.
    let mut idxed_chars = break_into_chars(impl_code);
    let mut output = Output::default();

    // Iterate over each character/index pair.
    while let Some(first) = idxed_chars.next() {
        // Try to create a lexical token given the first character of the token's source code.
        let token = tokenize(&mut idxed_chars, first.val)
            .map(|tkzed| {
                // The token was successfully... well, tokenized.

                tkzed.maybe_token.map(|token| {
                    // Include this token in the final sequence.
                    Span::new(token, (first.idx)..(first.idx + tkzed.len))
                })
            })
            .map_err(|e| {
                // Tokenization failed; this only occurs when the first character is
                // incompatible with all tokenizers. The error range is therefore the first
                // character.
                Span::new(e, (first.idx)..(first.idx + 1))
            })?;

        if let Some(token) = token {
            output.tokens.push(token);
        }
    }

    Ok(output)
}

fn break_into_chars(it: &str) -> Peekable<impl Iterator<Item = IndexedChar> + '_> {
    it
        .chars()
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
    // form a token from multiple characters, while single-character tokenizers require only one
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
            Err(Error::TokenizerNotFound { first_ch })
        })
}

fn find_compat_multi_tokenizer(ch: char) -> Option<&'static Tokenizer> {
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
        '{' => Some(Token::LBrace),
        '}' => Some(Token::RBrace),
        '+' => Some(Token::Plus),
        '-' => Some(Token::Minus),
        '*' => Some(Token::Star),
        '/' => Some(Token::Slash),
        '$' => Some(Token::Dollar),
        '#' => Some(Token::Hash),
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
    /// A left, or opening, parenthesis (`(`).
    LParen,
    /// A right, or closing, parenthesis (`)`).
    RParen,
    /// A left, or opening, brace (`{`).
    LBrace,
    /// A right, or closing, brace (`}`).
    RBrace,
    /// A plus sign (`+`).
    Plus,
    /// A minus sign (`-`).
    Minus,
    /// An asterisk (`*`).
    Star,
    /// A forward slash (`/`).
    Slash,
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let it = match self {
            Self::LParen => '('.to_string(),
            Self::RParen => ')'.to_string(),
            Self::LBrace => '{'.to_string(),
            Self::RBrace => '}'.to_string(),
            Self::Plus => '+'.to_string(),
            Self::Minus => '-'.to_string(),
            Self::Star => '*'.to_string(),
            Self::Slash => '/'.to_string(),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    TokenizerNotFound { first_ch: char }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
