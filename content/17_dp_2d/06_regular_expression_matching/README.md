## Description

Implement a tiny regular-expression matcher. Given a string `s` and a pattern
`p`, decide whether `p` matches **the whole of** `s` (not just a part of it).
The pattern supports:

- `'.'` — matches any single character.
- `'*'` — matches **zero or more** copies of the element right before it.

Every other character matches only itself. There is no escaping and there are
no other operators. Both strings may contain any Unicode text; "character"
means `char`.

A `*` must follow a character or `.` that it repeats. A pattern with a `*` at
the very start, or a `*` straight after another `*`, is malformed: return
`Err(PatternError::NothingToRepeat { at })`, where `at` is the byte offset of
the first such `*` in `p`. A malformed pattern is an error even if matching
could have failed earlier.

## Signature

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternError {
    /// A `*` with nothing to repeat; `at` is its byte offset in the pattern.
    NothingToRepeat { at: usize },
}

impl fmt::Display for PatternError { /* given */ }
impl std::error::Error for PatternError {}

pub fn is_match(s: &str, p: &str) -> Result<bool, PatternError>
```

"Doesn't match" and "isn't a valid pattern" are different answers, so the
result is a `Result<bool, _>` rather than a plain `bool`. The error type is a
small enum implementing `std::error::Error`, so callers can use `?` with it.
Byte offsets are what you'd use to slice `p` (`&p[at..]`).

## Examples

```rust
assert_eq!(is_match("aa", "a"), Ok(false));     // only the first 'a' is matched
assert_eq!(is_match("aa", "a*"), Ok(true));     // two copies of 'a'
assert_eq!(is_match("ab", ".*"), Ok(true));     // zero or more of anything
assert_eq!(is_match("aab", "c*a*b"), Ok(true)); // zero c's
assert_eq!(is_match("héllo", "h.l*o"), Ok(true)); // '.' is one char, not one byte
assert_eq!(is_match("a", "*a"), Err(PatternError::NothingToRepeat { at: 0 }));
assert_eq!(is_match("a", "a**"), Err(PatternError::NothingToRepeat { at: 2 }));
```

## Constraints

- `0 <= s.chars().count() <= 30`
- `0 <= p.chars().count() <= 30`

## Rust notes

- Parse first, match second: turn `p` into a `Vec` of small tokens (an enum
  `Atom { Any, Char(char) }` plus a `starred: bool`) and validate while you do
  it. The matcher then never looks at a raw `*`, and the DP indexes tokens.
- `p.char_indices()` yields `(byte_offset, char)` — exactly the `at` you need
  for the error. `tokens.last_mut()` lets a `*` mark the previous token.
- Collect `s` into a `Vec<char>` for `O(1)` indexing; `s.get(i)` returns `None`
  past the end, which reads nicely as "no character left to match".
- Rolling rows with `std::mem::swap` work if you fill from the end of `s`.
