//! The IMPL lexer.
//!
//! The lexer translates IMPL code into lexical tokens.
//!
//! This is the only component which directly interacts with IMPL source code, and so the lexer is
//! responsible for preserving 'links' between tokens and the original code so that erroneous code
//! snippets may be identified and presented to the user; such links are represented by 'spans'.

use crate::error::Span;
use std::{fmt, iter::Peekable};

/// Translates IMPL code into [lexical tokens](Token).
pub fn lex(input: &str) -> Result<Vec<Span<Token>>, Span<Error>> {
    let mut input = input
        // Break the input string into individual characters.
        .chars()
        .enumerate()
        .map(|(idx, ch)| Tokenizable { idx, ch })
        // Enable LALR(1) lookahead.
        .peekable();

    let mut tokens = Vec::new();

    // Process the input string, one character at a time.
    while let Some(first) = input.next() {
        let output = tokenize(&mut input, first.ch).map_err(|e| {
            // [`tokenize`] only fails when the first character is incompatible with all
            // tokenizers. Thus, the error range must be the first character.
            Span::new(e, (first.idx)..(first.idx + 1))
        })?;

        if let Some(token) = output.token {
            tokens.push(Span::new(token, (first.idx)..(first.idx + output.len)));
        }
    }

    Ok(tokens)
}

#[derive(Clone, Copy)]
struct Tokenizable {
    idx: usize,
    ch: char,
}

/// Translates IMPL code into a token based on the first character.
fn tokenize(
    input: &mut Peekable<impl Iterator<Item = Tokenizable>>,
    first_ch: char,
) -> Result<Tokenized, Error> {
    // Search for a compatible multi-character tokenizer in this order.
    [
        SYMBOL_TOKENIZER,
        RATIONAL_TOKENIZER,
        STR_LIT_TOKENIZER,
        WHITESPACE_TOKENIZER,
        COMMENT_TOKENIZER,
    ]
    .iter()
    // Try the first tokenizer that accepts the character.
    .find(|tokenizer| (tokenizer.accepts)("", first_ch))
    .map(|tokenizer| {
        // A compatible multi-character tokenizer was found.
        tokenize_multi(input, tokenizer, first_ch)
    })
    .or_else(|| {
        // None of the multi-character tokenizers are compatible; let's try a single-character
        // tokenizer.
        tokenize_single(first_ch).map(|token| Tokenized {
            len: 1,
            token: Some(token),
        })
    })
    .map(Ok)
    .unwrap_or_else(|| {
        // Neither a compatible multi- nor single- character tokenizer was found.
        Err(Error::TokenizerNotFound { first_ch })
    })
}

/// Translates IMPL code into a token, based on the first character, with a given multi-character
/// tokenizer.
fn tokenize_multi(
    input: &mut Peekable<impl Iterator<Item = Tokenizable>>,
    tokenizer: &Tokenizer,
    first_ch: char,
) -> Tokenized {
    // The token in text form.
    let mut raw_token = first_ch.to_string();

    // Continue processing characters with the selected tokenizer. Use [`Peekable::peek`] so that
    // rejected characters may be processed again through a different tokenizer.
    while let Some(Tokenizable { idx: _, ch }) = input.peek().copied() {
        if (tokenizer.accepts)(raw_token.as_str(), ch) {
            // The tokenizer accepts the next character; append it to the running raw token.
            raw_token.push(ch);
            // Manually advance the iterator because [`Peekable::peek`] does not.
            let _ = input.next();
        } else {
            // The tokenizer rejected the character. `raw_token` is now complete.
            break;
        }
    }

    Tokenized {
        len: raw_token.len(),
        token: (tokenizer.tokenize)(raw_token),
    }
}

/// Translates a single character into a token.
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
    /// The length, in characters, of the original token text.
    len: usize,
    /// The token, if one should be included.
    token: Option<Token>,
}

#[derive(Debug)]
pub enum Error {
    TokenizerNotFound { first_ch: char },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenizerNotFound { first_ch } => todo!(),
        }
    }
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
        match self {
            Self::LParen => write!(f, "'('"),
            Self::RParen => write!(f, "')'"),
            Self::LBrace => write!(f, "'{{'"),
            Self::RBrace => write!(f, "'}}'"),
            Self::Plus => write!(f, "'+'"),
            Self::Minus => write!(f, "'-'"),
            Self::Star => write!(f, "'*'"),
            Self::Slash => write!(f, "'/'"),
            Self::Dollar => write!(f, "'$'"),
            Self::Hash => write!(f, "'#'"),
            Self::Rational(it) => write!(f, "{}", it),
            Self::StrLit(it) => write!(f, "\"{}\"", it),
            Self::Symbol(it) => write!(f, "\\{}", it),
        }
    }
}

struct Tokenizer {
    /// Determines if the next character should be appended to the current raw token.
    accepts: fn(current: &str, next: char) -> bool,
    /// Produces a token from its raw text form.
    tokenize: fn(raw: String) -> Option<Token>,
}

/// A tokenizer that accepts symbols.
const SYMBOL_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |current, next| {
        if current.is_empty() {
            // Symbols are prefixed with a backslash.
            next == '\\'
        } else {
            // Following the backslash is an alphabetic sequence.
            next.is_ascii_alphabetic()
        }
    },
    tokenize: |mut raw| {
        // Remove the backslash.
        raw.remove(0);

        Some(Token::Symbol(raw))
    },
};
