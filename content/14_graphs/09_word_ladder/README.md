## Description

A **transformation sequence** from `begin` to `end` is a list of words
`begin -> s1 -> s2 -> ... -> sk` such that:

- each pair of consecutive words differs in **exactly one** letter,
- every `si` (including `sk`) is in `words` — `begin` itself does not need to be,
- `sk == end`.

Return the **number of words** in the shortest such sequence (counting `begin`),
or `None` if no sequence exists.

## Signature

```rust
pub fn ladder_length(begin: &str, end: &str, words: &[&str]) -> Option<usize>
```

Everything is only read, so everything is borrowed — the caller's dictionary
can be a `Vec<&str>`, an array literal, or slices of a `Vec<String>`. "No
ladder" is `None` rather than the classic `0`, which would otherwise look like a
(meaningless) length.

## Examples

```rust
// "hit" -> "hot" -> "dot" -> "dog" -> "cog" has 5 words.
assert_eq!(ladder_length("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"]), Some(5));
// "cog" is not in the list, so it can never be reached.
assert_eq!(ladder_length("hit", "cog", &["hot", "dot", "dog", "lot", "log"]), None);
assert_eq!(ladder_length("cat", "cot", &["cot"]), Some(2));
```

## Constraints

- `1 <= begin.len() <= 12`
- `end.len() == begin.len()`, and every word in `words` has that length
- `1 <= words.len() <= 5000`
- All words are lowercase ASCII letters
- `begin != end`; the words in `words` are distinct

## Rust notes

- The words are ASCII, so work on `as_bytes()`: mutate a `Vec<u8>` scratch
  word in place and look it up in a `HashSet<&[u8]>` — `&scratch[..]` borrows
  it as a `&[u8]` key, no `String` allocation per candidate.
- `HashSet::remove` returns `bool`: removing a word the moment it's enqueued
  doubles as the "visited" check.
- Breadth-first search wants a `VecDeque`: `push_back` / `pop_front`. Store
  `(word, steps)` pairs or process the queue one level at a time.
