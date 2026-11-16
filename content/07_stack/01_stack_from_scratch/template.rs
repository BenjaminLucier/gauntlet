// Stack — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: store elements in a Vec, using only its end — no VecDeque/LinkedList, no insert(0)/remove(0). No unsafe.
#![allow(unused_imports, unused_variables, dead_code)]

use std::marker::PhantomData;

pub struct Stack<T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, value: T) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Borrows the elements top first, as `pop` would return them.
    pub fn iter(&self) -> Iter<'_, T> {
        todo!()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Pushes the items in order: the last one ends up on top.
impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}

/// Owning iterator: yields the elements top first, as `pop` would.
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

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        todo!()
    }
}

/// Borrowing iterator: yields `&T` top first, leaving the stack unchanged.
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

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
