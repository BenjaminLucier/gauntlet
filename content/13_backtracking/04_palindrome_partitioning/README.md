## Description

Given a string `s`, split it into consecutive non-empty pieces so that **every
piece is a palindrome** (reads the same forwards and backwards, character by
character). Return every possible way to do this.

Each partition lists its pieces in left-to-right order. The partitions
themselves may be returned in **any order**. The empty string has exactly one
partition: the one with no pieces.

## Signature

```rust
pub fn partition(s: &str) -> Vec<Vec<&str>>
```

Every piece is a substring of `s`, so the output **borrows** from the input
instead of allocating a `String` per piece. The elided lifetime ties them
together: the result can't outlive `s`.

The input may contain any Unicode text; palindromes are judged by `char`, so
`"éé"` is a palindrome even though its bytes reversed are not.

## Examples

```rust
let mut parts = partition("aab");
parts.sort();
assert_eq!(parts, vec![vec!["a", "a", "b"], vec!["aa", "b"]]);
assert_eq!(partition("a"), vec![vec!["a"]]);
assert_eq!(partition("ab"), vec![vec!["a", "b"]]);
assert_eq!(partition(""), vec![Vec::<&str>::new()]);   // one partition, no pieces
assert_eq!(partition("éé").len(), 2);                  // ["é", "é"] and ["éé"]
```

## Constraints

- `0 <= s.chars().count() <= 16`

## Rust notes

- Slicing a `&str` takes **byte** offsets and panics if one falls inside a
  multi-byte character. `s.char_indices()` gives you every valid cut position;
  remember `s.len()` as the final one.
- Returning `Vec<Vec<&'a str>>` means the backtracking path is a
  `&mut Vec<&'a str>` — `push(&s[i..j])`, recurse, `pop()`. Pushing slices is
  free; only the final `path.clone()` copies (pointers, not text).
- Compare characters, not bytes: collect `s.chars()` into a `Vec<char>` for the
  palindrome table.
