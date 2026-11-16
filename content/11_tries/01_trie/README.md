## Description

A **trie** (prefix tree) stores a set of strings as a tree of characters:
every word is a path from the root, and words that share a prefix share the
nodes for it. Lookups cost time proportional to the length of the query, not
to the number of stored words, and everything below a node starts with the
same prefix — which makes prefix queries cheap.

Build `Trie`, a set of strings, with the full textbook API:

- `insert(word)` adds `word`; returns `true` if it was new (like
  `HashSet::insert`).
- `contains(word)`: was `word` inserted (and not removed since)?
- `starts_with(prefix)`: does some stored word start with `prefix`? A word is
  a prefix of itself, and the empty prefix matches as soon as the trie holds
  any word.
- `remove(word)` deletes `word`; returns `true` if it was there. Nodes that no
  longer lead to any word must be **pruned**, so a trie that had words
  inserted and removed is as small as one built from the remaining words.
- `len()` / `is_empty()`: the number of distinct words stored.
- `count_with_prefix(prefix)`: how many stored words start with `prefix`.
- `words_with_prefix(prefix)`: those words, in sorted order.
- `longest_common_prefix()`: the longest string that is a prefix of every
  stored word; `None` for an empty trie.
- `node_count()`: the number of nodes, **not counting the root** — one per
  distinct non-empty prefix of the stored words. It's how the tests see your
  pruning.
- `FromIterator` and `Extend` for any item type that is `AsRef<str>` (`&str`,
  `String`, `&String`, …), inserting each word like `insert` does, so
  `words.iter().collect::<Trie>()` and `t.extend(["a", "b"])` work.

Words are arbitrary Unicode: the trie works on `char`s, so `"日本"` is two
nodes deep, not six. The empty string is a valid word.

## Restrictions

- No `HashSet`, `BTreeSet`, `Vec<String>` or other collection of whole words
  inside the trie — the tree *is* the set. (`HashMap`/`BTreeMap` from `char` to
  child node is fine.)
- No sorting in `words_with_prefix`: the order must come from the structure.

## Signature

```rust
pub struct Trie { /* your fields */ }

impl Trie {
    pub fn new() -> Self;
    pub fn insert(&mut self, word: &str) -> bool;
    pub fn contains(&self, word: &str) -> bool;
    pub fn starts_with(&self, prefix: &str) -> bool;
    pub fn remove(&mut self, word: &str) -> bool;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn count_with_prefix(&self, prefix: &str) -> usize;
    pub fn words_with_prefix(&self, prefix: &str) -> Vec<String>;
    pub fn longest_common_prefix(&self) -> Option<String>;
    pub fn node_count(&self) -> usize;
}

impl Default for Trie { /* given: calls Trie::new() */ }
impl<S: AsRef<str>> FromIterator<S> for Trie { /* … */ }
impl<S: AsRef<str>> Extend<S> for Trie { /* … */ }
```

Queries take `&str` and `&self`; only `insert` and `remove` need `&mut self`.
The trie doesn't keep the words as strings, so `words_with_prefix` and
`longest_common_prefix` build new `String`s. "Sorted" means `String`'s own
`Ord`, i.e. by Unicode code point.

## Examples

```rust
let mut t = Trie::new();
for w in ["car", "cart", "care", "cat", "dog"] {
    assert!(t.insert(w));
}
assert!(!t.insert("cat"));                    // already there
assert_eq!(t.len(), 5);
assert!(t.contains("car") && !t.contains("ca"));
assert!(t.starts_with("ca"));
assert_eq!(t.count_with_prefix("car"), 3);    // car, care, cart
assert_eq!(t.words_with_prefix("car"), ["car", "care", "cart"]);
assert_eq!(t.longest_common_prefix(), Some(String::new()));

assert!(t.remove("dog"));
assert!(!t.remove("dog"));
assert!(!t.starts_with("d"));                 // the "dog" branch is gone
assert_eq!(t.node_count(), 6);                // c, ca, car, cart, care, cat
assert_eq!(t.longest_common_prefix(), Some("ca".to_string()));

let mut u = Trie::new();
u.insert("日本");
u.insert("日本語");
assert_eq!(u.node_count(), 3);                // one node per char
assert_eq!(u.words_with_prefix("日"), ["日本", "日本語"]);

let mut v: Trie = ["to", "tea", "to"].into_iter().collect();
v.extend(vec![String::from("ten")]);
assert_eq!(v.words_with_prefix("t"), ["tea", "ten", "to"]);
```

## Constraints

- Words up to `1000` chars; up to `10^5` operations.
- `count_with_prefix` can be answered without enumerating the words below the
  prefix.

## Rust notes

- `BTreeMap<char, Node>` keeps children ordered, which hands you sorted output
  for free; `HashMap` is faster per step but you'd have to sort the children
  at each node you visit.
- Walking down with `let mut node = &mut self.root;` and reassigning `node`
  each step is fine for the borrow checker; the read-only walk is a
  `try_fold` over `word.chars()`.
- In `remove`, check that the word is there *before* changing anything, so a
  miss leaves the trie untouched. Removing a child from a map while you hold a
  `&mut` to that child won't compile — decide what to cut from the parent.
- `FromIterator` can be a two-liner on top of `Extend`: make an empty trie
  and `extend` it. `word.as_ref()` turns any `S: AsRef<str>` into a `&str`.
- `String::push` / `String::pop` work on `char`s, so one buffer serves the
  whole depth-first walk.
- Deeply nested boxes or maps drop recursively; words of a few thousand chars
  are fine, millions would need an iterative `Drop`.
