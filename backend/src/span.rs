// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
