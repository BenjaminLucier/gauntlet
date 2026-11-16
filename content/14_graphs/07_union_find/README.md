## Description

A **union-find** (disjoint-set) structure tracks a partition of the elements
`0..n` into sets, starting with every element alone, under two operations:
*merge the sets containing `a` and `b`* and *which set is `x` in?* It's the
engine behind Kruskal's minimum spanning tree, connectivity under edge
insertions, and equivalence classes in general.

Build `UnionFind` with both standard optimizations — **union by size** and
**path compression** — so that a million operations run in near-linear time.

- `new(n)`: `n` singleton sets.
- `find(x)`: the *representative* (root) of `x`'s set. Two elements are in the
  same set exactly when they have the same representative. Which element
  becomes the representative is up to you. Takes `&mut self` because it
  compresses paths as it goes.
- `union(a, b)`: merge the two sets; returns `true` if they were different
  sets, `false` if `a` and `b` were already together (nothing changes).
- `connected(a, b)`: same set?
- `component_size(x)`: number of elements in `x`'s set.
- `count()`: number of disjoint sets.
- `len()` / `is_empty()`: the number of elements `n`.

Elements `>= n` are a caller bug: every method may panic on them.

## Restrictions

- No `HashMap`, `HashSet` or `BTreeMap`: plain `Vec<usize>` arrays indexed by
  element.
- `find` must not recurse — a long parent chain must not overflow the stack.
- Use **both** path compression and union by size (or rank). This one is on
  the honor system: either technique alone already brings every operation
  down to `O(log n)` amortized, which is fast enough that no test can tell it
  apart from using both. Using neither still passes, but the long chains in
  the tests make it visibly slow.

## Signature

```rust
pub struct UnionFind { /* your fields */ }

impl UnionFind {
    pub fn new(n: usize) -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn find(&mut self, x: usize) -> usize;
    pub fn union(&mut self, a: usize, b: usize) -> bool;
    pub fn connected(&mut self, a: usize, b: usize) -> bool;
    pub fn component_size(&mut self, x: usize) -> usize;
    pub fn count(&self) -> usize;
}
```

Every query that calls `find` takes `&mut self`: path compression rewrites the
parent array even while "only reading". (Hiding that behind `&self` would need
`Cell`s — not worth it here.)

## Examples

```rust
let mut uf = UnionFind::new(6);
assert_eq!(uf.count(), 6);
assert!(uf.union(0, 1));
assert!(uf.union(2, 3));
assert!(uf.union(1, 3));          // {0, 1, 2, 3}  {4}  {5}
assert!(!uf.union(0, 2));         // already together
assert!(uf.connected(0, 3));
assert!(!uf.connected(0, 4));
assert_eq!(uf.component_size(2), 4);
assert_eq!(uf.component_size(5), 1);
assert_eq!(uf.count(), 3);
assert_eq!(uf.find(1), uf.find(2));
assert_eq!(uf.len(), 6);
```

## Constraints

- `0 <= n <= 10^6`, up to `10^6` operations.
- Without either optimization, adversarial union orders build chains of
  length `n`, and each `find` on them costs `O(n)`.

## Rust notes

- `find` needs the root before it can compress; path halving
  (`parent[x] = parent[parent[x]]` as you climb) does both in one loop.
- `union(a, b)`: find both roots first, then compare sizes; `std::mem::swap`
  on the two roots lets you always hang `small` under `large`.
- Only a root's `size` entry needs to be correct.
- `connected(&mut self, …)` can't be called while you hold a `&` borrow of the
  same `UnionFind`; finish with one call before starting the next.
