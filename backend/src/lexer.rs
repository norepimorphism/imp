use crate::{error::{self, Error}, saved::Saved};
use std::fmt;

/// Translates a raw string into [lexical tokens](Token).
pub fn lex(input: &str) -> Result<Vec<Saved<Token>>, Error> {
    // Raw input.
    let mut input = input.chars().enumerate().peekable();
    // Tokenized output.
    let mut tokens = Vec::new();

    // Process the entire input string, one character at a time.
    while let Some(next_token) = input
        .next()
        .map(|(pos, c)| Saved::new(c, pos))
    {
        let maybe_token = match next_token.inner {
            // Single-character tokens.
            '(' => Ok(Some(Token::LParen)),
            ')' => Ok(Some(Token::RParen)),
            '{' => Ok(Some(Token::LBrace)),
            '}' => Ok(Some(Token::RBrace)),
            '+' => Ok(Some(Token::Plus)),
            '-' => Ok(Some(Token::Minus)),
            '*' => Ok(Some(Token::Star)),
            '/' => Ok(Some(Token::Slash)),
            '$' => Ok(Some(Token::Dollar)),
            // Multi-character tokens.
            _ => {
                // Try each tokenizer in this order.
                [
                    RATIONAL_TOKENIZER,
                    STR_LIT_TOKENIZER,
                    SYMBOL_TOKENIZER,
                    WHITESPACE_TOKENIZER,
                    COMMENT_TOKENIZER,
                ]
                .into_iter()
                // Select the first tokenizer to accept the given character.
                .find(|tokenizer| (tokenizer.accepts)("", next_token.inner))
                .map_or_else(
                    // The character was not accepted by any tokenizers, so it is considered
                    // invalid.
                    || Err(Error {
                        kind: error::Kind::Invalid,
                        class: error::Class::Char,
                        pos: next_token.pos,
                    }),
                    // A compatible tokenizer was found.
                    |tokenizer| {
                        let mut current_token = next_token.inner.to_string();

                        // Continue processing characters with the selected tokenizer. Use
                        // [`Peekable::peek`] so that rejected characters may be processed again
                        // through a different tokenizer.
                        while let Some(new) = input.peek().copied() {
                            if (tokenizer.accepts)(current_token.as_str(), new.1) {
                                current_token.push(new.1);
                                // Manually advance the iterator because [`Peekable::peek`] does
                                // not.
                                let _ = input.next();
                            } else {
                                // The tokenizer rejected the new character. `current_token` is now
                                // complete.
                                break;
                            }
                        }

                        // Translate `current_token` into a [`Token`].
                        Ok((tokenizer.tokenize)(current_token))
                    },
                )
            }
        }?;

        if let Some(token) = maybe_token {
            tokens.push(Saved::new(token, next_token.pos));
        }
    }

    Ok(tokens)
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
    /// A rational number.
    Rational(String),
    /// A string literal.
    StrLit(String),
    /// A [symbol](crate::parser::Symbol).
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
            Self::Rational(it) => write!(f, "{}", it),
            Self::StrLit(it) => write!(f, "\"{}\"", it),
            Self::Symbol(it) => write!(f, "{}", it),
        }
    }
}

/// A tokenizer that accepts rational numbers.
const RATIONAL_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |current, new| {
        // A rational number may include a digit or decimal point in any location, including the
        // first and last character.
        // TODO: Make decimal point configurable to a comma.
        if new.is_ascii_digit() || (new == '.') {
            return true;
        }

        if current.is_empty() {
            // A number may also be prefixed by its sign.
            return (new == '-') || (new == '+');
        }

        false
    },
    tokenize: |it| Some(Token::Rational(it)),
};

/// A tokenizer that accepts string literals.
const STR_LIT_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |current, new| {
        if current.is_empty() {
            // A string literal is prefixed by double quotes.
            new == '"'
        } else if current.len() == 1 {
            // The second character is always accepted, even in the case of an empty string literal
            // (i.e., "").
            true
        } else {
            // The content of the string, as well as the final double quote, are accepted.
            !current.ends_with('"')
        }
    },
    tokenize: |mut it| {
        // Remove the surrounding double quotes.
        let _ = it.remove(0);
        let _ = it.pop();

        Some(Token::StrLit(it))
    }
};

/// A tokenizer that accepts symbols.
const SYMBOL_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |current, new| {
        if new.is_ascii_lowercase() {
            // A symbol may always include lowecase alphabetic characters in any position.
            return true;
        }

        if !current.is_empty() {
            // A symbol may also include numeric characters as well as `-` and '.', but they cannot
            // be the first character.
            return new.is_ascii_digit() || (new == '-');
        }

        false
    },
    tokenize: |it| Some(Token::Symbol(it)),
};

/// A tokenizer that consumes whitespace.
const WHITESPACE_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |_, new| new.is_ascii_whitespace(),
    // Whitespace is not necessary for parsing, so it is simply stripped out.
    tokenize: |_| None,
};

/// A tokenizer that consumes comments.
const COMMENT_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |current, new| {
        if current.is_empty() {
            new == ';'
        } else {
            new != '\n'
        }
    },
    tokenize: |_| None,
};

struct Tokenizer {
    accepts: fn(current: &str, new: char) -> bool,
    tokenize: fn(it: String) -> Option<Token>,
}
