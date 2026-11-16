## Description

Given two sequences `a` and `b`, return the length of their longest **common
subsequence**, or `0` if they share no elements.

A subsequence is what remains after deleting zero or more elements without
changing the order of the rest — `"ace"` is a subsequence of `"abcde"`, `"aec"`
is not. A common subsequence is a subsequence of both.

The function is generic: it works on bytes of ASCII strings, on `char`s, on
words, on numbers — anything that can be compared for equality.

## Signature

```rust
pub fn longest_common_subsequence<T: Eq>(a: &[T], b: &[T]) -> usize
```

Only equality is needed, so the element type is any `T: Eq`. Both inputs are
borrowed slices; for ASCII text pass `s.as_bytes()` (or a byte string literal
`b"..."`), for arbitrary text collect the `char`s first. The result is a length,
hence `usize`.

## Examples

```rust
assert_eq!(longest_common_subsequence(b"abcde", b"ace"), 3); // "ace"
assert_eq!(longest_common_subsequence(b"abc", b"abc"), 3);
assert_eq!(longest_common_subsequence(b"abc", b"def"), 0);
assert_eq!(longest_common_subsequence("abc".as_bytes(), b""), 0);
// Whole words as elements: "the", "sat" is common.
assert_eq!(longest_common_subsequence(&["the", "cat", "sat"], &["the", "dog", "sat"]), 2);
```

## Constraints

- `0 <= a.len(), b.len() <= 1000`

## Rust notes

- Two rolling rows plus `std::mem::swap(&mut prev, &mut cur)` swap the
  buffers in `O(1)` — no reallocation, no cloning.
- If you do want a full table, a flat `vec![0; (m + 1) * (n + 1)]` indexed as
  `i * (n + 1) + j` is one allocation instead of `m + 1`.
- Zipping the row with the other sequence (`b.iter().enumerate()`, or
  `windows`) keeps bounds checks out of the inner loop.
- For text, `as_bytes()` indexing is `O(1)`; `chars().nth(i)` is `O(i)` and
  turns the DP cubic. Collect into a `Vec<char>` if you need Unicode.
