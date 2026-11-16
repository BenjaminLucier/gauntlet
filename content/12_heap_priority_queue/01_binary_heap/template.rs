// Binary Heap — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no std BinaryHeap, no sort/sort_unstable. Storage is one Vec<T>; no Clone bound.
#![allow(unused_imports, unused_variables, dead_code)]

use std::marker::PhantomData;

pub struct MaxHeap<T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<T>,
}

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn push(&mut self, item: T) {
        todo!()
    }

    /// Remove and return the largest item.
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    /// Borrow the largest item.
    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    /// Borrow every item, in the heap's internal (array) order — not sorted.
    pub fn iter(&self) -> Iter<'_, T> {
        todo!()
    }

    /// Consume the heap and return its storage, in the same order as `iter`.
    pub fn into_vec(self) -> Vec<T> {
        todo!()
    }
}

impl<T: Ord> Default for MaxHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> From<Vec<T>> for MaxHeap<T> {
    /// Bottom-up heapify: O(n).
    fn from(items: Vec<T>) -> Self {
        todo!()
    }
}

impl<T: Ord> FromIterator<T> for MaxHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}

/// Pushes every item.
impl<T: Ord> Extend<T> for MaxHeap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        todo!()
    }
}

/// Borrowing iterator over a heap's items, in array (level) order.
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

impl<'a, T: Ord> IntoIterator for &'a MaxHeap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
