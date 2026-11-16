## Description

Given a slice of integers `nums`, return every triplet of values
`[nums[i], nums[j], nums[k]]` with `i`, `j`, `k` pairwise distinct and
`nums[i] + nums[j] + nums[k] == 0`.

The result must not contain **duplicate triplets**: two triplets with the same
values count as the same triplet, whatever positions they came from. Return
each triplet with its values in **non-decreasing order**; the triplets
themselves may come in any order.

## Signature

```rust
pub fn three_sum(nums: &[i32]) -> Vec<[i32; 3]>
```

A triplet always has exactly three values, so it's an array `[i32; 3]` — not a
`Vec<i32>` that could have any length. The input is borrowed: if you want to
sort, sort your own copy (`nums.to_vec()`), so the caller's data is untouched.

## Examples

```rust
let mut found = three_sum(&[-1, 0, 1, 2, -1, -4]);
found.sort_unstable();                     // triplets may come in any order
assert_eq!(found, vec![[-1, -1, 2], [-1, 0, 1]]);
// [0, 1, -1] (positions 1, 2, 4) is the same triplet as [-1, 0, 1].

assert!(three_sum(&[0, 1, 1]).is_empty());
assert_eq!(three_sum(&[0, 0, 0, 0]), vec![[0, 0, 0]]);   // reported once
assert!(three_sum(&[1, -1]).is_empty());                // fewer than three values
```

## Constraints

- `0 <= nums.len() <= 3000`
- `-10^5 <= nums[i] <= 10^5`

## Rust notes

- `sort_unstable` is faster than `sort` and fine here: equal integers are
  indistinguishable.
- `std::cmp::Ordering` and `match sum.cmp(&0)` keep the pointer moves tidy.
- `n - 1` or `n - 2` on a short slice underflows a `usize`; ranges like
  `i + 1..n` are naturally empty instead.
- Three values of magnitude `10^5` sum to at most `3 * 10^5` — no overflow
  in `i32`.
