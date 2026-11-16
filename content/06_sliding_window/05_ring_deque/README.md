## Description

Build `RingDeque<T>`, a double-ended queue — your own `VecDeque<T>` — on a
circular buffer.

The elements sit in a fixed-size boxed slice of slots. Instead of shifting
elements around, the deque remembers where its front is (`head`) and how many
elements it holds (`len`); the occupied region may run off the end of the
buffer and continue from slot 0. Pushing or popping at either end only moves
`head` or changes `len`, so every operation is `O(1)`. When the buffer is
full, allocate one twice the size and move everything across.

Implement:

- `new`, `with_capacity`, `len`, `is_empty`, `capacity`, and `Default`
- `push_front`, `push_back`, `pop_front`, `pop_back`
- `front`, `back`, `get(i)` — `i` counts from the front, `0..len`
- `iter`, returning your own `Iter<'_, T>` that walks front to back — and,
  as a `DoubleEndedIterator`, back to front with `.rev()`
- the collection traits: `IntoIterator for &RingDeque<T>` (`for x in &d`),
  `Index<usize>` (`d[i]`, panicking out of bounds), `Extend` (pushes onto the
  back) and `FromIterator` (`collect()` keeps the order)

## Restrictions

- **No `VecDeque`, `Vec` or other std collection for storage.** The buffer is
  a `Box<[Option<T>]>`. (Collecting an iterator straight into a
  `Box<[Option<T>]>` is fine — that's how you allocate one.)
- Safe Rust only: no `unsafe`.

## Signature

```rust
pub struct RingDeque<T> { /* your fields */ }

impl<T> RingDeque<T> {
    pub fn new() -> Self;
    pub fn with_capacity(capacity: usize) -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn capacity(&self) -> usize;
    pub fn push_front(&mut self, value: T);
    pub fn push_back(&mut self, value: T);
    pub fn pop_front(&mut self) -> Option<T>;
    pub fn pop_back(&mut self) -> Option<T>;
    pub fn front(&self) -> Option<&T>;
    pub fn back(&self) -> Option<&T>;
    pub fn get(&self, index: usize) -> Option<&T>;
    pub fn iter(&self) -> Iter<'_, T>;
}

pub struct Iter<'a, T> { /* your fields */ }
impl<'a, T> Iterator for Iter<'a, T> { type Item = &'a T; /* … */ }
impl<T> DoubleEndedIterator for Iter<'_, T> { /* … */ }

impl<'a, T> IntoIterator for &'a RingDeque<T> { type Item = &'a T; type IntoIter = Iter<'a, T>; /* … */ }
impl<T> Index<usize> for RingDeque<T> { type Output = T; /* … */ }
impl<T> Extend<T> for RingDeque<T> { /* … */ }
impl<T> FromIterator<T> for RingDeque<T> { /* … */ }
```

`capacity` is the number of slots in the buffer. `with_capacity(n)` allocates
**exactly** `n` slots and must not reallocate until more than `n` elements are
stored; `with_capacity(0)` and `new` allocate nothing at all. `T` needs no
bounds.

## Examples

```rust
let mut d = RingDeque::with_capacity(4);
d.push_back(2);
d.push_back(3);
d.push_front(1);                         // [1, 2, 3]
assert_eq!(d.front(), Some(&1));
assert_eq!(d.back(), Some(&3));
assert_eq!(d.get(1), Some(&2));
assert_eq!(d.get(3), None);

assert_eq!(d.pop_front(), Some(1));      // [2, 3]
d.push_back(4);
d.push_back(5);                          // [2, 3, 4, 5]: all 4 slots used
assert_eq!(d.capacity(), 4);
d.push_front(0);                         // full: grows to 8 slots
assert!(d.capacity() >= 5);
assert_eq!(d.iter().copied().collect::<Vec<_>>(), [0, 2, 3, 4, 5]);
assert_eq!(d.iter().rev().copied().collect::<Vec<_>>(), [5, 4, 3, 2, 0]);
assert_eq!(d[1], 2);
assert_eq!(d.pop_back(), Some(5));

let mut e: RingDeque<char> = "bc".chars().collect();
e.extend(['d', 'e']);
e.push_front('a');
assert_eq!(e.iter().collect::<String>(), "abcde");
```

## Constraints

- Up to `10^6` operations. Aim for amortized `O(1)` per push, at either end.
- Growth is by a constant factor, and not a wasteful one: once the deque has
  grown past its initial capacity, `capacity() < 4 * len()`.
- Every element is dropped exactly once.

## Rust notes

- `(0..n).map(|_| None).collect::<Box<[Option<T>]>>()` allocates `n` empty
  slots without `T: Clone`.
- `Option::take` moves a value out of a slot and leaves `None`;
  `std::mem::replace` swaps out the whole buffer when you grow.
- `usize` subtraction that goes below zero panics (overflow checks are on).
- `Iter<'a, T>` can hold a `&'a [Option<T>]` (or two of them, one per side of
  the wrap) plus whatever indices it needs. `next` and `next_back` consume
  from opposite ends of the same remaining range, so they can never yield an
  element twice.
