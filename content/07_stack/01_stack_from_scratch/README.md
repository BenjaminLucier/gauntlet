## Description

Build `Stack<T>`, a last-in, first-out collection. The last value pushed is
the first one popped; `peek` looks at it without removing it.

The data structure is the easy part — a `Vec` already does the work. The
lesson is the API a Rust user expects of a collection:

- `new`, `push`, `pop`, `peek`, `peek_mut`, `len`, `is_empty`, and `Default`
- `FromIterator`, so `iter.collect::<Stack<_>>()` works: items are pushed in
  iteration order, so the **last** item ends up on top
- an owning `IntoIterator` (`for x in stack`) that yields the elements
  **top first**, i.e. in the order `pop` would return them, via your own
  `IntoIter<T>` type
- a borrowing `iter()` and `IntoIterator for &Stack<T>` (`for x in &stack`),
  also top first, via your own `Iter<'a, T>` type — the stack is unchanged

## Restrictions

- Store the elements in a `Vec<T>`, using only its end: no `VecDeque`,
  `LinkedList` or other collection, and no `insert(0, ..)` / `remove(0)`.
- Safe Rust only: no `unsafe`.

## Signature

```rust
pub struct Stack<T> { /* your fields */ }

impl<T> Stack<T> {
    pub fn new() -> Self;
    pub fn push(&mut self, value: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn peek(&self) -> Option<&T>;
    pub fn peek_mut(&mut self) -> Option<&mut T>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn iter(&self) -> Iter<'_, T>;
}

impl<T> Default for Stack<T> { /* … */ }
impl<T> FromIterator<T> for Stack<T> { /* … */ }
impl<T> IntoIterator for Stack<T> { type Item = T; type IntoIter = IntoIter<T>; /* … */ }

pub struct IntoIter<T> { /* your fields */ }
impl<T> Iterator for IntoIter<T> { type Item = T; /* … */ }

pub struct Iter<'a, T> { /* your fields */ }
impl<'a, T> Iterator for Iter<'a, T> { type Item = &'a T; /* … */ }
impl<'a, T> IntoIterator for &'a Stack<T> { type Item = &'a T; type IntoIter = Iter<'a, T>; /* … */ }
```

An empty stack is not an error: `pop`, `peek` and `peek_mut` return `None`.
`T` needs no bounds — not `Clone`, not `Default`.

## Examples

```rust
let mut s = Stack::new();
assert!(s.is_empty());
s.push(1);
s.push(2);
s.push(3);
assert_eq!(s.peek(), Some(&3));
assert_eq!(s.pop(), Some(3));
assert_eq!(s.len(), 2);

if let Some(top) = s.peek_mut() {
    *top *= 10;                               // [1, 20]
}
assert_eq!(s.pop(), Some(20));
assert_eq!(s.pop(), Some(1));
assert_eq!(s.pop(), None);

let s: Stack<char> = "abc".chars().collect(); // 'c' is on top
assert_eq!(s.iter().collect::<String>(), "cba");  // borrows: s is unchanged
assert_eq!(s.len(), 3);
assert_eq!(s.into_iter().collect::<String>(), "cba");
```

## Constraints

- Up to `10^6` operations; every operation is `O(1)` (amortized for `push`).

## Rust notes

- `Vec::last` / `Vec::last_mut` give `Option<&T>` / `Option<&mut T>` with no
  bounds check to write yourself.
- Clippy wants `Default` whenever there's an argument-free `new`.
- `impl<T> FromIterator<T>` has a generic method,
  `fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`.
- `IntoIter<T>` owns what it iterates, so it has no lifetime parameter —
  unlike the borrowing `Iter<'a, T>`, whose items are `&'a T`.
- `Iter` can simply wrap a std iterator: `self.items.iter().rev()` has type
  `std::iter::Rev<std::slice::Iter<'a, T>>`.
