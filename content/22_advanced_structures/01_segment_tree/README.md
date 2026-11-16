## Description

Build a segment tree: a structure over a fixed-length array that answers
"combine everything in `start..end`" and "replace element `i`" in `O(log n)`
each, after an `O(n)` build.

What "combine" means is a type parameter. A `Monoid` is a type-level choice of
an associative operation and its identity element: `Sum` (with `0`) gives range
sums, `Min` (with `i64::MAX`) gives range minimums, and the tests bring their
own. Implement `Sum` and `Min`, then `SegmentTree<M: Monoid>`:

- `new(values)` — build in `O(n)`
- `len`, `is_empty`
- `set(index, value)` — replace one element
- `query(range)` — combine `values[range]` left to right; an empty range gives
  `M::identity()`

`combine` is associative but **not necessarily commutative**: `query(0..3)`
must be `combine(combine(a0, a1), a2)` (or any regrouping of it), never
`combine(a1, a0)`.

## Restrictions

- Every operation except `new` must be `O(log n)`: no scanning the range.

## Signature

```rust
pub trait Monoid {
    type Item: Copy;
    fn identity() -> Self::Item;
    fn combine(a: Self::Item, b: Self::Item) -> Self::Item;
}

pub struct Sum; // Item = i64
pub struct Min; // Item = i64

pub struct SegmentTree<M: Monoid> { /* your fields */ }

impl<M: Monoid> SegmentTree<M> {
    pub fn new(values: &[M::Item]) -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn set(&mut self, index: usize, value: M::Item);
    pub fn query(&self, range: Range<usize>) -> M::Item;
}
```

`Sum` and `Min` are empty marker types: the monoid is chosen by the type, as
in `SegmentTree::<Min>::new(&v)`, and the tree stores plain `M::Item`s. The
trait has no `self` — there is nothing to store. `query` takes a half-open
`Range<usize>` like slicing does, and panics like slicing does when the range
is reversed or runs past the end, with a message containing `out of bounds`
(so does `set` with a bad index).

## Examples

```rust
let mut sums = SegmentTree::<Sum>::new(&[5, 3, -2, 8, 1]);
assert_eq!(sums.len(), 5);
assert_eq!(sums.query(0..5), 15);
assert_eq!(sums.query(1..4), 9);           // 3 + -2 + 8
assert_eq!(sums.query(2..2), 0);           // empty range: the identity
sums.set(2, 10);                           // replace, not add
assert_eq!(sums.query(1..4), 21);

let mins = SegmentTree::<Min>::new(&[5, 3, -2, 8, 1]);
assert_eq!(mins.query(0..2), 3);
assert_eq!(mins.query(3..5), 1);
assert_eq!(mins.query(4..4), i64::MAX);
```

## Constraints

- `0 <= n <= 2 * 10^5`, up to `2 * 10^5` operations
- For `Sum`, every value and every range sum fits in `i64` in the tests.

## Rust notes

- Associated types: inside `impl<M: Monoid>`, element values are `M::Item`,
  and you call the operations as `M::combine(a, b)` and `M::identity()`.
- `type Item: Copy` lets you read `tree[i]` by value — no `clone()` noise —
  and `vec![M::identity(); n]` fills a `Vec` with it.
- A struct field of type `M` isn't needed: the tree stores `Vec<M::Item>`,
  which already mentions `M`.
- Destructure a range with `let Range { start, end } = range;` (or use
  `range.start` / `range.end`).
- The bottom-up layout is index arithmetic: `i / 2` is the parent, `i % 2 == 1`
  (or `i & 1`) says "right child".
