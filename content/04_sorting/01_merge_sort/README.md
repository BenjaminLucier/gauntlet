## Description

Implement top-down **merge sort**: split the slice in half, sort each half
recursively, and merge the two sorted halves into one.

Merge sort is the classic `O(n log n)` **stable** sort: elements that compare
equal stay in the order they started in, which is what you want when sorting
records by one field after another. Its cost is an `O(n)` scratch buffer.

Write `merge_sort` for `T: Ord`, and `merge_sort_by`, which orders by a
comparator. Allocate the scratch buffer once, not once per merge.

## Restrictions

- No `sort`, `sort_unstable`, `sort_by*`, `select_nth_unstable`, and no
  `BinaryHeap` or `BTreeMap`.

## Signature

```rust
pub fn merge_sort<T: Ord + Clone>(v: &mut [T]);
pub fn merge_sort_by<T: Clone, F>(v: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering;
```

**Why `Clone`?** Merging needs room to hold one half while the other is
written over it. In safe Rust a slot can't be left "moved-out" — every slot of
a `Vec<T>` or `&mut [T]` must always hold a valid `T` — so the buffer must be
filled with *something* when it is created. The simplest honest choice is
`v[..n / 2].to_vec()`: one clone of half the input, up front. After that,
elements only travel by swapping (`mem::swap`, `swap_with_slice`), so no
further clones are needed. (std's `slice::sort` avoids `Clone` by using
`unsafe` raw-pointer copies; sorting a vector of indices and then permuting is
the other safe alternative.)

## Examples

```rust
let mut v = vec![38, 27, 43, 3, 9, 82, 10];
merge_sort(&mut v);
assert_eq!(v, [3, 9, 10, 27, 38, 43, 82]);

// Stable: sort by age only, names with equal ages keep their order.
let mut people = vec![("ann", 31), ("bob", 25), ("cat", 31), ("dan", 25)];
merge_sort_by(&mut people, |a, b| a.1.cmp(&b.1));
assert_eq!(people, [("bob", 25), ("dan", 25), ("ann", 31), ("cat", 31)]);

// Any comparator: descending order of string length.
let mut words = vec!["a", "ccc", "bb"];
merge_sort_by(&mut words, |a, b| b.len().cmp(&a.len()));
assert_eq!(words, ["ccc", "bb", "a"]);
```

## Constraints

- `0 <= v.len() <= 10^6`.
- `merge_sort_by` must make at most `n * ceil(log2 n)` calls to `compare`.
- A whole sort may clone at most `n` elements in total (the tests count):
  one scratch buffer, not one per merge.

## Rust notes

- `let (left, right) = v.split_at_mut(mid);` gives two non-overlapping
  `&mut` halves the borrow checker is happy with.
- Pass the comparator down the recursion as `&mut F` (an `&mut F` is itself
  `FnMut`), rather than requiring `F: Copy`.
- `merge_sort` can just call `merge_sort_by(v, T::cmp)`.
- A cheap shortcut: if the last element of the left half is not greater than
  the first of the right half, the halves are already in order — skip the
  merge. That makes sorted input `O(n)` comparisons.
