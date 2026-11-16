## Description

Given two strings `from` and `to`, return the minimum number of operations
needed to turn `from` into `to`. Each operation is one of:

- **insert** a character,
- **delete** a character,
- **replace** a character with another.

(This is the Levenshtein distance.) The strings may contain any Unicode text;
an operation acts on one `char`.

## Signature

```rust
pub fn edit_distance(from: &str, to: &str) -> usize
```

Both strings are only read, so they are borrowed. A distance is a count of
operations, at most the longer string's length, so it is a `usize`.

## Examples

```rust
// horse -> rorse (replace 'h') -> rose (delete 'r') -> ros (delete 'e')
assert_eq!(edit_distance("horse", "ros"), 3);
// intention -> inention -> enention -> exention -> exection -> execution
assert_eq!(edit_distance("intention", "execution"), 5);
assert_eq!(edit_distance("", "abc"), 3);     // three inserts
assert_eq!(edit_distance("café", "cafe"), 1); // one char replaced, not two bytes
```

## Constraints

- `0 <= from.chars().count(), to.chars().count() <= 500`

## Rust notes

- Work on `char`s: collect one string into a `Vec<char>` for indexing and walk
  the other with `chars().enumerate()`. Comparing `as_bytes()` counts a
  two-byte `'é'` as two edits.
- `(0..=n).collect::<Vec<usize>>()` builds the first row (`d[0][j] = j`) in
  one line.
- One rolling row is enough if you keep the old `d[i - 1][j - 1]` in a local
  before overwriting it; two rows with `std::mem::swap` are simpler to get right.
- With `usize` cells, `1 + a.min(b).min(c)` can't underflow; avoid writing the
  recurrence with subtractions.
