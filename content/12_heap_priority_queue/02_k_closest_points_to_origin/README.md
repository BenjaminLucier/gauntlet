## Description

Given a list of points `(x, y)` on the plane and a count `k`, return the `k`
points closest to the origin `(0, 0)` by Euclidean distance
`sqrt(x^2 + y^2)`.

The answer may be returned in **any order**. If several points tie for the
last place, any of them may be chosen. If `k` is at least the number of
points, return all of them; if `k == 0`, return nothing. Duplicate points are
separate points.

## Signature

```rust
pub fn k_closest(points: &[(i32, i32)], k: usize) -> Vec<(i32, i32)>
```

A point is a plain `(i32, i32)` tuple — `Copy`, comparable, and far lighter
than a two-element `Vec`. The input is borrowed; the result is a fresh `Vec`
of copies. `k` is a count, so it's a `usize`.

## Examples

```rust
// dist²(1,3) = 10, dist²(-2,2) = 8.
assert_eq!(k_closest(&[(1, 3), (-2, 2)], 1), vec![(-2, 2)]);

let mut two = k_closest(&[(3, 3), (5, -1), (-2, 4)], 2);
two.sort();                                     // any order is accepted
assert_eq!(two, vec![(-2, 4), (3, 3)]);

assert_eq!(k_closest(&[(1, 1)], 5), vec![(1, 1)]);   // k larger than n
assert!(k_closest(&[(1, 1)], 0).is_empty());
```

## Constraints

- `0 <= points.len() <= 10^4`
- `0 <= k`
- `x` and `y` may be any `i32`. A squared distance can reach `2^63`, which
  overflows even `i64`: compute it in `u64`.

## Rust notes

- `BinaryHeap` is a max-heap — exactly right for "keep the `k` smallest,
  evict the largest". Push `(dist, point)` tuples: tuples are `Ord`
  lexicographically, so the distance decides.
- `slice::select_nth_unstable_by_key(i, f)` reorders a slice so everything
  before index `i` is `<=` it by key, in `O(n)` average — a quickselect in std.
  It needs `&mut`, so work on `points.to_vec()`.
- `i32 * i32` overflows for large coordinates, and so does `i64` for
  `(i32::MIN, i32::MIN)`. `x.unsigned_abs()` gives a `u32`; widen it with
  `u64::from` before squaring.
- Don't take a square root — comparing squared distances gives the same order
  and stays exact.
