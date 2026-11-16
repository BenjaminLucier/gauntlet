// Singly Linked List — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no std LinkedList/VecDeque/Vec inside the list — Box<Node<T>> links only. No unsafe.
#![allow(unused_imports, unused_variables, dead_code)]

use std::marker::PhantomData;
use std::mem;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        todo!()
    }

    /// `O(1)`.
    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn push_front(&mut self, value: T) {
        todo!()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        todo!()
    }

    pub fn peek_front(&self) -> Option<&T> {
        todo!()
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Reverse the list in place, re-linking the existing nodes.
    pub fn reverse(&mut self) {
        todo!()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        todo!()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        todo!()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // TODO: dismantle the list in a loop. Left empty, the fields are dropped
        // the default way, which recurses once per node and overflows the stack
        // on long lists. (Not a `todo!()`: a panic inside `drop` while another
        // panic is unwinding would abort every test.)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Keeps iteration order: the first item ends up at the front.
impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}

/// Borrowing iterator, front to back.
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

/// Mutably borrowing iterator, front to back.
pub struct IterMut<'a, T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        todo!()
    }
}

/// Owning iterator, front to back.
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

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
