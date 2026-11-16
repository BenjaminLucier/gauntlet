## Description

Implement `WordDictionary`, which stores words and answers whole-word
queries where `.` is a wildcard:

- `WordDictionary::new()` (or `::default()`) creates an empty dictionary.
- `insert(word)` stores `word` (storing it again changes nothing).
- `search(pattern)` returns `true` if some stored word matches `pattern`
  exactly, where each `.` matches **exactly one** arbitrary letter. Lengths
  must match.

Words are lowercase ASCII letters; patterns are lowercase ASCII letters and
`.`. The empty string is a valid word and a valid pattern (it matches only
the empty word).

## Signature

```rust
pub struct WordDictionary { /* your fields */ }

impl WordDictionary {
    pub fn new() -> Self
    pub fn insert(&mut self, word: &str)
    pub fn search(&self, pattern: &str) -> bool
}

impl Default for WordDictionary { /* given: calls WordDictionary::new() */ }
```

Arguments are borrowed `&str`s: the dictionary copies what it needs into its
own nodes, so callers never have to hand over (or allocate) a `String`.
Searching doesn't change the dictionary, so it takes `&self`.

## Examples

```rust
let mut d = WordDictionary::new();
d.insert("bad");
d.insert("dad");
d.insert("mad");
assert!(!d.search("pad"));
assert!(d.search("bad"));
assert!(d.search(".ad"));  // bad, dad and mad all match
assert!(d.search("b.."));  // bad
assert!(!d.search("b."));  // too short: a pattern must match a whole word
```

## Constraints

- `0 <= word.len(), pattern.len() <= 25`
- Typical patterns have at most 2 dots, but the tests also use more
- At most `10^4` calls in total

## Rust notes

- Slice patterns make the recursive search read like its definition:
  `match pattern { [] => …, [b'.', rest @ ..] => …, [b, rest @ ..] => … }`
  on `pattern.as_bytes()`.
- `children.iter().flatten()` walks only the children that exist (an
  `Option` is an iterator of zero or one items), and `any` stops at the first
  branch that matches.
- `Option::is_some_and` and `as_deref()` turn "follow this child if it exists
  and check the rest" into one expression.
- Recursion depth is the pattern length (at most 25), so recursion is the
  natural choice here.
