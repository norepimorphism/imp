use std::iter::Peekable;

pub struct Stream<T, I: Iterator<Item = T>>(Peekable<I>);

impl<T, I: Iterator<Item = T>> Stream<T, I> {
    pub fn new(iter: I) -> Self {
        Self(iter.peekable())
    }

    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.0.peek()
    }

    pub fn advance(&mut self) {
        let _ = self.next();
    }

    pub fn next(&mut self) -> Option<T> {
        self.0.next()
    }

    pub fn expect(&mut self, expected: &T) -> Result<(), Error> {
        let actual = self
            .peek()
            .ok_or_else(|| Error::ExpectedToken(expected.clone()))?;

        if *actual == *expected {
            self.advance();

            Ok(())
        } else {
            Err(Error::ExpectedToken(expected.clone()))
        }
    }
}
