## Description

Letters are encoded as numbers: `'A' -> "1"`, `'B' -> "2"`, …, `'Z' -> "26"`.
Given a string of digits, return the number of ways to split it into pieces
that each decode to a letter.

A piece is valid only if it is `"1"`–`"9"` or `"10"`–`"26"`. In particular
`"0"` alone is invalid and pieces with a leading zero such as `"06"` are
invalid. If the string cannot be decoded at all, the answer is `0`.

The empty string has exactly one decoding: the empty message.

## Signature

```rust
pub fn num_decodings(digits: &str) -> u64
```

The input is borrowed text. The count is never negative and grows like the
Fibonacci numbers, so it is a `u64`: a string of 92 `'1'`s already has more
decodings than an `i64` can hold. `0` is a real answer here ("no decodings"),
not an error, so there is no `Option`.

## Examples

```rust
assert_eq!(num_decodings("12"), 2);  // "AB" (1 2) or "L" (12)
assert_eq!(num_decodings("226"), 3); // "BZ" (2 26), "VF" (22 6), "BBF" (2 2 6)
assert_eq!(num_decodings("06"), 0);  // "06" is not a piece, and "0" alone isn't either
assert_eq!(num_decodings("10"), 1);  // only "J"
assert_eq!(num_decodings(""), 1);
```

## Constraints

- `0 <= digits.len() <= 100`
- `digits` contains only the ASCII digits `'0'`–`'9'` and may have leading zeros.
- The number of decodings of every **prefix** of `digits` fits in a `u64`.

## Rust notes

- The input is ASCII digits, so work on `digits.as_bytes()` and compare with
  byte literals like `b'0'` — no `char` decoding or `parse` needed.
- `matches!` with byte ranges reads like the rule itself:
  `matches!((tens, ones), (b'1', b'0'..=b'9') | (b'2', b'0'..=b'6'))`.
- Two rolling variables replace the `ways` table. Tests run with overflow
  checks on and include counts above `i64::MAX`, so the type matters.
- `as_bytes().windows(2)` yields each two-digit candidate as a `&[u8]`, if you
  prefer to look at pairs directly.
