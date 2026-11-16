## Description

Given a slice of integers `nums`, return a vector `answer` of the same length
where `answer[i]` is the product of every element of `nums` **except**
`nums[i]`.

Solve it in `O(n)` time **without using division**. A product of no numbers
is `1`, so a one-element input gives `[1]` and an empty input gives `[]`.

Every prefix product and every suffix product of `nums` is guaranteed to fit
in an `i32` — and therefore so does every answer.

## Signature

```rust
pub fn product_except_self(nums: &[i32]) -> Vec<i32>
```

The input is only read, so it's borrowed; the result is a new `Vec` of the
same length. The values stay `i32`: the guarantee above means no intermediate
product ever needs a wider type.

## Examples

```rust
assert_eq!(product_except_self(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
assert_eq!(product_except_self(&[-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
assert_eq!(product_except_self(&[0, 2, 0]), vec![0, 0, 0]);   // two zeros
assert_eq!(product_except_self(&[7]), vec![1]);                // empty product
assert_eq!(product_except_self(&[]), Vec::<i32>::new());
```

## Constraints

- `0 <= nums.len() <= 10^5`
- `-30 <= nums[i] <= 30`
- Every prefix/suffix product of `nums` fits in an `i32`.

## Rust notes

- Tests run with overflow checks on, so only multiply what the guarantee
  covers: a prefix or suffix product, never "everything times everything".
- `vec![1; n]` allocates the output pre-filled with the multiplicative
  identity.
- `iter_mut().zip(...)` and `.rev()` let you sweep both directions without
  index arithmetic; plain index loops are fine too if they read better to you.
- Iterator `scan` is another way to produce running products.
