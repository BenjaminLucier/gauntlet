## Description

`temperatures[i]` is the temperature on day `i`. Return a vector `answer` where
`answer[i]` is the number of days you must wait after day `i` until a
**strictly warmer** day, or `None` if no warmer day ever comes.

## Signature

```rust
pub fn daily_temperatures(temperatures: &[i32]) -> Vec<Option<usize>>
```

The readings are only read, so they are borrowed. A wait is a count of days —
`usize` — and "never" is `None` rather than a magic `0`, which would otherwise
be indistinguishable from a real answer in a differently-shaped problem.

## Examples

```rust
assert_eq!(
    daily_temperatures(&[73, 74, 75, 71, 69, 72, 76, 73]),
    [Some(1), Some(1), Some(4), Some(2), Some(1), Some(1), None, None],
); // after day 2 (75) the next warmer day is day 6 (76): 4 days later
assert_eq!(daily_temperatures(&[30, 40, 50, 60]), [Some(1), Some(1), Some(1), None]);
assert_eq!(daily_temperatures(&[30, 60, 90]), [Some(1), Some(1), None]);
assert_eq!(daily_temperatures(&[]), []);
```

## Constraints

- `0 <= temperatures.len() <= 10^5`
- `-100 <= temperatures[i] <= 100`

## Rust notes

- `vec![None; n]` builds the answer up front; fill in the days that get resolved.
- `while let Some(&j) = stack.last()` is the idiomatic "peek while the
  condition holds" loop; `break` out when it stops holding.
- `iter().enumerate()` gives you `(index, &value)` pairs without index
  arithmetic.
