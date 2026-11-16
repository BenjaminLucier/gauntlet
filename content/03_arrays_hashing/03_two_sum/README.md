## Description

Given a slice of integers `nums` and an integer `target`, find two
**different positions** `i < j` such that `nums[i] + nums[j] == target`, and
return them as `Some((i, j))`. If no such pair exists, return `None`.

The two positions must differ, though the values at them may be equal. When
several pairs work, any one of them is accepted.

## Signature

```rust
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)>
```

The slice is only read, so it is borrowed. Positions are `usize` because
that's what indexes a slice. A pair of positions is a tuple — always exactly
two, with `i < j` — and "no answer" is `None` rather than an empty `Vec` or
`-1`.

## Examples

```rust
assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));   // (0, 0) would reuse index 0
assert_eq!(two_sum(&[3, 3], 6), Some((0, 1)));
assert_eq!(two_sum(&[1, 2, 3], 7), None);            // no pair adds up to 7
assert_eq!(two_sum(&[], 0), None);
```

## Constraints

- `0 <= nums.len() <= 10^4`
- `-10^9 <= nums[i] <= 10^9`
- `-10^9 <= target <= 10^9`
- There may be zero, one or several valid pairs.

## Rust notes

- `target - nums[i]` can be as large as `2 * 10^9` in magnitude — outside the
  value range but still inside `i32` (max ≈ `2.147 * 10^9`). Check this kind of
  arithmetic every time; tests run with overflow checks on.
- `iter().enumerate()` gives you `(index, &value)` pairs, already `usize`.
- `HashMap::get` returns an `Option<&V>`; `if let Some(&j) = map.get(&key)`
  copies the index out of the map.
- Returning from inside a `for` loop and falling through to `None` is the
  natural shape here.
