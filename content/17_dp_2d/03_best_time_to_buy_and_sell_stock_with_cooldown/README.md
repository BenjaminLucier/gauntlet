## Description

`prices[i]` is the price of a stock on day `i`. You may complete as many
transactions (buy one share, later sell it) as you like, subject to two rules:

- You can hold at most one share at a time — you must sell before buying again.
- After you **sell**, you must sit out the next day (a one-day cooldown) before
  you can buy again.

Return the maximum total profit you can achieve. Doing nothing is always
allowed, so the answer is never negative.

## Signature

```rust
pub fn max_profit(prices: &[u32]) -> u64
```

Prices can't be negative, so they are `u32`; the slice is only read, so it is
borrowed. The total profit can exceed `u32::MAX` (thousands of trades of up to
`u32::MAX` each), so it is returned as a `u64`. Intermediate states such as
"holding a share I paid for" are negative — use a signed, wider type inside.

## Examples

```rust
// buy (1), sell (2), cooldown, buy (0), sell (2) → 1 + 2
assert_eq!(max_profit(&[1, 2, 3, 0, 2]), 3);
assert_eq!(max_profit(&[1]), 0);
assert_eq!(max_profit(&[]), 0);             // no days, no trades
assert_eq!(max_profit(&[5, 4, 3]), 0);      // never buy
```

## Constraints

- `0 <= prices.len() <= 10^5`
- `0 <= prices[i] <= u32::MAX`

## Rust notes

- Updating three states "simultaneously" is easiest with tuple assignment:
  `(hold, sold, rest) = (…, …, …)` evaluates every right-hand side with the old
  values first.
- `let Some((&first, rest)) = prices.split_first() else { return 0 };` handles
  the empty slice and gives you a natural starting state.
- Convert with `i64::from(price)` (lossless, no `as`), and back with
  `u64::try_from(profit)` — the final profit can't be negative.
- Seeding "holding" with `i64::MIN` and then subtracting overflows; seed it from
  the first day instead.
