// Ring Buffer Deque — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no VecDeque/Vec/collections for storage — a Box<[Option<T>]> ring buffer. No unsafe.
#![allow(unused_imports, unused_variables, dead_code)]

use std::marker::PhantomData;
use std::mem;
use std::ops::Index;

pub struct RingDeque<T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<T>,
}

impl<T> RingDeque<T> {
    pub fn new() -> Self {
        todo!()
    }

    /// Room for `capacity` elements before the first reallocation.
    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Number of slots in the buffer.
    pub fn capacity(&self) -> usize {
        todo!()
    }

    pub fn push_front(&mut self, value: T) {
        todo!()
    }

    pub fn push_back(&mut self, value: T) {
        todo!()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        todo!()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        todo!()
    }

    pub fn front(&self) -> Option<&T> {
        todo!()
    }

    pub fn back(&self) -> Option<&T> {
        todo!()
    }

    /// The element `index` places from the front.
    pub fn get(&self, index: usize) -> Option<&T> {
        todo!()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        todo!()
    }
}

impl<T> Default for RingDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Borrowing iterator over a `RingDeque`, front to back.
pub struct Iter<'a, T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        todo!()
    }
}

/// Walks from the back: `d.iter().rev()`. Front and back meet in the middle.
impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a RingDeque<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        todo!()
    }
}

impl<T> Index<usize> for RingDeque<T> {
    type Output = T;

    /// Like `get`, counting from the front, but panics ("index … out of bounds …")
    /// if `index >= len`.
    fn index(&self, index: usize) -> &T {
        todo!()
    }
}

/// Pushes every item onto the back, in order.
impl<T> Extend<T> for RingDeque<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        todo!()
    }
}

/// Front to back in iteration order.
impl<T> FromIterator<T> for RingDeque<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
