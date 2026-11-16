## Description

Given a string `s` and a dictionary `words`, return `true` if `s` can be split
into a sequence of dictionary words with nothing left over, and `false`
otherwise. A dictionary word may be used any number of times.

The empty string is trivially the concatenation of zero words, so it is
always `true`. Strings and words may contain any Unicode text.

## Signature

```rust
pub fn word_break(s: &str, words: &[&str]) -> bool
```

Both the text and the dictionary are only read, so both are borrowed: a slice
of string slices. Callers holding a `Vec<String>` can pass
`&owned.iter().map(String::as_str).collect::<Vec<_>>()`.

## Examples

```rust
assert!(word_break("gauntlet", &["gaunt", "let"]));
assert!(word_break("applepenapple", &["apple", "pen"])); // words can be reused
assert!(!word_break("catsandog", &["cats", "dog", "sand", "and", "cat"]));
assert!(word_break("cars", &["car", "ca", "rs"]));        // "car" first is a dead end
assert!(word_break("crème brûlée", &["crème", " ", "brûlée"]));
assert!(word_break("", &[]));
```

## Constraints

- `0 <= s.len() <= 300` (bytes)
- `0 <= words.len() <= 1000`
- `1 <= words[i].len() <= 20` (bytes); no word is empty
- Words are distinct. Any valid UTF-8 is allowed.

## Rust notes

- `&s[j..i]` **panics** if `j` or `i` lands inside a multi-byte character.
  `s.get(j..i)` returns an `Option<&str>` instead — `None` at a non-boundary —
  so a split point inside `'é'` is simply skipped.
- Byte-level matching is also sound for UTF-8: if a word's bytes match
  `s.as_bytes()[j..i]`, both ends are guaranteed to be character boundaries.
  `s.as_bytes()[..i].ends_with(w.as_bytes())` is a fine test.
- A `HashSet<&str>` can borrow straight from `words` — no `String`s needed —
  and `contains` accepts the `&str` pieces you slice out of `s`.
- `i.saturating_sub(max_len)` bounds the start of the last word without
  underflowing.
