use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Span<T> {
    pub inner: T,
    pub range: Range<usize>,
}

impl<T> Span<T> {
    pub fn new(inner: T, range: Range<usize>) -> Self {
        Self { inner, range }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Span<U> {
        Span {
            inner: f(self.inner),
            range: self.range,
        }
    }
}
