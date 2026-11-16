// Singly Linked List — reference solution.
// `Option<Box<Node>>` links rewired with `Option::take`; `len` is cached, so
// every front operation is O(1). `reverse` and `Drop` walk the list in a loop
// (O(n) time, O(1) stack), so even very long lists never recurse.

use std::mem;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push_front(&mut self, value: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { elem: value, next }));
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        self.len -= 1;
        Some(node.elem)
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn reverse(&mut self) {
        let mut reversed: Link<T> = None;
        let mut rest = self.head.take();
        while let Some(mut node) = rest {
            rest = mem::replace(&mut node.next, reversed);
            reversed = Some(node);
        }
        self.head = reversed;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Unhook each node's tail before the node is dropped, so no drop recurses.
        let mut rest = self.head.take();
        while let Some(mut node) = rest {
            rest = node.next.take();
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();
        let mut tail = &mut list.head;
        for elem in iter {
            tail = &mut tail.insert(Box::new(Node { elem, next: None })).next;
            list.len += 1;
        }
        list
    }
}

/// Borrowing iterator, front to back.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let node = self.next?;
        self.next = node.next.as_deref();
        Some(&node.elem)
    }
}

/// Mutably borrowing iterator, front to back.
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        let node = self.next.take()?;
        self.next = node.next.as_deref_mut();
        Some(&mut node.elem)
    }
}

/// Owning iterator, front to back.
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len(), Some(self.list.len()))
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
