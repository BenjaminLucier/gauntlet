// Doubly Linked List (Arena) — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: one Vec of slots linked by index — no LinkedList/VecDeque, no Rc/RefCell, no Box-linked nodes. No unsafe.
#![allow(unused_imports, unused_variables, dead_code)]

use std::marker::PhantomData;
use std::mem;

/// Refers to one element of a `DList`: the index of its slot in the arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(usize);

pub struct DList<T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<T>,
}

impl<T> DList<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn push_front(&mut self, value: T) -> Handle {
        todo!()
    }

    pub fn push_back(&mut self, value: T) -> Handle {
        todo!()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        todo!()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        todo!()
    }

    /// Insert `value` right after the element `at`.
    /// Invalid handle: the value is handed back as `Err(value)`.
    pub fn insert_after(&mut self, at: Handle, value: T) -> Result<Handle, T> {
        todo!()
    }

    /// Unlink and return the element `at`; its slot is reused later.
    pub fn remove(&mut self, at: Handle) -> Option<T> {
        todo!()
    }

    pub fn get(&self, at: Handle) -> Option<&T> {
        todo!()
    }

    pub fn get_mut(&mut self, at: Handle) -> Option<&mut T> {
        todo!()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        todo!()
    }
}

impl<T> Default for DList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Borrowing iterator over a `DList`, front to back.
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

/// Walks from the back: `list.iter().rev()`, following the `prev` links.
/// Mixing `next` and `next_back` yields every element exactly once.
impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a DList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
