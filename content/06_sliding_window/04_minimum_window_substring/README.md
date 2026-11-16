## Description

Given strings `s` and `t`, find the **shortest** substring of `s` that
contains every character of `t`, **including duplicates** (if `t` has two
`'a'`s, the window needs at least two `'a'`s). Characters are case-sensitive.

Return that substring as `Some(window)`, or `None` if no substring of `s`
covers `t`. If several shortest windows exist, return the **leftmost** one.
An empty `t` is covered by the empty window at the start of `s`.

## Signature

```rust
pub fn min_window<'a>(s: &'a str, t: &str) -> Option<&'a str>
```

The answer is a piece of `s`, so the function returns a slice *of `s`*
instead of allocating a new `String`. The lifetime `'a` ties the result to `s`
only — `t` can be a temporary. "No window" is `None`, not an empty string
(which would be ambiguous with the answer for an empty `t`).

## Examples

```rust
assert_eq!(min_window("ADOBECODEBANC", "ABC"), Some("BANC"));
assert_eq!(min_window("a", "a"), Some("a"));
assert_eq!(min_window("a", "aa"), None);          // t needs two 'a's
assert_eq!(min_window("aAbBa", "Ab"), Some("Ab")); // case-sensitive
assert_eq!(min_window("xyzyx", "y"), Some("y"));  // leftmost of two shortest
assert_eq!(min_window("abc", ""), Some(""));
```

## Constraints

- `0 <= s.len(), t.len() <= 10^5`
- `s` and `t` consist of ASCII letters (`A`–`Z`, `a`–`z`).

## Rust notes

- The input is ASCII, so byte positions are character positions: work on
  `as_bytes()` and return `&s[start..end]` — slicing at those byte offsets
  can't split a character.
- A `[usize; 128]` (or `[i32; 128]`) table indexed by `usize::from(byte)`
  covers both cases of every letter; a 26-entry table can't.
- `Option<(usize, usize)>` is a natural "best window so far"; combine it with
  `is_none_or` (or `map_or(true, …)`) for the "is this one shorter?" check,
  and finish with `best.map(|(l, r)| &s[l..r])`.
- Returning `&'a str` means the borrow checker guarantees the caller can't
  drop `s` while still holding the window.
