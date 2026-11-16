## Description

You are given a string `s` of uppercase ASCII letters and a budget `k`. You
may change any character of `s` to any other uppercase letter, at most `k`
times in total.

Return the length of the longest substring containing a single repeated letter
that you can get after these changes.

## Signature

```rust
pub fn character_replacement(s: &str, k: usize) -> usize
```

The string is only read, so it's borrowed. The budget and the answer are
counts, so both are `usize` — a negative budget is unrepresentable.

## Examples

```rust
assert_eq!(character_replacement("ABAB", 2), 4);      // change both 'A's to 'B' (or vice versa)
assert_eq!(character_replacement("AABABBA", 1), 4);   // "AABBBBA": "BBBB"
assert_eq!(character_replacement("ABCDE", 0), 1);     // no changes: longest run is 1
assert_eq!(character_replacement("ABC", 10), 3);      // can't exceed the string's length
assert_eq!(character_replacement("", 3), 0);
```

## Constraints

- `0 <= s.len() <= 10^5`
- `s` consists of uppercase ASCII letters `A`–`Z`.
- `k` is any `usize` (it may exceed `s.len()`).

## Rust notes

- The input is ASCII, so work on `s.as_bytes()`; `usize::from(b - b'A')`
  indexes a `[usize; 26]` of counts.
- A window `[left, right]` holds `right + 1 - left` characters. Write
  `usize` comparisons so nothing is subtracted below zero —
  `len - max_count > k` is safe because `max_count <= len`, but
  `len - k` is not.
- `k` can be huge (`usize::MAX`), so never compute `something + k`.
