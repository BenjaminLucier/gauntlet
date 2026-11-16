## Description

Build `DynArray<T>`, a growable array — your own `Vec<T>` — on top of a
fixed-size boxed slice of slots.

Elements live in a heap buffer of `capacity` slots, of which the first `len`
are in use. Pushing onto a full buffer allocates a larger one and moves every
element over. How much larger is the whole lesson: grow by a constant factor
and `push` is amortized `O(1)`.

Implement:

- `new`, `with_capacity`, `len`, `is_empty`, `capacity`
- `push`, `pop`, `get`, `get_mut`, `insert`, `remove`, `clear`
- `iter`, returning your own `Iter<'_, T>` that implements `Iterator`
- the traits a Rust user expects of a collection: `Default`, `Index` /
  `IndexMut` (panicking out of bounds, like `Vec`), `FromIterator`, `Extend`,
  `IntoIterator for &DynArray<T>` (so `for x in &arr` works), and an owning
  `IntoIterator for DynArray<T>` (so `for x in arr` moves the elements out)
  via your own `IntoIter<T>`

## Restrictions

- **No `Vec`, `VecDeque` or other std collection for storage.** The buffer is
  a `Box<[Option<T>]>`. (Collecting an iterator straight into a
  `Box<[Option<T>]>` is fine — that's how you allocate one.)
- Safe Rust only: no `unsafe`.

## Signature

```rust
pub struct DynArray<T> { /* your fields */ }

impl<T> DynArray<T> {
    pub fn new() -> Self;
    pub fn with_capacity(capacity: usize) -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn capacity(&self) -> usize;
    pub fn push(&mut self, value: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn get(&self, index: usize) -> Option<&T>;
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), T>;
    pub fn remove(&mut self, index: usize) -> Option<T>;
    pub fn clear(&mut self);
    pub fn iter(&self) -> Iter<'_, T>;
}

pub struct Iter<'a, T> { /* your fields */ }
impl<'a, T> Iterator for Iter<'a, T> { type Item = &'a T; /* … */ }

pub struct IntoIter<T> { /* your fields */ }
impl<T> Iterator for IntoIter<T> { type Item = T; /* … */ }

// Plus: Default, Index<usize>, IndexMut<usize>, FromIterator<T>, Extend<T>,
// IntoIterator for DynArray<T> and for &DynArray<T>.
```

`get`, `get_mut` and `remove` return `Option` where `Vec` would panic. `insert`
at `index > len` hands the value back as `Err(value)` instead of dropping it.
`T` needs no bounds at all: not `Clone`, not `Default`.

## Examples

```rust
let mut a = DynArray::new();
assert!(a.is_empty());
a.push(10);
a.push(20);
a.push(30);
assert_eq!(a.len(), 3);
assert_eq!(a[1], 20);
assert_eq!(a.get(3), None);
assert!(a.capacity() >= 3);

a.insert(0, 5).unwrap();                     // [5, 10, 20, 30]
assert_eq!(a.insert(9, 99), Err(99));        // out of range: value comes back
assert_eq!(a.remove(2), Some(20));           // [5, 10, 30]
assert_eq!(a.pop(), Some(30));               // [5, 10]
a[1] += 1;                                   // [5, 11]
assert_eq!(a.iter().copied().collect::<Vec<_>>(), [5, 11]);

let b: DynArray<String> = ["x", "y"].into_iter().map(String::from).collect();
assert_eq!(b.len(), 2);
let owned: Vec<String> = b.into_iter().collect();  // moves the Strings out
assert_eq!(owned, ["x", "y"]);
```

## Constraints

- Up to `10^6` operations; `push` must not degrade to `O(n)` per call.
- Growth is by a constant factor, and not a wasteful one: once the array has
  grown past its initial capacity, `capacity() < 4 * len()`.
- Every element is dropped exactly once (by `pop`, `remove`, `clear`, an owning
  iterator, or when the array or a half-used iterator is dropped).

## Rust notes

- `(0..n).map(|_| None).collect::<Box<[Option<T>]>>()` allocates `n` empty
  slots without needing `T: Clone` (which `vec![None; n]` would).
- `std::mem::replace` swaps a whole buffer out of `&mut self`, so you can move
  the old elements into the new one.
- `slice::rotate_right` / `rotate_left` on a sub-slice shift a run of slots by
  one — handy for `insert` and `remove`.
- The iterator borrows the array: `Iter<'a, T>` can hold a `&'a [Option<T>]`
  and hand out `&'a T`s from it.
- `impl<'a, T> IntoIterator for &'a DynArray<T>` is three lines once `iter`
  exists, and it's what makes `for x in &arr` compile.
- The owning `IntoIter<T>` can simply own the whole `DynArray<T>` plus a
  cursor, and `take` each slot as it goes; slots it never reaches are dropped
  with the buffer.
