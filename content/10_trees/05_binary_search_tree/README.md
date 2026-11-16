## Description

Build `Bst<K, V>`, an ordered map stored as a plain (unbalanced) binary search
tree of owned boxes: every key in a node's left subtree is smaller than the
node's key, every key in its right subtree is larger.

Implement:

- `new`, `len`, `is_empty`
- `insert` (returning the previous value for that key, if any), `get`,
  `get_mut`, `contains_key`
- `remove`, covering a node with no children, one child, or two children
- `min`, `max`, and `floor` / `ceiling` (the entry with the largest key
  `<=` the given one / the smallest key `>=` it)
- `iter`, an in-order iterator (keys ascending) of your own type `Iter`,
  plus `IntoIterator for &Bst` so `for (k, v) in &bst` works
- `FromIterator<(K, V)>` and `Extend<(K, V)>`, behaving like repeated
  `insert` (a later value for a key replaces an earlier one), so
  `pairs.into_iter().collect::<Bst<_, _>>()` works
- `Drop`, so that dropping a tree doesn't recurse

The tree doesn't rebalance, so keys inserted in sorted order produce a path
`n` nodes deep. That is legal, and the tests do it: **no operation may recurse
on the height** — not even `drop`, which by default recurses once per level.

## Restrictions

- No `BTreeMap`, `BTreeSet` or other ordered std collection. (A `Vec` as a
  stack is fine.)
- No recursion on the tree's height, no `unsafe`, no `Rc`/`RefCell`.

## Signature

```rust
pub struct Bst<K, V> {
    root: Link<K, V>,
    len: usize,
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

type Link<K, V> = Option<Box<Node<K, V>>>;

impl<K: Ord, V> Bst<K, V> {
    pub fn new() -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn insert(&mut self, key: K, value: V) -> Option<V>;
    // Each lookup is generic like BTreeMap's:
    //     fn get<Q>(&self, key: &Q) -> Option<&V>
    //     where K: Borrow<Q>, Q: Ord + ?Sized;
    // The same `<Q>` and `where` clause go on all six below.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>;
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>;
    pub fn contains_key<Q>(&self, key: &Q) -> bool;
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>;
    pub fn floor<Q>(&self, key: &Q) -> Option<(&K, &V)>;
    pub fn ceiling<Q>(&self, key: &Q) -> Option<(&K, &V)>;
    pub fn min(&self) -> Option<(&K, &V)>;
    pub fn max(&self) -> Option<(&K, &V)>;
    pub fn iter(&self) -> Iter<'_, K, V>;
}

impl<K: Ord, V> FromIterator<(K, V)> for Bst<K, V> { /* … */ }
impl<K: Ord, V> Extend<(K, V)> for Bst<K, V> { /* … */ }

pub struct Iter<'a, K, V> { /* your fields */ }
impl<'a, K, V> Iterator for Iter<'a, K, V> { type Item = (&'a K, &'a V); /* … */ }
```

The node layout is given: each node owns its two subtrees through a `Link`,
so the whole tree has a single owner and no `Rc` is needed. Keys only need
`Ord` — not `Clone`, not `Copy` — and values need nothing at all. `remove`
hands the value back, like `BTreeMap::remove`.

The lookups take `&Q` where `K: Borrow<Q>`, the same shape as the std maps,
so a `Bst<String, _>` can be searched with a plain `&str` (no allocating a
`String` just to look something up). `Borrow` promises that `Q`'s ordering
agrees with `K`'s, so compare `key.cmp(node.key.borrow())` everywhere.

## Examples

```rust
let mut m = Bst::new();
assert_eq!(m.insert(5, "five"), None);
m.insert(2, "two");
m.insert(8, "eight");
assert_eq!(m.insert(5, "FIVE"), Some("five")); // replaced: len stays 3
assert_eq!(m.len(), 3);
assert_eq!(m.get(&5), Some(&"FIVE"));
assert!(m.contains_key(&2));
assert_eq!(m.min(), Some((&2, &"two")));
assert_eq!(m.floor(&7), Some((&5, &"FIVE")));   // largest key <= 7
assert_eq!(m.ceiling(&9), None);                // no key >= 9
assert_eq!(m.remove(&5), Some("FIVE"));         // the root, with two children
assert_eq!(m.iter().collect::<Vec<_>>(), [(&2, &"two"), (&8, &"eight")]);

let mut names: Bst<String, u32> = [("bo".to_string(), 1)].into_iter().collect();
names.extend([("al".to_string(), 2), ("bo".to_string(), 3)]);
assert_eq!(names.get("bo"), Some(&3)); // a &str finds a String key
assert_eq!(names.len(), 2);
```

## Constraints

- Up to `2 * 10^5` operations; random keys keep the height logarithmic.
- Sorted keys make a path several thousand nodes deep; the tests run that on
  a thread with a small stack.
- Every value is dropped exactly once.

## Rust notes

- **Cursors.** A `&mut Link<K, V>` is a *cursor*: it points at a slot in the
  tree — the root field, or some node's `left` or `right` — and through it you
  can overwrite that slot: `*link = Some(new_node)` inserts, `link.take()`
  unhooks a whole subtree, `*link = child` splices a child into its parent's
  place. That is how removal works without parent pointers: walk the cursor
  down to the slot holding the key, then replace the slot's contents.
- Moving a cursor down is `link = &mut node.left;` inside a loop, where
  `node` came from `link.as_mut()`. Each step *reborrows* through the old
  cursor and then overwrites it, so the borrow checker sees one live `&mut`
  at a time.
- The trap: a loop that matches `Some(node)` on the cursor, moves it down in
  some arms and `return`s or `break`s with the cursor in another, is rejected
  by today's borrow checker — the mutable borrow of `node` is considered to
  live as long as the cursor itself. Decide first with a shared look
  (`link.as_deref()` and compare keys, keeping just the `Ordering`), and only
  then take the `&mut` you descend through.
- With `K: Borrow<Q>`, `node.key.borrow()` gives a `&Q` to compare against.
  `insert` still takes an owned `K`, and `&K` is a `&Q` with `Q = K` (every
  type borrows as itself), so one generic search helper serves all methods.
- `std::mem::replace(&mut node.value, value)` swaps a new value in and hands
  you the old one. Moving whole nodes around (rather than swapping keys) means
  `K` and `V` need no `Default` or `Clone`.
- The iterator can hold a `Vec<&'a Node<K, V>>`: references into the tree that
  live as long as the borrow `iter(&self)` took.
