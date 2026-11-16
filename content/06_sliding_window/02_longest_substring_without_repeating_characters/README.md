## Description

Given a string `s`, return the length of the longest **substring** (contiguous
run of characters) in which no character appears more than once.

Characters are Unicode scalar values (`char`s), and the length is counted in
characters, not bytes: `"日本語"` has three distinct characters, so the answer
is `3` even though it is nine bytes long.

## Signature

```rust
pub fn length_of_longest_substring(s: &str) -> usize
```

The string is only read, so it's borrowed. The answer is a length — `usize`.

## Examples

```rust
assert_eq!(length_of_longest_substring("abcabcbb"), 3);   // "abc"
assert_eq!(length_of_longest_substring("bbbbb"), 1);
assert_eq!(length_of_longest_substring("pwwkew"), 3);     // "wke"; "pwke" isn't contiguous
assert_eq!(length_of_longest_substring(""), 0);
assert_eq!(length_of_longest_substring("日本日本語"), 3);  // "本日本" repeats; "日本語" doesn't
```

## Constraints

- `0 <= s.len() <= 5 * 10^4` (bytes)
- `s` is any valid UTF-8; most tests use printable ASCII.

## Rust notes

- `s.chars().enumerate()` gives `(char position, char)`; `s.char_indices()`
  gives `(byte offset, char)`. Either works, but don't mix the two units in
  one window.
- A `HashMap<char, usize>` remembering where each character was last seen
  replaces a fixed `[usize; 128]` table once you leave ASCII.
- `HashMap::insert` returns the *previous* value as an `Option` — exactly the
  "where did I last see this?" you need, in the same call that updates it.
- Bytes are not characters: on `"ééé"` a byte-based window finds two
  "distinct" bytes in a row.
