## Description

Given a slice of integers `nums`, find a non-empty **contiguous** subarray whose
elements have the largest product, and return that product. An empty slice has
no non-empty subarray, so the answer is `None`.

## Signature

```rust
pub fn max_product(nums: &[i32]) -> Option<i64>
```

The values are small `i32`s, but products of many of them are not: the result
is widened to `i64`, and the input guarantees that every subarray's product
fits in an `i64`. `Option` covers the empty slice instead of panicking or
inventing a value.

## Examples

```rust
assert_eq!(max_product(&[2, 3, -2, 4]), Some(6)); // [2, 3]
assert_eq!(max_product(&[-2, 0, -1]), Some(0));   // [-2, -1] isn't contiguous
assert_eq!(max_product(&[-2, 3, -4]), Some(24));  // two negatives cancel out
assert_eq!(max_product(&[-5]), Some(-5));
assert_eq!(max_product(&[]), None);
assert_eq!(max_product(&[10; 18]), Some(1_000_000_000_000_000_000)); // > i32::MAX
```

## Constraints

- `0 <= nums.len() <= 2 * 10^4`
- `-10 <= nums[i] <= 10`
- The product of any subarray of `nums` fits in an `i64`.

## Rust notes

- `let (&first, rest) = nums.split_first()?;` handles the empty case and
  seeds the running state in one line — `?` works on `Option` too.
- Convert with `i64::from(x)` **before** multiplying; `x * y` in `i32` would
  overflow (and panic under overflow checks) long before the `i64` limit.
- A `fold` carrying `(largest_ending_here, smallest_ending_here, best)` keeps
  all state in one place; `max`/`min` chain naturally: `x.max(a).max(b)`.
- Only multiply values that are products of real subarrays — anything else
  (like a sentinel `i64::MIN`) can overflow.
