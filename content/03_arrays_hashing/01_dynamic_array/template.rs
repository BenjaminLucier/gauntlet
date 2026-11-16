// Dynamic Array — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no Vec/VecDeque/collections for storage — a Box<[Option<T>]> buffer. No unsafe.
#![allow(unused_imports, unused_variables, dead_code)]

use std::marker::PhantomData;
use std::mem;
use std::ops::{Index, IndexMut};

pub struct DynArray<T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<T>,
}

impl<T> DynArray<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }

    pub fn push(&mut self, value: T) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        todo!()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        todo!()
    }

    /// Insert at `index` (`0..=len`), shifting later elements right.
    /// Out of range: the value is handed back as `Err(value)`.
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), T> {
        todo!()
    }

    /// Remove and return the element at `index`, shifting later ones left.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        todo!()
    }

    /// Drop every element. The capacity is kept.
    pub fn clear(&mut self) {
        todo!()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        todo!()
    }
}

/// Borrowing iterator over a `DynArray`, front to back.
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

impl<T> Default for DynArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for DynArray<T> {
    type Output = T;

    /// Panics if `index >= len`, like `Vec`.
    fn index(&self, index: usize) -> &T {
        todo!()
    }
}

impl<T> IndexMut<usize> for DynArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        todo!()
    }
}

impl<T> FromIterator<T> for DynArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}

impl<T> Extend<T> for DynArray<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        todo!()
    }
}

/// Owning iterator over a `DynArray`, front to back.
pub struct IntoIter<T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        todo!()
    }
}

impl<T> IntoIterator for DynArray<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a DynArray<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
