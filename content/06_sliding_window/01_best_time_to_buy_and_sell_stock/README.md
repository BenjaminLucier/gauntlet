## Description

`prices[i]` is the price of a stock on day `i`. You may buy on one day and
sell on a **later** day — one transaction at most.

Return the maximum profit you can make. If no profitable trade exists
(including when there are fewer than two days), return `0`: not trading is
always an option.

## Signature

```rust
pub fn max_profit(prices: &[u32]) -> u32
```

Prices are never negative, and neither is the best profit (you can always
decline to trade), so both are unsigned. The slice is only read, so it's
borrowed.

## Examples

```rust
assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 5);   // buy at 1 (day 1), sell at 6 (day 4)
assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);      // prices only fall: don't trade
assert_eq!(max_profit(&[2, 10, 1, 4]), 8);        // not 10 - 1: you must buy first
assert_eq!(max_profit(&[]), 0);
```

## Constraints

- `0 <= prices.len() <= 10^5`
- `0 <= prices[i] <= u32::MAX`

## Rust notes

- `price - min` on `u32` panics if `min > price`. Make sure the minimum
  you subtract already includes today's price, or use `saturating_sub`.
- Starting the running minimum at `u32::MAX` is a common trick — but
  `Option<u32>` or "the first price" express "nothing seen yet" more honestly.
- A single `fold` (or a `for` loop with two `mut` variables) is all this
  needs; `Ord::min` / `Ord::max` keep the updates short.
