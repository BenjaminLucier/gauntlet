## Description

Build `ChainedMap<K, V>`, a hash map that resolves collisions by **separate
chaining**: an array of buckets, each holding the `(key, value)` pairs whose
hash maps to it.

To find a key, hash it, pick a bucket from the hash, and scan that bucket for
an equal key. As long as buckets stay short, every operation is `O(1)`
expected. They stay short only if the table **grows**: when the load factor
`len / bucket_count` would pass a threshold (3/4 is a fine choice), double the
bucket count and rehash every pair into the new array.

Implement:

- `new`, `len`, `is_empty`
- `insert` (returns the previous value, like `HashMap::insert`), `get`,
  `get_mut`, `contains_key`, `remove`
- `get_or_insert_with(key, default)` — a small cousin of the `entry` API:
  the value for `key`, inserting `default()` first if it's missing
- `iter`, returning your own `Iter<'_, K, V>` over `(&K, &V)` pairs in any
  order, plus `IntoIterator for &ChainedMap` so `for (k, v) in &map` works

Hash keys with `std::hash::RandomState` (one per map, created in `new`) and
its `BuildHasher::hash_one`.

## Restrictions

- **No `HashMap`, `HashSet`, `BTreeMap` or `BTreeSet`.** Buckets are a
  `Vec<Vec<(K, V)>>`.
- Safe Rust only: no `unsafe`.

## Signature

```rust
pub struct ChainedMap<K, V> { /* your fields */ }

impl<K: Hash + Eq, V> ChainedMap<K, V> {
    pub fn new() -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn insert(&mut self, key: K, value: V) -> Option<V>;
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where K: Borrow<Q>, Q: Hash + Eq + ?Sized;
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where K: Borrow<Q>, Q: Hash + Eq + ?Sized;
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where K: Borrow<Q>, Q: Hash + Eq + ?Sized;
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where K: Borrow<Q>, Q: Hash + Eq + ?Sized;
    pub fn get_or_insert_with<F: FnOnce() -> V>(&mut self, key: K, default: F) -> &mut V;
    pub fn iter(&self) -> Iter<'_, K, V>;
}

pub struct Iter<'a, K, V> { /* your fields */ }
impl<'a, K, V> Iterator for Iter<'a, K, V> { type Item = (&'a K, &'a V); /* … */ }
```

Lookups take `&Q` where `K: Borrow<Q>`, exactly like std's `HashMap`: a
`ChainedMap<String, V>` can be queried with a `&str`, without allocating a
`String`. This works because `Borrow` promises that a key and its borrowed
form hash and compare identically. Keys and values need no other bounds — not
`Clone`, not `Default`.

## Examples

```rust
let mut m = ChainedMap::new();
assert!(m.is_empty());
assert_eq!(m.insert("apple", 3), None);
assert_eq!(m.insert("pear", 5), None);
assert_eq!(m.insert("apple", 4), Some(3));   // overwrite: old value comes back
assert_eq!(m.len(), 2);
assert_eq!(m.get("apple"), Some(&4));
assert_eq!(m.get("plum"), None);
*m.get_mut("pear").unwrap() += 1;
assert_eq!(m.get("pear"), Some(&6));
assert_eq!(m.remove("apple"), Some(4));
assert!(!m.contains_key("apple"));

let mut counts: ChainedMap<String, usize> = ChainedMap::new();
for word in "to be or not to be".split(' ') {
    *counts.get_or_insert_with(word.to_string(), || 0) += 1;
}
assert_eq!(counts.get("be"), Some(&2));     // looked up by &str
assert_eq!(counts.len(), 4);
```

## Constraints

- Up to `2 * 10^5` keys in one map; every operation must stay `O(1)` expected,
  so the bucket array has to grow with the map.
- Distinct keys may have equal hashes; equality decides, not the hash.
- Every value is dropped exactly once.

## Rust notes

- `self.hasher.hash_one(key)` hashes anything `Hash` — including a `&Q` — to a
  `u64`. Keep the `RandomState` in the struct: a fresh one hashes differently.
- `(0..n).map(|_| Vec::new()).collect()` builds `n` empty buckets without
  needing `K: Clone` (which `vec![Vec::new(); n]` would).
- `std::mem::replace` swaps the bucket array out of `&mut self` so you can
  drain the old one into the new one by value.
- Inside a bucket the order of pairs doesn't matter, so `Vec::swap_remove`
  deletes in `O(1)`.
- `Iter` can wrap `std::iter::Flatten<std::slice::Iter<'a, Vec<(K, V)>>>` —
  that's exactly what `self.buckets.iter().flatten()` returns.
