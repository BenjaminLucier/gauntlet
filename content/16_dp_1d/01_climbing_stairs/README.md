## Description

You are at the bottom of a staircase with `n` steps. Each move takes you up
either **1** or **2** steps. Return the number of distinct sequences of moves
that land you exactly on the top step.

With `n = 0` you are already at the top: the empty sequence is the one way.

## Signature

```rust
pub fn climb_stairs(n: u32) -> u64
```

A number of steps can't be negative, so `n` is unsigned. The count grows like
the Fibonacci numbers — exponentially — so it is returned as a `u64`, which
holds the answer for every `n` up to 92 (an `i32` would overflow at `n = 45`).

## Examples

```rust
assert_eq!(climb_stairs(2), 2);  // 1 + 1, or 2
assert_eq!(climb_stairs(3), 3);  // 1 + 1 + 1, 1 + 2, or 2 + 1
assert_eq!(climb_stairs(1), 1);
assert_eq!(climb_stairs(0), 1);  // already at the top: one (empty) way
assert_eq!(climb_stairs(92), 12_200_160_415_121_876_738); // still fits in u64
```

## Constraints

- `0 <= n <= 92` (the answer for `n = 93` no longer fits in a `u64`)

## Rust notes

- Only the last two values matter, so a pair of rolling variables beats a
  `Vec` table. Tuple assignment `(a, b) = (b, a + b);` updates both at once
  without a temporary.
- Tests run with overflow checks on: computing even **one** term past the one
  you need (a common off-by-one in Fibonacci loops) panics at `n = 92`.
- `(0..n).fold(...)` expresses "apply this step `n` times" without a mutable
  loop counter.
- A recursive version with a `HashMap` cache works too, but the iterative one
  needs no allocation at all.
