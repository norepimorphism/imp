use crate::error::{self, Error};

#[derive(Clone)]
pub struct Saved<T> {
    pub inner: T,
    pub pos: usize,
}

impl<T> Saved<T> {
    pub fn new(inner: T, pos: usize) -> Self {
        Self { inner, pos }
    }
}

#[derive(Default)]
pub struct Iter<T> {
    items: Vec<Saved<T>>,
    idx: usize,
    prev_pos: usize,
}

impl<T> Iter<T> {
    pub fn new(items: Vec<Saved<T>>) -> Self {
        Self {
            items,
            idx: 0,
            prev_pos: 0,
        }
    }

    pub fn prev_pos(&self) -> usize {
        self.prev_pos
    }

    pub fn next_pos(&self) -> usize {
        if let Some(item) = self.peek() {
            item.pos
        } else {
            self.prev_pos
        }
    }

    pub fn peek(&self) -> Option<&Saved<T>> {
        self.items.get(self.idx)
    }
}

impl<T: PartialEq> Iter<T> where Saved<T>: Clone {
    pub fn expect(&mut self, expected: &T, error_class: error::Class) -> Result<(), Error> {
        let actual = self
            .peek()
            .ok_or_else(|| Error::expected(error_class, self.next_pos()))?;

        if actual.inner == *expected {
            let _ = self.next();
            Ok(())
        } else {
            Err(Error::expected(error_class, actual.pos))
        }
    }
}

impl<T> Iterator for Iter<T> where Saved<T>: Clone {
    type Item = Saved<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_saved = self.items.get(self.idx).cloned();
        self.idx += 1;

        if let Some(saved) = maybe_saved {
            self.prev_pos = saved.pos;
        }

        maybe_saved
    }
}
