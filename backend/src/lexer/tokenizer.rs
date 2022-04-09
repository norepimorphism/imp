// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Token;

pub struct Tokenizer {
    /// Determines if the next character should be appended to the current raw token.
    pub accepts: fn(current: &str, next: char) -> bool,
    /// Produces a token from its raw text form.
    pub tokenize: fn(raw: String) -> Option<Token>,
}

/// A tokenizer that consumes comments.
pub const COMMENT: Tokenizer = Tokenizer {
    accepts: |current, next| {
        if current.is_empty() {
            next == ';'
        } else {
            next != '\n'
        }
    },
    tokenize: |_| None,
};

/// A tokenizer that accepts rational numbers.
pub const RATIONAL: Tokenizer = Tokenizer {
    accepts: |_, next| {
        // A rational number may include a digit or decimal point in any location, including the
        // first and last character.
        // TODO: Make decimal point configurable to a comma.
        next.is_ascii_digit() || (next == '.')
    },
    tokenize: |raw| Some(Token::Rational(raw)),
};

/// A tokenizer that accepts string literals.
pub const STR_LIT: Tokenizer = Tokenizer {
    accepts: |current, next| {
        if current.is_empty() {
            // A string literal is prefixed by double quotes.
            next == '"'
        } else if current.len() == 1 {
            // The second character is always accepted, even in the case of an empty string literal
            // (i.e., "").
            true
        } else {
            // The content of the string, as well as the final double quote, are accepted.
            !current.ends_with('"')
        }
    },
    tokenize: |mut raw| {
        // Remove the surrounding double quotes.
        let _ = raw.remove(0);
        let _ = raw.pop();

        Some(Token::StrLit(raw))
    },
};

/// A tokenizer that accepts symbols.
pub const SYMBOL: Tokenizer = Tokenizer {
    accepts: |_, next| {
        next.is_ascii_alphabetic()
    },
    tokenize: |raw| {
        Some(Token::Symbol(raw))
    },
};

/// A tokenizer that consumes whitespace.
pub const WHITESPACE: Tokenizer = Tokenizer {
    accepts: |_, next| next.is_ascii_whitespace(),
    // Whitespace is not necessary for parsing, so it is simply stripped out.
    tokenize: |_| None,
};
