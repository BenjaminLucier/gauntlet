## Description

A CPU must run a list of tasks, each labelled with a single byte (usually an
uppercase letter such as `b'A'`). Every task takes exactly one time unit, and
in each unit the CPU either runs one task or sits idle. Tasks may run in any
order, but two runs of the **same** label must be separated by at least
`cooldown` time units (other tasks or idling).

Return the minimum number of time units needed to finish all tasks. An empty
task list takes `0` units.

## Signature

```rust
pub fn least_interval(tasks: &[u8], cooldown: usize) -> usize
```

Labels are bytes, so the input is a byte slice — tests read naturally as
`least_interval(b"AAABBB", 2)`. Any byte is a valid label (there are only 256),
so a fixed `[usize; 256]` table counts them without hashing and without
assuming `A..=Z`. The cooldown and the result are lengths of time, so they're
`usize`.

## Examples

```rust
// A B idle A B idle A B
assert_eq!(least_interval(b"AAABBB", 2), 8);
// A B C D A B — no idling needed
assert_eq!(least_interval(b"ACABDB", 1), 6);
// A B idle idle A B idle idle A B
assert_eq!(least_interval(b"AAABBB", 3), 10);
assert_eq!(least_interval(b"", 5), 0);
```

## Constraints

- `0 <= tasks.len() <= 10^4`
- `0 <= cooldown <= 100`
- Labels are arbitrary bytes; the tests mostly use `b'A'..=b'Z'`.

## Rust notes

- `b"AAB"` is a `&[u8; 3]`, which coerces to `&[u8]`. Index a
  `[usize; 256]` table directly with `usize::from(byte)` — no subtraction of
  `b'A'`, so no underflow on other bytes.
- `Iterator::max`, `filter(..).count()` and `Ord::max` express the counting
  version in a few lines; `max()` returns `Option` — the empty input is real.
- For the simulation: `BinaryHeap<usize>` (max-heap) of remaining counts plus
  a `VecDeque<(ready_at, count)>` of labels cooling down.
- `max_count - 1` underflows a `usize` when there are no tasks at all.
