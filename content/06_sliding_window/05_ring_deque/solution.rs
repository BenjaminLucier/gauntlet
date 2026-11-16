// Ring Buffer Deque — reference solution.
// A boxed slice of `Option<T>` slots used circularly: logical index i lives in
// slot (head + i) % capacity. Both ends are O(1); a full buffer is replaced by
// one twice the size (elements re-laid out from slot 0), so pushes are
// amortized O(1). Space O(capacity).

use std::mem;
use std::ops::Index;

const MIN_CAPACITY: usize = 4;

pub struct RingDeque<T> {
    buf: Box<[Option<T>]>,
    /// Slot of the front element (meaningless while empty).
    head: usize,
    len: usize,
}

fn empty_slots<T>(n: usize) -> Box<[Option<T>]> {
    (0..n).map(|_| None).collect()
}

impl<T> RingDeque<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        RingDeque {
            buf: empty_slots(capacity),
            head: 0,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// Physical slot of logical index `i`. Only call with a non-empty buffer.
    fn slot(&self, i: usize) -> usize {
        (self.head + i) % self.buf.len()
    }

    /// Make room for one more element, unwrapping the ring into a bigger buffer.
    fn grow_if_full(&mut self) {
        if self.len < self.buf.len() {
            return;
        }
        let new_cap = (self.buf.len() * 2).max(MIN_CAPACITY);
        let mut old = mem::replace(&mut self.buf, empty_slots(new_cap));
        let old_cap = old.len();
        for (i, slot) in self.buf.iter_mut().take(self.len).enumerate() {
            *slot = old[(self.head + i) % old_cap].take();
        }
        self.head = 0;
    }

    pub fn push_front(&mut self, value: T) {
        self.grow_if_full();
        let cap = self.buf.len();
        self.head = (self.head + cap - 1) % cap;
        self.buf[self.head] = Some(value);
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        self.grow_if_full();
        let tail = self.slot(self.len);
        self.buf[tail] = Some(value);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let value = self.buf[self.head].take();
        self.head = (self.head + 1) % self.buf.len();
        self.len -= 1;
        value
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.len = self.len.checked_sub(1)?;
        let tail = self.slot(self.len);
        self.buf[tail].take()
    }

    pub fn front(&self) -> Option<&T> {
        self.get(0)
    }

    pub fn back(&self) -> Option<&T> {
        self.get(self.len.checked_sub(1)?)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        self.buf[self.slot(index)].as_ref()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        // The occupied slots are buf[head..] followed by the wrapped part at buf[..].
        let first_end = (self.head + self.len).min(self.buf.len());
        let first = &self.buf[self.head..first_end];
        let second = &self.buf[..self.len - first.len()];
        Iter { first, second }
    }
}

impl<T> Default for RingDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Borrowing iterator over a `RingDeque`, front to back.
pub struct Iter<'a, T> {
    first: &'a [Option<T>],
    second: &'a [Option<T>],
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.first.is_empty() {
            self.first = mem::take(&mut self.second);
        }
        let (slot, rest) = self.first.split_first()?;
        self.first = rest;
        slot.as_ref()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.first.len() + self.second.len();
        (n, Some(n))
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.second.is_empty() {
            self.second = mem::take(&mut self.first);
        }
        let (slot, rest) = self.second.split_last()?;
        self.second = rest;
        slot.as_ref()
    }
}

impl<'a, T> IntoIterator for &'a RingDeque<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T> Index<usize> for RingDeque<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        let len = self.len;
        self.get(index)
            .unwrap_or_else(|| panic!("index {index} out of bounds for length {len}"))
    }
}

impl<T> Extend<T> for RingDeque<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.push_back(value);
        }
    }
}

impl<T> FromIterator<T> for RingDeque<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut deque = RingDeque::new();
        deque.extend(iter);
        deque
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
