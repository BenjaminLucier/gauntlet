## Description

A **minimum spanning tree** (MST) of a connected, undirected, weighted graph
is a set of `n - 1` edges that connects all `n` vertices with the smallest
possible total weight. Implement both classic algorithms:

- **Kruskal**, on an edge list: sort the edges and add each one that joins
  two different components, tracked with a union-find (disjoint set union)
  you write yourself. Return the total weight *and* the chosen edges.
- **Prim**, on adjacency lists: grow a tree from vertex `0` with a priority
  queue, always taking the cheapest edge out of the tree. Return the total
  weight.

If the graph is not connected there is no spanning tree: return `None`. A
graph with zero or one vertex is trivially connected (total `0`, no edges).

The graph may contain self-loops (never part of a tree) and parallel edges
(only the cheapest of them can matter).

## Signature

```rust
/// An undirected weighted edge `(u, v, weight)`.
pub type Edge = (usize, usize, u64);

pub fn kruskal(n: usize, edges: &[Edge]) -> Option<(u64, Vec<Edge>)>;

pub fn prim(adj: &[Vec<(usize, u64)>]) -> Option<u64>;
```

`kruskal`'s edges are copied from the input exactly as given (same endpoint
order), in any order you like. When weights tie, several different trees can be minimum; any one is
accepted, but its total must be the minimum.

`prim`'s graph has `adj.len()` vertices; `adj[u]` lists `(v, weight)` pairs.
The graph is undirected, so every edge appears in both lists (a self-loop
appears in its vertex's list). `prim` returns only the total to keep its
heap entries small: recovering the edges would mean storing each entry's
source vertex too, and the tree itself is already exercised by `kruskal`.
(Try it as an extension: push `(weight, v, from)` and record `(from, v,
weight)` when `v` joins.)

## Examples

```rust
//   0 --1-- 1
//   |     / |
//   4   2   5
//   | /     |
//   2 --3-- 3
let edges = [(0, 1, 1), (0, 2, 4), (1, 2, 2), (1, 3, 5), (2, 3, 3)];
let (total, mut tree) = kruskal(4, &edges).unwrap();
assert_eq!(total, 6);
tree.sort_unstable(); // the order of the returned edges is up to you
assert_eq!(tree, [(0, 1, 1), (1, 2, 2), (2, 3, 3)]);

let adj = vec![
    vec![(1, 1), (2, 4)],
    vec![(0, 1), (2, 2), (3, 5)],
    vec![(0, 4), (1, 2), (3, 3)],
    vec![(1, 5), (2, 3)],
];
assert_eq!(prim(&adj), Some(6));

assert_eq!(kruskal(3, &[(0, 1, 7)]), None); // vertex 2 is cut off
assert_eq!(kruskal(1, &[]), Some((0, vec![])));
assert_eq!(prim(&[]), Some(0));
```

## Constraints

- `0 <= n <= 10^5`, `0 <= edges.len() <= 2 * 10^5`
- Every endpoint is `< n`. `0 <= weight <= 10^9`.

## Rust notes

- `sort_unstable_by_key(|&(_, _, w)| w)` on a copy of the edges (`to_vec`):
  the input is borrowed, so you can't sort it in place.
- `std::cmp::Reverse` turns `BinaryHeap`'s max-heap into the min-heap Prim
  needs; tuples compare lexicographically, so put the weight first.
- A recursive `find` is fine with union by size (depth stays `O(log n)`), but
  an iterative one with path halving is just as short.
- Prim stops early once all `n` vertices are in — no need to drain the heap.
