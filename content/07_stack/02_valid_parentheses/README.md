## Description

You are given a string made up only of the bracket characters `(`, `)`, `[`,
`]`, `{` and `}`. Decide whether it is **valid**:

- every opening bracket is closed by a bracket of the **same type**,
- brackets are closed in the **correct order** (the most recently opened one
  must be closed first), and
- every closing bracket has a matching opening bracket before it.

The empty string is valid: it opens nothing and closes nothing.

## Signature

```rust
pub fn is_valid(s: &str) -> bool
```

The string is only read, so it is borrowed as `&str`; callers can pass a
literal, a `&String`, or a slice of a larger buffer without allocating.

## Examples

```rust
assert!(is_valid("()"));
assert!(is_valid("()[]{}"));
assert!(!is_valid("(]"));        // "(" closed by a bracket of a different type
assert!(is_valid("([])"));
assert!(!is_valid("([)]"));      // counts balance, but "[" must close before ")"
assert!(is_valid(""));           // nothing to match
```

## Constraints

- `0 <= s.len() <= 10^4`
- `s` consists only of the ASCII characters `()[]{}`

## Rust notes

- The input is ASCII, so `s.bytes()` and byte literals like `b'('` are enough;
  no need for `chars()`.
- A `Vec<u8>` is a perfectly good stack: `push`, `pop` and `last`.
- `Vec::pop` returns an `Option`, so comparing it directly with `Some(expected)`
  handles the empty-stack case without a separate check.
- A `match` on the byte keeps the bracket table readable — one arm per opener.
