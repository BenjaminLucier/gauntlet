## Description

You are given an `m x n` map whose rows are strings of `'1'` (land) and `'0'`
(water). An island is a group of land cells connected **4-directionally**
(up, down, left, right). The **area** of an island is the number of cells in it.

Return the largest island area on the map, or `0` if there is no land at all.

## Signature

```rust
pub fn max_area_of_island(grid: &[&str]) -> usize
```

The map is only read, so it is borrowed as ASCII rows (`&["0110", "0100"]`);
track visited cells in your own buffer. An area is a count, hence `usize`.

## Examples

```rust
let grid = [
    "0010000100000",
    "0000000111000",
    "0110100000000",
    "0100110010100",
    "0100110011100",
    "0000000000100",
    "0000000111000",
    "0000000110000",
];
assert_eq!(max_area_of_island(&grid), 6); // diagonal neighbours don't join islands
assert_eq!(max_area_of_island(&["00000000"]), 0);
assert_eq!(max_area_of_island(&["101", "101", "111"]), 7);
assert_eq!(max_area_of_island(&[]), 0);
```

## Constraints

- `0 <= m, n <= 300`; all rows have the same length
- every character is `'0'` or `'1'`

## Rust notes

- `row.as_bytes()[c] == b'1'` — the rows are ASCII, so bytes are characters.
- Build the four neighbours with `wrapping_sub(1)` and one `a < m && b < n`
  check, or with `checked_sub` and `Option` combinators; plain `r - 1` panics
  at the top edge (tests run with overflow checks on).
- Use an explicit `Vec` stack instead of recursion: an all-land 300 x 300 map
  is a 90,000-cell island.
- `Iterator::max` over the island sizes returns `Option` — `unwrap_or(0)`
  covers the no-land case.
