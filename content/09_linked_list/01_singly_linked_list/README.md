## Description

Build `LinkedList<T>`, a singly linked list of heap-allocated nodes, each owning
the next one through an `Option<Box<Node<T>>>`. Operations happen at the front.

The algorithms are short; the lesson is moving owned boxes around without
fighting the borrow checker, and writing the iterators and traits a Rust
collection is expected to have:

- `new`, `len`, `is_empty`, `push_front`, `pop_front`, `peek_front`,
  `peek_front_mut`, and `Default`
- `reverse`, **in place**: re-link the existing nodes, allocate nothing
- `iter` (`&T`), `iter_mut` (`&mut T`), and an owning `IntoIterator`, all
  front to back, plus `IntoIterator` for `&LinkedList` and `&mut LinkedList`
  so `for` loops work in all three forms
- `FromIterator`, keeping iteration order: the first item is at the front
- an **iterative** `Drop`

That last one matters. Dropping a `Box<Node>` drops its `next`, which drops
its `next`… one stack frame per node. The compiler-generated drop of a
100 000-node list overflows the stack and kills the process. Your `Drop` must
dismantle the list in a loop.

`Node<T>` and the `Link<T>` alias are given in the template.

## Restrictions

- No `std::collections::LinkedList`, `VecDeque` or `Vec` inside the list: the
  nodes are `Box<Node<T>>`.
- Safe Rust only: no `unsafe`.

## Signature

```rust
type Link<T> = Option<Box<Node<T>>>;
struct Node<T> { elem: T, next: Link<T> }

pub struct LinkedList<T> { /* your fields */ }

impl<T> LinkedList<T> {
    pub fn new() -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn push_front(&mut self, value: T);
    pub fn pop_front(&mut self) -> Option<T>;
    pub fn peek_front(&self) -> Option<&T>;
    pub fn peek_front_mut(&mut self) -> Option<&mut T>;
    pub fn reverse(&mut self);
    pub fn iter(&self) -> Iter<'_, T>;
    pub fn iter_mut(&mut self) -> IterMut<'_, T>;
}

impl<T> Drop for LinkedList<T> { /* iterative */ }
impl<T> Default for LinkedList<T> { /* … */ }
impl<T> FromIterator<T> for LinkedList<T> { /* … */ }
impl<T> IntoIterator for LinkedList<T> { type Item = T; type IntoIter = IntoIter<T>; /* … */ }
impl<'a, T> IntoIterator for &'a LinkedList<T> { type Item = &'a T; /* … */ }
impl<'a, T> IntoIterator for &'a mut LinkedList<T> { type Item = &'a mut T; /* … */ }
```

`T` needs no bounds at all.

## Examples

```rust
let mut list = LinkedList::new();
list.push_front(3);
list.push_front(2);
list.push_front(1);                       // 1 -> 2 -> 3
assert_eq!(list.len(), 3);
assert_eq!(list.peek_front(), Some(&1));

list.reverse();                           // 3 -> 2 -> 1
assert_eq!(list.iter().copied().collect::<Vec<_>>(), [3, 2, 1]);

for x in &mut list {
    *x *= 10;                             // 30 -> 20 -> 10
}
assert_eq!(list.pop_front(), Some(30));
assert_eq!(list.into_iter().collect::<Vec<_>>(), [20, 10]);

let words: LinkedList<&str> = ["a", "b", "c"].into_iter().collect();
assert_eq!(words.peek_front(), Some(&"a"));   // keeps iteration order
```

## Constraints

- Up to `10^5` nodes in one list; dropping, reversing and iterating such a
  list must not recurse.
- Every element is dropped exactly once.

## Rust notes

- `Option::take`, `Option::as_deref`, `Option::as_deref_mut` and
  `Option::insert` cover nearly every pointer move you need.
- Building in order (for `FromIterator`) can keep a cursor
  `tail: &mut Link<T>` pointing at the empty link to fill next.
- A type that implements `Drop` can't have fields moved out of it; an owning
  iterator can simply wrap the whole list and call `pop_front`.
- `IterMut`'s `next` must not hand out two `&mut` to the same node: move the
  current reference out of `self` before splitting it into `&mut elem` and the
  next link.
