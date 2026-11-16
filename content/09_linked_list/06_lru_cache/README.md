## Description

Design a key-value cache with a fixed `capacity` that evicts the **least
recently used** entry when it is full.

- `LruCache::new(capacity)` creates an empty cache holding at most `capacity`
  entries. A capacity of `0` is allowed: such a cache never stores anything.
- `get(&key)` returns a reference to the value stored for `key`, or `None`.
  A successful `get` counts as a *use* of that key.
- `put(key, value)` stores `value` under `key`, overwriting any existing value;
  this also counts as a use. If inserting a **new** key would exceed
  `capacity`, first evict the least recently used entry.
- `len()` / `is_empty()` report how many entries are stored.

Aim for `O(1)` average time per `get` and `put`. Looking a key up must not
compare it against the stored keys one by one: a test counts those comparisons.

## Signature

```rust
pub struct LruCache<K, V> { /* your fields */ }

impl<K: Hash + Eq + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self
    pub fn get<Q>(&mut self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
    pub fn put(&mut self, key: K, value: V)
    pub fn len(&self) -> usize
    pub fn is_empty(&self) -> bool
}
```

- Generic over keys and values. Keys must be `Hash + Eq` for the hash map, and
  `Clone` because each key is needed in two places: the map (to find an entry)
  and the entry itself (to remove it from the map when it is evicted). Values
  are stored once and never cloned — any `V` works.
- `get` returns `Option<&V>` instead of a sentinel, and lends the value rather
  than copying it. It takes `&mut self` because a lookup updates recency.
- `get` accepts any borrowed form of the key, exactly like `HashMap::get`: a
  `LruCache<String, _>` can be queried with a `&str`.

## Examples

```rust
let mut cache = LruCache::new(2);
cache.put(1, 1);                    // {1=1}
cache.put(2, 2);                    // {1=1, 2=2}
assert_eq!(cache.get(&1), Some(&1)); // key 1 is now most recently used
cache.put(3, 3);                    // full: evicts key 2 -> {1=1, 3=3}
assert_eq!(cache.get(&2), None);
cache.put(4, 4);                    // full: evicts key 1 -> {3=3, 4=4}
assert_eq!(cache.get(&1), None);
assert_eq!(cache.get(&3), Some(&3));
assert_eq!(cache.get(&4), Some(&4));
assert_eq!(cache.len(), 2);

let mut names: LruCache<String, Vec<u8>> = LruCache::new(1);
names.put("rust".to_string(), vec![1, 2]);
assert_eq!(names.get("rust"), Some(&vec![1, 2])); // look up with a &str
```

## Constraints

- `0 <= capacity <= 3000`
- At most `2 * 10^5` calls to `get` and `put`.

## Rust notes

- A pointer-based doubly linked list needs every node to have two owners —
  painful in safe Rust (`Rc<RefCell<…>>` plus `Weak` for the back links). The
  idiomatic alternative is an **arena**: keep entries in a `Vec` and link them
  with `usize` indices (`Option<usize>` for "no neighbour").
- Evicting the oldest entry frees a slot you can reuse for the new one —
  `std::mem::replace(&mut slots[i], new_entry)` hands back the old entry so you
  can remove its key from the map.
- `HashMap::get` already supports the `Borrow` pattern; just forward the bounds.
- Returning `Option<&V>` from a `&mut self` method keeps the cache mutably
  borrowed until the reference is gone; copy the value out (`.copied()`,
  `.cloned()`) if you need to call the cache again while holding it.
- `len` without `is_empty` triggers a clippy lint; implement both.
