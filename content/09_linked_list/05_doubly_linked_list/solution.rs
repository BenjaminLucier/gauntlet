// Doubly Linked List (Arena) — reference solution.
// Nodes live in a Vec of slots and link to each other by index; removed slots
// form a free list that later inserts reuse. Every operation is O(1)
// (amortized when the Vec grows); space is O(peak length).

use std::mem;

/// Refers to one element of a `DList`: the index of its slot in the arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(usize);

enum Slot<T> {
    Used {
        value: T,
        prev: Option<usize>,
        next: Option<usize>,
    },
    Free {
        next_free: Option<usize>,
    },
}

pub struct DList<T> {
    slots: Vec<Slot<T>>,
    head: Option<usize>,
    tail: Option<usize>,
    /// First slot of the free list, threaded through `Slot::Free::next_free`.
    free: Option<usize>,
    len: usize,
}

impl<T> DList<T> {
    pub fn new() -> Self {
        DList {
            slots: Vec::new(),
            head: None,
            tail: None,
            free: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// `(prev, next)` of an occupied slot, or `None` for a free / unknown one.
    fn links(&self, i: usize) -> Option<(Option<usize>, Option<usize>)> {
        match self.slots.get(i)? {
            Slot::Used { prev, next, .. } => Some((*prev, *next)),
            Slot::Free { .. } => None,
        }
    }

    fn set_next(&mut self, i: Option<usize>, to: Option<usize>) {
        match i {
            None => self.head = to,
            Some(i) => match &mut self.slots[i] {
                Slot::Used { next, .. } => *next = to,
                Slot::Free { .. } => unreachable!("a linked neighbour is in use"),
            },
        }
    }

    fn set_prev(&mut self, i: Option<usize>, to: Option<usize>) {
        match i {
            None => self.tail = to,
            Some(i) => match &mut self.slots[i] {
                Slot::Used { prev, .. } => *prev = to,
                Slot::Free { .. } => unreachable!("a linked neighbour is in use"),
            },
        }
    }

    /// Put `value` in a slot between `prev` and `next` (adjacent, or `None`
    /// for the list's ends) and patch the links on both sides.
    fn link(&mut self, prev: Option<usize>, value: T, next: Option<usize>) -> Handle {
        let slot = Slot::Used { value, prev, next };
        let i = match self.free {
            Some(i) => {
                let old = mem::replace(&mut self.slots[i], slot);
                let Slot::Free { next_free } = old else {
                    unreachable!("free list only holds free slots")
                };
                self.free = next_free;
                i
            }
            None => {
                self.slots.push(slot);
                self.slots.len() - 1
            }
        };
        self.set_next(prev, Some(i));
        self.set_prev(next, Some(i));
        self.len += 1;
        Handle(i)
    }

    pub fn push_front(&mut self, value: T) -> Handle {
        self.link(None, value, self.head)
    }

    pub fn push_back(&mut self, value: T) -> Handle {
        self.link(self.tail, value, None)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.remove(Handle(self.head?))
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.remove(Handle(self.tail?))
    }

    pub fn insert_after(&mut self, at: Handle, value: T) -> Result<Handle, T> {
        let Some((_, next)) = self.links(at.0) else {
            return Err(value);
        };
        Ok(self.link(Some(at.0), value, next))
    }

    pub fn remove(&mut self, at: Handle) -> Option<T> {
        let (prev, next) = self.links(at.0)?;
        let freed = Slot::Free {
            next_free: self.free,
        };
        let Slot::Used { value, .. } = mem::replace(&mut self.slots[at.0], freed) else {
            unreachable!("links() checked the slot is in use")
        };
        self.free = Some(at.0);
        self.set_next(prev, next);
        self.set_prev(next, prev);
        self.len -= 1;
        Some(value)
    }

    pub fn get(&self, at: Handle) -> Option<&T> {
        match self.slots.get(at.0)? {
            Slot::Used { value, .. } => Some(value),
            Slot::Free { .. } => None,
        }
    }

    pub fn get_mut(&mut self, at: Handle) -> Option<&mut T> {
        match self.slots.get_mut(at.0)? {
            Slot::Used { value, .. } => Some(value),
            Slot::Free { .. } => None,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            slots: &self.slots,
            front: self.head,
            back: self.tail,
            remaining: self.len,
        }
    }
}

impl<T> Default for DList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Borrowing iterator over a `DList`, front to back (or back to front via
/// `rev`). `remaining` stops the two ends once they have met.
pub struct Iter<'a, T> {
    slots: &'a [Slot<T>],
    front: Option<usize>,
    back: Option<usize>,
    remaining: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.remaining == 0 {
            return None;
        }
        let i = self.front?;
        let Slot::Used { value, next, .. } = &self.slots[i] else {
            unreachable!("every linked slot is in use")
        };
        self.front = *next;
        self.remaining -= 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let i = self.back?;
        let Slot::Used { value, prev, .. } = &self.slots[i] else {
            unreachable!("every linked slot is in use")
        };
        self.back = *prev;
        self.remaining -= 1;
        Some(value)
    }
}

impl<'a, T> IntoIterator for &'a DList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
