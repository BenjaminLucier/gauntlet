## Description

You have an unlimited supply of coins of each denomination in `coins`.
Return the **fewest** coins needed to make exactly `amount`, or `None` if no
combination of coins adds up to `amount`.

Making `0` takes no coins at all, whatever the denominations.

## Signature

```rust
pub fn coin_change(coins: &[u32], amount: u32) -> Option<usize>
```

Denominations and amounts are non-negative money values, so `u32`; the result
is a count of coins, so `usize`. "Impossible" is `None` rather than a magic
`-1` — the caller can't mistake it for a coin count.

## Examples

```rust
assert_eq!(coin_change(&[1, 2, 5], 11), Some(3)); // 5 + 5 + 1
assert_eq!(coin_change(&[2], 3), None);           // odd amounts are out of reach
assert_eq!(coin_change(&[1], 0), Some(0));
assert_eq!(coin_change(&[1, 3, 4], 6), Some(2));  // 3 + 3, not greedy 4 + 1 + 1
assert_eq!(coin_change(&[u32::MAX, 3], 9), Some(3)); // huge coins are simply unusable
```

## Constraints

- `0 <= coins.len() <= 12`
- `1 <= coins[i] <= u32::MAX`
- `0 <= amount <= 10^4`

## Rust notes

- A `Vec<Option<usize>>` table needs no "infinity" sentinel: `None` is
  unreachable, and `Iterator::min` over the reachable predecessors returns an
  `Option` already — `.map(|m| m + 1)` finishes the job.
- `usize::checked_sub` gives `None` when a coin is larger than the target, so
  you never compute a negative (or wrapped) index. Adding instead (`a + coin`)
  can overflow with coins near `u32::MAX`.
- `filter_map` chains "skip coins that don't fit" and "skip unreachable
  remainders" without nested `if`s.
- In `table[a] = expression_reading_table`, the right-hand side is evaluated
  first, so the borrow checker accepts it.
