use super::Error;
use crate::lexer::Token;
use std::iter::Peekable;

pub struct Stream<I: Iterator<Item = Token>>(Peekable<I>);

impl<I: Iterator<Item = Token>> Stream<I> {
    pub fn new(iter: I) -> Self {
        Self(iter.peekable())
    }

    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn advance(&mut self) {
        let _ = self.next();
    }

    pub fn next(&mut self) -> Option<Token> {
        self.0.next()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.0.peek()
    }

    pub fn expect(&mut self, expected: &Token) -> Result<(), Error> {
        let actual = self
            .peek()
            .ok_or_else(|| Error::ExpectedToken(expected.clone()))?;

        if *actual == *expected {
            let _ = self.next();

            Ok(())
        } else {
            Err(Error::ExpectedToken(expected.clone()))
        }
    }
}
