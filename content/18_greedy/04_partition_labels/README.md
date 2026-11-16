## Description

Given an ASCII string `s`, cut it into as **many** contiguous pieces as
possible so that every character appears in at most one piece. Concatenating
the pieces in order gives back `s`.

Return the pieces, in order. An empty string has no pieces.

## Signature

```rust
pub fn partition_labels(s: &str) -> Vec<&str>
```

The pieces are returned as `&str` slices **borrowed from the input** — no
copying, and the caller can still get the lengths with `.len()`. Lifetime
elision ties the output to `s`: the full signature is
`fn partition_labels<'a>(s: &'a str) -> Vec<&'a str>`, so the pieces can't
outlive the string they point into.

## Examples

```rust
// Splitting further would put some letter into two pieces.
assert_eq!(
    partition_labels("ababcbacadefegdehijhklij"),
    ["ababcbaca", "defegde", "hijhklij"]
);
assert_eq!(partition_labels("eccbbbbdec"), ["eccbbbbdec"]);
assert_eq!(partition_labels("abc"), ["a", "b", "c"]);
assert!(partition_labels("").is_empty());
```

## Constraints

- `0 <= s.len() <= 10^5`
- `s` is ASCII (the classic version uses lowercase letters only).

## Rust notes

- Because `s` is ASCII, byte indices are character boundaries:
  work on `s.as_bytes()` and slice the original with `&s[start..=end]`.
- A fixed `[usize; 256]` table indexed by `usize::from(byte)` replaces a hash
  map when the key space is tiny.
- Returning `Vec<&str>` instead of `Vec<String>` avoids an allocation per piece;
  the borrow checker guarantees the caller keeps `s` alive while using them.
- `assert_eq!` can compare a `Vec<&str>` with an array of `&str` directly.
