use std::fmt;

/// Translates a raw string into lexical tokens.
pub fn lex(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut input = input.chars().peekable();

    // Process the entire `input` string, one character at a time.
    while let Some(new) = input.next() {
        let maybe_token = match new {
            // Single-character tokens.
            '(' => Ok(Some(Token::LParen)),
            ')' => Ok(Some(Token::RParen)),
            '+' => Ok(Some(Token::Plus)),
            '-' => Ok(Some(Token::Minus)),
            '*' => Ok(Some(Token::Star)),
            '/' => Ok(Some(Token::Slash)),
            '$' => Ok(Some(Token::Dollar)),
            // Multi-character tokens.
            _ => {
                // Try each tokenizer in this order.
                [
                    RATIO_TOKENIZER,
                    STR_LIT_TOKENIZER,
                    SYMBOL_TOKENIZER,
                    WHITESPACE_TOKENIZER,
                    COMMENT_TOKENIZER,
                ]
                .into_iter()
                // Select the first tokenizer to accept the given character.
                .find(|tokenizer| (tokenizer.accepts)("", new))
                .map_or_else(
                    // The character was not accepted by any tokenizers, so it is considered
                    // invalid.
                    || Err(Error::InvalidInput(new)),
                    // A compatible tokenizer was found.
                    |tokenizer| {
                        let mut current_token = new.to_string();

                        // Continue processing characters with the selected tokenizer. Use
                        // [`Peekable::peek`] so that rejected characters may be processed again.
                        while let Some(new) = input.peek().copied() {
                            if (tokenizer.accepts)(current_token.as_str(), new) {
                                current_token.push(new);
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
            tokens.push(token);
        }
    }

    Ok(tokens)
}

/// A tokenizer that accepts rational numbers.
const RATIO_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |current, new| {
        if new.is_ascii_digit() {
            return true;
        }

        if current.is_empty() {
            // A number may be prefixed by its sign.
            (new == '-') || (new == '+')
        } else {
            // A number may also include a decimal point in any location, including the first and
            // last character.
            // TODO: Make this configurable to a comma.
            new == '.'
        }
    },
    tokenize: |raw| Some(Token::Number(raw)),
};

/// A tokenizer that accepts string literals.
const STR_LIT_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |current, new| {
        if current.is_empty() {
            // A string literal is prefixed by double quotes.
            return new == '"';
        }

        if current.len() == 1 {
            // The second character is always accepted, even in the case of an empty string literal
            // (i.e., "").
            return true;
        }

        // The content of the string, as well as the final double quote, are accepted.
        !current.ends_with('"')
    },
    tokenize: |mut raw| {
        raw.remove(0);
        raw.pop();

        Some(Token::StrLit(raw))
    }
};

/// A tokenizer that accepts symbols.
const SYMBOL_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |current, new| {
        if new.is_alphabetic() {
            // A symbol may always include alphabetic characters at any position.
            return true;
        }

        if !current.is_empty() {
            // A symbol may also include numeric characters as well as `-`, but neither can be the
            // first character.
            return new.is_ascii_digit() || (new == '-');
        }

        false
    },
    tokenize: |raw| Some(Token::Symbol(raw)),
};

/// A tokenizer that consumes whitespace.
const WHITESPACE_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |_, new| new.is_ascii_whitespace(),
    tokenize: |_| None,
};

/// A tokenizer that consumes comments.
const COMMENT_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |current, new| {
        if current.is_empty() {
            return new == ';';
        }

        new != '\n'
    },
    tokenize: |_| None,
};

struct Tokenizer {
    accepts: fn(current: &str, new: char) -> bool,
    tokenize: fn(raw: String) -> Option<Token>,
}

/// A lexical token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    /// A left, or opening, parenthesis.
    LParen,
    /// A right, or closing, parenthesis.
    RParen,
    /// A plus sign (+).
    Plus,
    /// A minus sign (-).
    Minus,
    /// An asterisk (*).
    Star,
    /// A forward slash (/).
    Slash,
    /// A dollar sign ($).
    Dollar,
    /// A rational number.
    Number(String),
    /// A string literal.
    StrLit(String),
    /// A symbol.
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
