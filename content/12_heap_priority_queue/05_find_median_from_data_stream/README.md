## Description

The median of a list of numbers is the middle value once sorted; for an even
count it is the mean of the two middle values (`[2, 3]` has median `2.5`).

Design a structure that accepts integers one at a time and can report the
median of everything added so far.

- `MedianFinder::new()` creates an empty finder (`MedianFinder::default()`
  too).
- `add(num)` adds `num`.
- `median()` returns the median of all numbers added so far, or `None` if
  nothing has been added yet. Answers within `10^-5` of the true value are
  accepted.

## Signature

```rust
#[derive(Default)]
pub struct MedianFinder { /* your fields */ }

impl MedianFinder {
    pub fn new() -> Self
    pub fn add(&mut self, num: i32)
    pub fn median(&self) -> Option<f64>
}
```

An empty stream has no median, so `median` returns `Option<f64>` instead of
assuming at least one number was added. It only reads, so it takes `&self`.
The result is `f64` because the mean of two integers can be a half.

## Examples

```rust
let mut mf = MedianFinder::new();
assert_eq!(mf.median(), None);        // nothing added yet
mf.add(1);
mf.add(2);
assert_eq!(mf.median(), Some(1.5));   // (1 + 2) / 2
mf.add(3);
assert_eq!(mf.median(), Some(2.0));   // [1, 2, 3] -> 2
```

## Constraints

- `num` may be any `i32` — including `i32::MIN` and `i32::MAX`.
- At most `5 * 10^4` calls in total.

## Rust notes

- `BinaryHeap` is a max-heap. A min-heap is `BinaryHeap<Reverse<i32>>`;
  destructure with `let Reverse(x) = …` or read `.0`.
- `peek` returns `Option<&T>`, so `median` can be built from `?` and
  `Option` combinators instead of `unwrap`s.
- Two `i32`s near the extremes overflow when added (a panic under overflow
  checks). Convert to `f64` *before* adding — `f64::from(a)` is lossless for
  every `i32`.
- With `#[derive(Default)]` on the struct, `new()` is just `Self::default()`.
