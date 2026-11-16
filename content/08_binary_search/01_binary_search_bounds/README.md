## Description

Build the binary-search family on a slice sorted in ascending order (duplicates
allowed). All of them reduce to one primitive:

- `partition_point(items, pred)`: the slice is *partitioned* by `pred` — every
  item for which `pred` is `true` comes before every item for which it is
  `false`. Return the index of the first `false` item (`items.len()` if there
  is none).
- `lower_bound(items, target)`: the first index `i` with `items[i] >= target`.
- `upper_bound(items, target)`: the first index `i` with `items[i] > target`.
- `equal_range(items, target)`: the range of indices holding items equal to
  `target` — `lower_bound..upper_bound`. Empty (but positioned where `target`
  would be inserted) when it is absent.
- `binary_search(items, target)`: `Some(index)` of the **first** occurrence of
  `target`, or `None`.

Both bounds return a value in `0..=items.len()`: it is also the position where
`target` can be inserted while keeping the slice sorted.

## Restrictions

- **No `slice::binary_search`, `binary_search_by`, `binary_search_by_key` or
  `partition_point`** — they are exactly what you are building. No sorting or
  linear scans either.

## Signature

```rust
pub fn partition_point<T>(items: &[T], pred: impl FnMut(&T) -> bool) -> usize;
pub fn lower_bound<T: Ord>(items: &[T], target: &T) -> usize;
pub fn upper_bound<T: Ord>(items: &[T], target: &T) -> usize;
pub fn equal_range<T: Ord>(items: &[T], target: &T) -> Range<usize>;
pub fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize>;
```

Everything only compares items, so `T: Ord` is all it needs — and
`partition_point` needs nothing at all, since the predicate does the comparing.
Items are borrowed, never cloned. Indices are `usize`, "not found" is `None`,
and a run of indices is a `std::ops::Range<usize>`, which slices directly:
`&items[equal_range(items, &x)]`.

## Examples

```rust
let v = [1, 2, 2, 2, 5, 8];
assert_eq!(lower_bound(&v, &2), 1);
assert_eq!(upper_bound(&v, &2), 4);
assert_eq!(equal_range(&v, &2), 1..4);
assert_eq!(equal_range(&v, &3), 4..4);        // absent: empty, at its insertion point
assert_eq!(lower_bound(&v, &0), 0);           // below everything
assert_eq!(upper_bound(&v, &9), 6);           // above everything: len
assert_eq!(binary_search(&v, &2), Some(1));   // the first of the 2s
assert_eq!(binary_search(&v, &7), None);
assert_eq!(partition_point(&v, |&x| x < 5), 4);
assert_eq!(binary_search(&["ant", "bee", "cat"], &"cat"), Some(2));
```

## Constraints

- `0 <= items.len() <= 10^6`
- `items` is sorted ascending by `Ord` (for `partition_point`: partitioned by
  `pred`). Duplicates are allowed.
- `O(log n)` comparisons per call — and `O(log n)` predicate calls for
  `partition_point` itself (at most `⌊log2 n⌋ + 1`).

## Rust notes

- With `usize` indices, a closed window `[lo, hi]` and `hi = mid - 1` underflows
  (and panics) when `mid == 0`. A half-open window `[lo, hi)` with `hi = mid`
  never goes below zero.
- Compute `mid = lo + (hi - lo) / 2`; `(lo + hi) / 2` is the classic overflow bug.
  With `lo < hi`, `mid` is always a valid index and always `< hi`.
- `impl FnMut(&T) -> bool` in argument position is a generic parameter in
  disguise: a closure such as `|x| x < target` is monomorphized, with no boxing.
  `FnMut` (as in std's `partition_point`) lets callers pass closures that keep
  state, such as a counter; calling it needs the parameter declared `mut pred`.
- `Ord::cmp` returns an `Ordering`; `x < target` on `&T`s goes through
  `PartialOrd`, which `Ord` implies.
