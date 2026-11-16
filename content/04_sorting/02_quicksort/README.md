## Description

Implement **quicksort**, in place, with **3-way partitioning**.

`partition3(v, pivot_index)` rearranges `v` around the pivot value
`p = v[pivot_index]` into three blocks — everything `< p`, everything `== p`,
everything `> p` — and returns `(lt, gt)`, the bounds of the middle block:

```text
v[..lt]    < p
v[lt..gt]  == p     (never empty: it holds the pivot itself)
v[gt..]    > p
```

This is Dijkstra's "Dutch national flag" partition. `quicksort` then sorts
`v[..lt]` and `v[gt..]` and is done — the equal block is already in its final
place.

Your quicksort must survive the classic traps:

- **Sorted input**: always picking the first (or last) element as the pivot
  splits off one element per level — `O(n^2)` time and `O(n)` stack depth.
  Pick the pivot at random, or as a median of three.
- **Many equal keys**: a 2-way partition (`< p` vs `>= p`) on all-equal input
  makes no progress per level and goes quadratic. 3-way finishes it in one pass.
- **Deep recursion** (good practice; the tests don't check it): with a
  random pivot the stack is `O(log n)` deep on average. Recursing only into
  the smaller side and looping on the larger one makes that a guarantee.

## Restrictions

- No `sort`, `sort_unstable`, `sort_by*`, `select_nth_unstable`, and no
  `BinaryHeap` or `BTreeMap`.

## Signature

```rust
pub fn quicksort<T: Ord>(v: &mut [T]);
pub fn partition3<T: Ord>(v: &mut [T], pivot_index: usize) -> (usize, usize);
```

`partition3` panics if `pivot_index` is out of bounds (including on an empty
slice), like indexing does. Neither function needs `Clone`: compare against
the pivot where it sits, and move elements only by swapping.

## Examples

```rust
let mut v = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
quicksort(&mut v);
assert_eq!(v, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);

let mut w = [5, 2, 8, 5, 1, 5, 9];
let (lt, gt) = partition3(&mut w, 0); // pivot value 5
assert_eq!((lt, gt), (2, 5));
assert!(w[..lt].iter().all(|&x| x < 5));  // {1, 2} in some order
assert_eq!(w[lt..gt], [5, 5, 5]);
assert!(w[gt..].iter().all(|&x| x > 5));  // {8, 9} in some order

let mut words = vec!["kiwi", "apple", "fig"];
quicksort(&mut words);
assert_eq!(words, ["apple", "fig", "kiwi"]);
```

## Constraints

- `0 <= v.len() <= 10^6`; values may all be equal.
- `quicksort` must make `O(n log n)` comparisons on sorted, reversed and random
  input, and `O(n)` on all-equal input.
- `partition3` makes at most one comparison per element (the tests count).

## Rust notes

- Inside `partition3` you can keep the pivot *in the slice* at index `lt` and
  compare with `v[i].cmp(&v[lt])` — two shared borrows of `v` are fine; then
  `swap` after the comparison ends. A `match` on `Ordering` reads well here.
- `let (left, rest) = v.split_at_mut(lt);` then `&mut rest[gt - lt..]` gives
  two disjoint `&mut` parts.
- Reassigning `v = right_part` inside a loop over `v` needs the reference
  moved out first: `std::mem::take(&mut v)` leaves an empty slice behind
  (`&mut [T]` implements `Default`).
- `usize` underflow panics: watch `gt - 1` when `gt` reaches the boundary.
