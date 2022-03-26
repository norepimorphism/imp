use crate::error::{self, Error};
use std::{fmt, ops::Range};

#[derive(Clone)]
pub struct Span<T> {
    pub inner: T,
    pub range: Range<usize>,
}

impl<T: fmt::Display> fmt::Display for Span<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}@[{}..{}]",
            self.inner, self.range.start, self.range.end
        )
    }
}

impl<T> Span<T> {
    pub fn new(inner: T, range: Range<usize>) -> Self {
        Self { inner, range }
    }
}

#[derive(Default)]
pub struct Iter<T> {
    items: Vec<Span<T>>,
    idx: usize,
}

impl<T> Iter<T> {
    pub fn new(items: Vec<Span<T>>) -> Self {
        Self { items, idx: 0 }
    }
}

impl<T> Iter<T>
where
    Span<T>: Clone,
{
    pub fn peek_or(&mut self, class: error::Class) -> Result<Span<T>, Error> {
        self.get_or(class, self.peek().cloned())
    }

    pub fn next_or(&mut self, class: error::Class) -> Result<Span<T>, Error> {
        let it = self.next();
        self.get_or(class, it)
    }
}

impl<T: PartialEq> Iter<T>
where
    Span<T>: Clone,
{
    pub fn expect_or(&mut self, expected: &T, class: error::Class) -> Result<Span<T>, Error> {
        let actual = self.next_or(class.clone())?;
        if actual.inner == *expected {
            Ok(actual)
        } else {
            Err(Error::expected(class, actual.range))
        }
    }
}

impl<T> Iter<T>
where
    Span<T>: Clone,
{
    fn get_or(&mut self, class: error::Class, it: Option<Span<T>>) -> Result<Span<T>, Error> {
        it.ok_or_else(|| Error::expected(class, self.next_range()))
    }
}

impl<T> Iter<T> {
    pub fn peek(&self) -> Option<&Span<T>> {
        self.items.get(self.idx)
    }
}

impl<T> ExactSizeIterator for Iter<T> where Span<T>: Clone {}

impl<T> Iterator for Iter<T>
where
    Span<T>: Clone,
{
    type Item = Span<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_saved = self.items.get(self.idx).cloned();
        self.idx += 1;

        maybe_saved
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.items.len() - self.idx;
        (len, Some(len))
    }
}

impl<T> Iter<T> {
    pub fn prev_range(&self) -> Range<usize> {
        self.items
            .get(self.idx.saturating_sub(1))
            .map(|it| it.range.clone())
            .unwrap_or(0..1)
    }

    pub fn next_range(&self) -> Range<usize> {
        self.peek().map(|it| it.range.clone()).unwrap_or_else(|| {
            let prev = self.prev_range();
            (prev.start + 1)..(prev.end + 1)
        })
    }
}
