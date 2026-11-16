## Description

Given a slice of items and a position `k` (1-based), return the `k`-th largest
item, or `None` if there is no such item (`k == 0` or `k > items.len()`).
"k-th largest" is by sorted position, not by distinct value: in `[3, 3, 1]`
the 2nd largest is `3`.

Try to do it without fully sorting.

## Signature

```rust
pub fn kth_largest<T: Ord + Copy>(items: &[T], k: usize) -> Option<T>
```

The algorithm only compares items, so it's generic over `T: Ord`. The input is
borrowed — finding the k-th largest shouldn't scramble the caller's data — and
the answer is returned by value, hence `Copy` (integers, `char`s, `&str`s,
small tuples all qualify). An out-of-range `k` is `None`, not a panic.

## Examples

```rust
assert_eq!(kth_largest(&[3, 2, 1, 5, 6, 4], 2), Some(5));
// sorted descending: 6, 5, 5, 4, ... -> the 4th is 4
assert_eq!(kth_largest(&[3, 2, 3, 1, 2, 4, 5, 5, 6], 4), Some(4));
assert_eq!(kth_largest(&[3, 3, 1], 2), Some(3));      // duplicates count
assert_eq!(kth_largest(&["b", "c", "a"], 1), Some("c"));
assert_eq!(kth_largest(&[1, 2], 3), None);           // only two items
assert_eq!(kth_largest(&[1, 2], 0), None);           // positions start at 1
```

## Constraints

- `0 <= items.len() <= 10^5`
- `k` is any `usize`

## Rust notes

- `slice::select_nth_unstable(i)` is quickselect in std: afterwards the item
  at index `i` is the one a full sort would put there, in `O(n)`. It needs
  `&mut [T]` — work on `items.to_vec()`. Writing the quickselect yourself is
  good practice; the std one is what you'd use in real code.
- The k-th largest is index `n - k` in ascending order, or index `k - 1` with
  `select_nth_unstable_by(k - 1, |a, b| b.cmp(a))`. Check the range first:
  `n - k` underflows a `usize` when `k > n`.
- For the heap approach: `BinaryHeap` is a max-heap; `BinaryHeap<Reverse<T>>`
  of size `k` keeps the `k` largest, with the answer on top.
- `Option::filter` / `bool::then` turn a range check into an `Option` neatly.
