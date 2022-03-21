use std::fmt;

/// Translates a raw string into lexical tokens.
pub fn lex(input: &str) -> Result<Vec<Token>, Error> {
    let mut input = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(next) = input.next() {
        let maybe_token = match next {
            '(' => Ok(Some(Token::LeftParen)),
            ')' => Ok(Some(Token::RightParen)),
            '+' => Ok(Some(Token::Plus)),
            '-' => Ok(Some(Token::Minus)),
            '/' => Ok(Some(Token::Slash)),
            '*' => Ok(Some(Token::Star)),
            _ => {
                [
                    NUMBER_TOKENIZER,
                    WORD_TOKENIZER,
                    WHITESPACE_TOKENIZER,
                    COMMENT_TOKENIZER,
                ]
                .into_iter()
                .find(|tokenizer| (tokenizer.accepts)("", next))
                .map_or_else(
                    || Err(Error::InvalidInput(next)),
                    |tokenizer| {
                        let mut raw_token = next.to_string();

                        while let Some(next) = input.peek().copied() {
                            if (tokenizer.accepts)(raw_token.as_str(), next) {
                                raw_token.push(next);
                                let _ = input.next();
                            } else {
                                break;
                            }
                        }

                        Ok((tokenizer.tokenize)(raw_token))
                    }
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
        (prev.is_empty() && (next == '-')) || next.is_ascii_digit()
    },
    tokenize: |raw| Some(Token::Number(raw)),
};

const WORD_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |_, next| next.is_ascii_alphabetic(),
    tokenize: |raw| Some(Token::Word(raw)),
};

const WHITESPACE_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |_, next| next.is_ascii_whitespace(),
    tokenize: |_| None,
};

const COMMENT_TOKENIZER: Tokenizer = Tokenizer {
    accepts: |prev, next| {
        if prev.is_empty() {
            next == ';'
        } else {
            next != '\n'
        }
    },
    tokenize: |_| None,
};

struct Tokenizer {
    accepts: fn(prev: &str, next: char) -> bool,
    tokenize: fn(raw: String) -> Option<Token>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Slash,
    Star,
    Number(String),
    Word(String),
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
