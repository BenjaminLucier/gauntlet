## Description

Breadth-first and depth-first search are the two ways to walk a graph, and
almost every other graph algorithm is one of them with extra bookkeeping.
Implement both traversals and three classic uses.

The first three functions take a **directed** graph as adjacency lists:
`adj[v]` is the list of `v`'s out-neighbors, in the order to try them.

- `bfs_order(adj, start)`: the vertices reachable from `start`, in the order a
  breadth-first search dequeues them. Each vertex's neighbors are enqueued in
  `adj[v]` order.
- `dfs_order(adj, start)`: the vertices reachable from `start` in DFS
  *preorder* — exactly the order this recursive search first visits them:

  ```text
  visit(v): mark v, output v, then for each w in adj[v] in order: if w unmarked, visit(w)
  ```

  …but **without recursion**: a path of 200 000 vertices would overflow the
  call stack. Use an explicit stack and produce the same order.
- `shortest_path_unweighted(adj, s, t)`: a path `[s, …, t]` with the fewest
  edges, or `None` if `t` is unreachable. When several shortest paths exist,
  any one is accepted. The path from `s` to itself is `[s]`.

The last two take an **undirected** graph as a vertex count and an edge list.

- `connected_components(n, edges)`: a component id for every vertex. Ids are
  `0, 1, 2, …` in order of each component's lowest vertex — so vertex `0` is
  always in component `0`, and the first vertex not connected to it is in
  component `1`.
- `is_bipartite(n, edges)`: whether the vertices can be coloured with two
  colours so that every edge joins different colours. A self-loop makes that
  impossible; isolated vertices are fine.

## Restrictions

- `dfs_order` must not recurse — use a `Vec` as an explicit stack.

## Signature

```rust
pub fn bfs_order(adj: &[Vec<usize>], start: usize) -> Vec<usize>;
pub fn dfs_order(adj: &[Vec<usize>], start: usize) -> Vec<usize>;
pub fn shortest_path_unweighted(adj: &[Vec<usize>], s: usize, t: usize) -> Option<Vec<usize>>;
pub fn connected_components(n: usize, edges: &[(usize, usize)]) -> Vec<usize>;
pub fn is_bipartite(n: usize, edges: &[(usize, usize)]) -> bool;
```

Adjacency lists are borrowed as `&[Vec<usize>]`, so a `Vec<Vec<usize>>` or an
array of `Vec`s both work. "No path" is `None`, never an empty `Vec`.

## Examples

```rust
//   0 → 1 → 3
//   ↓   ↓
//   2 → 4      5 (unreachable)
let adj = vec![vec![1, 2], vec![3, 4], vec![4], vec![], vec![], vec![0]];
assert_eq!(bfs_order(&adj, 0), [0, 1, 2, 3, 4]);
assert_eq!(dfs_order(&adj, 0), [0, 1, 3, 4, 2]);
assert_eq!(shortest_path_unweighted(&adj, 0, 4).map(|p| p.len()), Some(3)); // 0-1-4 or 0-2-4
assert_eq!(shortest_path_unweighted(&adj, 0, 5), None);
assert_eq!(shortest_path_unweighted(&adj, 5, 3), Some(vec![5, 0, 1, 3]));

// Components {0, 3}, {1, 2}, {4}.
assert_eq!(connected_components(5, &[(2, 1), (3, 0)]), [0, 1, 1, 0, 2]);

assert!(is_bipartite(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]));   // even cycle
assert!(!is_bipartite(3, &[(0, 1), (1, 2), (2, 0)]));          // odd cycle
```

## Constraints

- `1 <= V <= 2 * 10^5`, up to `10^6` edges; `start`, `s`, `t` and every edge
  endpoint are `< V`.
- Graphs may contain self-loops, parallel edges and cycles.

## Rust notes

- `std::collections::VecDeque` is the queue: `push_back` / `pop_front`.
- `while let Some((v, next)) = stack.last_mut()` binds `&mut` references into
  the top frame, so you can advance its neighbor index in place.
- `std::iter::successors(Some(t), |&v| parent[v])` walks a parent chain;
  collect it and `reverse()`.
- A recursive DFS on a 200 000-vertex path overflows the stack and aborts the
  whole test run — you'll see `has overflowed its stack`, not a test failure.
