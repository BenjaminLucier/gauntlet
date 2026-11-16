## Description

Given a list of words, group together the words that are anagrams of each
other (same letters with the same counts). Every input word must appear in
exactly one group — duplicates included.

The groups, and the words within each group, may be returned in **any
order**.

## Signature

```rust
pub fn group_anagrams<'a>(words: &[&'a str]) -> Vec<Vec<&'a str>>
```

Grouping doesn't need to copy any text: the output holds the same `&str`s that
came in. The lifetime `'a` says exactly that — the returned slices borrow from
whatever the input words borrow from, not from the `words` slice itself, so
the result can outlive the slice of references.

## Examples

```rust
let mut groups = group_anagrams(&["eat", "tea", "tan", "ate", "nat", "bat"]);
// Any order is fine; sort to compare.
groups.iter_mut().for_each(|g| g.sort_unstable());
groups.sort_unstable();
assert_eq!(groups, vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]]);

assert_eq!(group_anagrams(&[""]), vec![vec![""]]);
assert_eq!(group_anagrams(&["a"]), vec![vec!["a"]]);
assert!(group_anagrams(&[]).is_empty());           // no words, no groups
```

## Constraints

- `0 <= words.len() <= 10^4`
- `0 <= words[i].len() <= 100`
- Words consist of lowercase ASCII letters `a`–`z`.

## Rust notes

- Arrays implement `Hash` and `Eq`, so a fixed-size `[u8; 26]` works directly
  as a `HashMap` key — no need to build a `String`.
- `map.entry(key).or_default().push(word)` creates each group on first sight.
- `HashMap::into_values()` hands over the grouped `Vec`s without cloning them.
- Because the input is ASCII, `word.bytes()` is safe to use and `b - b'a'`
  gives a letter index.
- Copying a `&'a str` out of the slice (`for &word in words`) is free and keeps
  the original lifetime `'a`.
