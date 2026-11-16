// Dynamic Array — reference solution.
// A boxed slice of `Option<T>` slots plus a length. A full buffer is replaced
// by one twice the size, so n pushes copy O(n) elements in total: push is
// amortized O(1). insert/remove shift a sub-slice with rotate. Space O(capacity).

use std::mem;
use std::ops::{Index, IndexMut};

const MIN_CAPACITY: usize = 4;

pub struct DynArray<T> {
    buf: Box<[Option<T>]>,
    len: usize,
}

fn empty_slots<T>(n: usize) -> Box<[Option<T>]> {
    (0..n).map(|_| None).collect()
}

impl<T> DynArray<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DynArray {
            buf: empty_slots(capacity),
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

    /// Make room for at least one more element.
    fn grow_if_full(&mut self) {
        if self.len < self.buf.len() {
            return;
        }
        let new_cap = (self.buf.len() * 2).max(MIN_CAPACITY);
        let old = mem::replace(&mut self.buf, empty_slots(new_cap));
        for (slot, value) in self.buf.iter_mut().zip(old) {
            *slot = value;
        }
    }

    pub fn push(&mut self, value: T) {
        self.grow_if_full();
        self.buf[self.len] = Some(value);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.len = self.len.checked_sub(1)?;
        self.buf[self.len].take()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.buf[..self.len].get(index)?.as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.buf[..self.len].get_mut(index)?.as_mut()
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), T> {
        if index > self.len {
            return Err(value);
        }
        self.push(value);
        // The new element is last; rotate it into place.
        self.buf[index..self.len].rotate_right(1);
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }
        self.buf[index..self.len].rotate_left(1);
        self.pop()
    }

    pub fn clear(&mut self) {
        self.buf[..self.len].iter_mut().for_each(|slot| *slot = None);
        self.len = 0;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            slots: &self.buf[..self.len],
        }
    }
}

/// Borrowing iterator over a `DynArray`, front to back.
pub struct Iter<'a, T> {
    slots: &'a [Option<T>],
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let (first, rest) = self.slots.split_first()?;
        self.slots = rest;
        first.as_ref()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.slots.len(), Some(self.slots.len()))
    }
}

impl<T> Default for DynArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for DynArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        let len = self.len;
        self.get(index)
            .unwrap_or_else(|| panic!("index {index} out of bounds for length {len}"))
    }
}

impl<T> IndexMut<usize> for DynArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        let len = self.len;
        self.get_mut(index)
            .unwrap_or_else(|| panic!("index {index} out of bounds for length {len}"))
    }
}

impl<T> FromIterator<T> for DynArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut array = DynArray::new();
        array.extend(iter);
        array
    }
}

impl<T> Extend<T> for DynArray<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.push(value);
        }
    }
}

/// Owning iterator over a `DynArray`, front to back. Takes each value out of
/// its slot; whatever is left is dropped with the buffer.
pub struct IntoIter<T> {
    array: DynArray<T>,
    front: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.front == self.array.len {
            return None;
        }
        self.front += 1;
        self.array.buf[self.front - 1].take()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.array.len - self.front;
        (left, Some(left))
    }
}

impl<T> IntoIterator for DynArray<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            array: self,
            front: 0,
        }
    }
}

impl<'a, T> IntoIterator for &'a DynArray<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
