## Description

Build `MaxHeap<T>`, a binary max-heap — your own `std::collections::BinaryHeap`
— stored in a `Vec<T>` laid out as a complete binary tree.

Implement:

- `new`, `len`, `is_empty`
- `push` — add an item; `pop` — remove and return the largest; `peek` — borrow
  the largest
- `iter` — borrow every item in the heap's internal array order (level by
  level: root first, but otherwise **not** sorted), via your own `Iter<'_, T>`;
  plus `IntoIterator for &MaxHeap<T>`
- `into_vec` — consume the heap and hand back its storage, in that same order,
  without copying
- `From<Vec<T>>` — turn a vector into a heap in `O(n)` with a bottom-up
  heapify, not `n` pushes
- `FromIterator<T>` — so `.collect::<MaxHeap<_>>()` works
- `Extend<T>` — push every item of an iterator
- `Default`

Then notice that you get a min-heap for free: `std::cmp::Reverse<T>` flips
`T`'s ordering, so `MaxHeap<Reverse<i32>>` pops the *smallest* number first.

## Restrictions

- **No `std::collections::BinaryHeap`**, and no `sort` / `sort_unstable` (or any
  other sort) — the heap does the ordering.
- Storage is a single `Vec<T>`; no `Clone` bound on `T`.

## Signature

```rust
pub struct MaxHeap<T> { /* your fields */ }

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn push(&mut self, item: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn peek(&self) -> Option<&T>;
    pub fn iter(&self) -> Iter<'_, T>;
    pub fn into_vec(self) -> Vec<T>;
}

impl<T: Ord> From<Vec<T>> for MaxHeap<T>;
impl<T: Ord> FromIterator<T> for MaxHeap<T>;
impl<T: Ord> Extend<T> for MaxHeap<T>;
impl<T: Ord> Default for MaxHeap<T>;

pub struct Iter<'a, T> { /* your fields */ }
impl<'a, T> Iterator for Iter<'a, T> { type Item = &'a T; /* … */ }
impl<'a, T: Ord> IntoIterator for &'a MaxHeap<T> { type Item = &'a T; type IntoIter = Iter<'a, T>; /* … */ }
```

`pop` and `peek` return `Option` because the heap may be empty. `peek` borrows:
the item stays in the heap. `From<Vec<T>>` takes the vector by value and reuses
its buffer — no copying — and `into_vec` gives that buffer back. Sorting a
heap's contents (heapsort) is a separate exercise; here `iter` and `into_vec`
just expose the array as it is.

## Examples

```rust
let mut h = MaxHeap::new();
h.push(3);
h.push(10);
h.push(7);
assert_eq!(h.peek(), Some(&10));
assert_eq!(h.pop(), Some(10));
assert_eq!(h.pop(), Some(7));
assert_eq!(h.len(), 1);

let mut h = MaxHeap::from(vec![4, 1, 8, 1, 5]);
assert_eq!(h.peek(), Some(&8));
assert_eq!(h.iter().count(), 5);                // every item, in array order
h.extend([9, 2]);
assert_eq!(h.pop(), Some(9));
let v = h.into_vec();                           // the heap's array, as is
assert_eq!(v.len(), 6);
assert_eq!(v[0], 8);                            // the root is the max

use std::cmp::Reverse;
let mut min: MaxHeap<Reverse<i32>> = [5, -2, 9].into_iter().map(Reverse).collect();
assert_eq!(min.pop(), Some(Reverse(-2)));   // smallest first
```

## Constraints

- Up to `10^6` items; any order, duplicates allowed.
- `From<Vec<T>>` must make `O(n)` comparisons (at most `2n`), not
  `O(n log n)`.

## Rust notes

- `Vec::swap(i, j)` moves items without `Clone`; `Vec::pop` takes the last one.
  `Vec::swap_remove(0)` is "swap the root with the last item and pop" in one call.
- With `usize` indices, `(i - 1) / 2` underflows at the root: only compute a
  parent for `i > 0`.
- `impl<T: Ord> FromIterator<T>` can collect into a `Vec` and reuse `From`.
- `Iter<'a, T>` can simply wrap a `std::slice::Iter<'a, T>`.
- `Reverse` is `Ord` whenever `T` is, so `MaxHeap<Reverse<T>>` needs no extra
  code at all — that's how std's `BinaryHeap` is used as a min-heap.
