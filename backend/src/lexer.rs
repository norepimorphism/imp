use std::fmt;

/// Translates a raw string into lexical tokens.
pub fn lex(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut input = input.chars().peekable();

    // Process the entire string, one character at a time.
    while let Some(next) = input.next() {
        let maybe_token = match next {
            // Single-character tokens.
            '(' => Ok(Some(Token::LParen)),
            ')' => Ok(Some(Token::RParen)),
            '+' => Ok(Some(Token::Plus)),
            '-' => Ok(Some(Token::Minus)),
            '*' => Ok(Some(Token::Star)),
            '/' => Ok(Some(Token::Slash)),
            // Multi-character tokens.
            _ => {
                // Try each tokenizer, in this order.
                [
                    NUMBER_TOKENIZER,
                    SYMBOL_TOKENIZER,
                    WHITESPACE_TOKENIZER,
                    COMMENT_TOKENIZER,
                ]
                .into_iter()
                // Select the first tokenizer to accept the given character.
                .find(|tokenizer| (tokenizer.accepts)(None, next))
                .map_or_else(
                    || Err(Error::InvalidInput(next)),
                    |tokenizer| {
                        let mut raw_token = next.to_string();

                        // Continue processing characters with the selected tokenizer.
                        while let Some(next) = input.peek().copied() {
                            if (tokenizer.accepts)(Some(raw_token.as_str()), next) {
                                raw_token.push(next);
                                // Manually advance the iterator because [`Iterator::peek`] does
                                // not.
                                let _ = input.next();
                            } else {
                                // The tokenizer rejected the next character. `raw_token` is now
                                // complete.
                                break;
                            }
                        }

                        // Translate `raw_token` into a [`Token`].
                        Ok((tokenizer.tokenize)(raw_token))
                    },
                )
            }
        }?;

        if let Some(token) = maybe_token {
            tokens.push(token);
        }
    }

    Ok(tokens)
}

const NUMBER_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |prev, next| {
        // A number may be prefixed by its sign.
        (prev.is_none() && ((next == '-') || (next == '+')))
        // A number may also include a decimal point in any location, including the first and last.
        || (next == '.')
        || next.is_ascii_digit()
    },
    tokenize: |raw| Some(Token::Number(raw)),
};

const SYMBOL_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |prev, next| {
        // A symbol may always include alphabetic characters at any position.
        next.is_alphabetic()
        // A symbol may also include numeric characters as well as `-`, but neither can be the first
        // character.
        || (prev.is_some() && (next.is_ascii_digit() || (next == '-')))

    },
    tokenize: |raw| Some(Token::Symbol(raw)),
};

const WHITESPACE_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |_, next| next.is_ascii_whitespace(),
    tokenize: |_| None,
};

const COMMENT_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |prev, next| {
        if prev.is_none() {
            next == ';'
        } else {
            next != '\n'
        }
    },
    tokenize: |_| None,
};

struct Tokenizer {
    accepts: fn(prev: Option<&str>, next: char) -> bool,
    tokenize: fn(raw: String) -> Option<Token>,
}

/// A lexical token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    Slash,
    /// A rational number.
    Number(String),
    Symbol(String),
}

#[derive(Debug)]
pub enum Error {
    InvalidInput(char),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
