## Description

An alien language uses the lowercase English letters, but in an unknown order.
You are given `words`, a list of words from its dictionary that is claimed to be
**sorted lexicographically** by the alien alphabet. (As usual, a word that is a
proper prefix of another comes first: `"ab"` < `"abc"`.)

Return a string containing **every distinct letter that appears in `words`**,
each exactly once, in an order consistent with the sorting. If several orders
are consistent, return any of them. If no order makes `words` sorted, return
`None`.

## Signature

```rust
pub fn alien_order(words: &[&str]) -> Option<String>
```

The words are only read, so they are borrowed as `&[&str]`. "No alphabet
works" is `None` rather than an empty string — an empty list of words has the
perfectly valid answer `Some(String::new())`, and the two shouldn't look alike.

## Examples

```rust
// wrt < wrf gives t < f; wrf < er gives w < e; er < ett gives r < t;
// ett < rftt gives e < r. Together: w < e < r < t < f.
assert_eq!(alien_order(&["wrt", "wrf", "er", "ett", "rftt"]).as_deref(), Some("wertf"));
assert_eq!(alien_order(&["z", "x"]).as_deref(), Some("zx"));
// z < x and x < z cannot both hold.
assert_eq!(alien_order(&["z", "x", "z"]), None);
// "ab" is a prefix of "abc", so it must come first — no alphabet can fix that.
assert_eq!(alien_order(&["abc", "ab"]), None);
// Several orders work here ("ab" or "ba"); any of them is accepted.
assert!(alien_order(&["ab"]).is_some());
```

## Constraints

- `0 <= words.len() <= 100`
- `1 <= words[i].len() <= 100`
- `words[i]` consists of lowercase ASCII letters `a`–`z`, so working on
  `as_bytes()` is fine

## Rust notes

- `words.windows(2)` hands you each adjacent pair; `a.iter().zip(b).find(|(x, y)| x != y)`
  finds the first difference (or `None` when one word is a prefix of the other).
- With only 26 letters, fixed-size arrays (`[bool; 26]`, `[[bool; 26]; 26]`,
  or a `u32` bitmask per letter) beat a `HashMap<char, HashSet<char>>` — no
  hashing, no allocation, and `b - b'a'` is the index.
- Don't add the same edge twice, or an in-degree count will never reach zero.
- `char::from(b'a' + i as u8)` turns an index back into a letter; `collect()`
  an iterator of `char`s straight into a `String`.
- `bool::then` / `then_some` turns "did every letter get placed?" into the
  `Option` result.
