use crate::lexer::Token;
use std::fmt;
use tokens::Tokens;

pub fn parse(tokens: impl Iterator<Item = Token>) -> Result<Ast, Error> {
    Ast::parse(Tokens::new(tokens))
}

mod tokens {
    use std::iter::Peekable;
    use super::{Error, Token};

    pub struct Tokens<I: Iterator<Item = Token>>(Peekable<I>);

    impl<I: Iterator<Item = Token>> Tokens<I> {
        pub fn new(iter: I) -> Self {
            Self(iter.peekable())
        }

        pub fn peek(&mut self) -> Option<&Token> {
            self.0.peek()
        }

        pub fn next(&mut self) -> Option<Token> {
            self.0.next()
        }

        pub fn expect(&mut self, expected: &Token) -> Result<(), Error> {
            let actual = self
                .0
                .peek()
                .ok_or_else(|| Error::ExpectedToken(expected.clone()))?;

            if *actual == *expected {
                Ok(())
            } else {
                Err(Error::ExpectedToken(expected.clone()))
            }
        }

        pub fn is_empty(&mut self) -> bool {
            self.0.peek().is_none()
        }
    }
}
