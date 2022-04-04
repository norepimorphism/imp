use super::err::{self, Error};
use crate::{b::Token, span::Span};
use std::ops::Range;

pub struct Tokens {
    inner: Vec<Span<Token>>,
    idx: usize,
}

impl Tokens {
    pub fn new(inner: Vec<Span<Token>>) -> Self {
        Self { inner, idx: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.idx >= self.inner.len()
    }

    pub fn expect(
        &mut self,
        err_subj: err::Subject,
        pred: impl Fn(&Span<Token>) -> bool,
    ) -> Result<Span<Token>, Span<Error>> {
        let peeked = self
            .peek()
            .ok_or_else(|| self.fail(Error::expected(err_subj.clone())))?;

        if pred(&peeked) {
            self.advance();

            Ok(peeked)
        } else {
            Err(peeked.map(|_| Error::invalid(err_subj.clone())))
        }
    }

    pub fn expected(&self, subject: err::Subject) -> Span<Error> {
        self.fail(Error::expected(subject))
    }

    pub fn fail(&self, e: Error) -> Span<Error> {
        Span::new(e, self.peek_next_span_range())
    }

    pub fn peek_next_span_range(&self) -> Range<usize> {
        self.peek().map_or_else(
            || {
                let prev_range = self.peek_prev_span_range();

                (prev_range.end)..(prev_range.end + 1)
            },
            |token| token.range,
        )
    }

    pub fn peek_prev_span_range(&self) -> Range<usize> {
        self.inner
            .get(self.idx.saturating_sub(1))
            .map_or(0..1, |token| token.range.clone())
    }

    pub fn peek(&self) -> Option<Span<Token>> {
        self.inner.get(self.idx).cloned()
    }

    pub fn next(&mut self) -> Option<Span<Token>> {
        let next = self.peek();
        self.advance();

        next
    }

    pub fn advance(&mut self) {
        self.idx += 1;
    }
}
