## Description

Given a string `s`, return a **contiguous** substring of `s` that reads the
same forwards and backwards and is as long as possible. If several longest
palindromic substrings exist, any one of them is accepted.

The string may contain any Unicode text. "Reads the same backwards" is
measured in `char`s (Unicode scalar values): `"héh"` is a palindrome, and
length means the number of `char`s. For an empty string, return `""`.

## Signature

```rust
pub fn longest_palindrome(s: &str) -> &str
```

The answer is part of the input, so the function returns a slice **borrowed
from `s`** rather than allocating a new `String`. Lifetime elision ties the two
together: the signature means `fn longest_palindrome<'a>(s: &'a str) -> &'a str`,
so the result stays valid exactly as long as `s` does.

## Examples

```rust
let got = longest_palindrome("babad");
assert!(got == "bab" || got == "aba"); // either is accepted
assert_eq!(longest_palindrome("cbbd"), "bb");
assert_eq!(longest_palindrome("a"), "a");
assert_eq!(longest_palindrome(""), "");
assert_eq!(longest_palindrome("xhéhy"), "héh"); // 'é' is two bytes but one char

// The result points into the input: no copy was made.
let s = String::from("xxracecaryy");
let p = longest_palindrome(&s);
assert_eq!(p, "racecar");
assert_eq!(p.as_ptr(), s[2..].as_ptr());
```

## Constraints

- `0 <= s.chars().count() <= 1000`
- `s` is any valid UTF-8; characters are compared as `char`s (no grapheme
  clustering or case folding: `'A' != 'a'`).

## Rust notes

- `&s[a..b]` takes **byte** offsets and panics if either one falls inside a
  multi-byte character. `char_indices()` yields each `char` with the byte offset
  where it starts — exactly the offsets you can slice at.
- Working on `s.as_bytes()` is fast and correct for ASCII, but wrong here:
  `"é"` is the bytes `[0xC3, 0xA9]`, and `"héh"` isn't a palindrome byte-wise.
  Collect the `char`s into a `Vec<char>` for `O(1)` indexing instead
  (`chars().nth(i)` is `O(i)`).
- Expanding with `usize` indices: keep the palindrome as a half-open range
  `lo..hi` and compare `chars[lo - 1]` with `chars[hi]` after checking
  `lo > 0`, so you never need a signed index or a `-1`.
- `Iterator::max_by_key` picks the widest range in one expression.
