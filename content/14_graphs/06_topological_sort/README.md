## Description

A **topological order** of a directed graph lists every vertex once so that
each edge `(u, v)` points forward: `u` comes before `v`. One exists exactly
when the graph has no directed cycle (it's a *DAG*). Implement both classic
algorithms, and make each one *prove* its failure by returning an actual
cycle.

- `topo_sort_kahn(n, edges)` — Kahn's algorithm, peeling off vertices with no
  remaining incoming edges. Among the vertices ready at each step it always
  takes the **smallest**, so the result is the lexicographically smallest
  topological order — exactly one correct answer.
- `topo_sort_dfs(n, edges)` — depth-first search, emitting vertices in reverse
  finishing order. Any valid topological order is accepted.

Both return `Err(Cycle { vertices })` when the graph isn't a DAG. `vertices`
is a directed cycle `[v0, v1, …, vk]`: distinct vertices with edges
`v0 → v1 → … → vk → v0`. A self-loop `(v, v)` is the cycle `[v]`. Any cycle in
the graph is accepted, starting at any of its vertices.

Edges may repeat; a repeated edge is a second, identical constraint.

## Signature

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycle {
    pub vertices: Vec<usize>,
}

pub fn topo_sort_kahn(n: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, Cycle>;
pub fn topo_sort_dfs(n: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, Cycle>;
```

Failure is a `Result::Err` carrying evidence rather than an empty `Vec` or a
`bool`: a caller who gets `Err` can show the user *which* dependencies loop.

## Examples

```rust
//  3 → 1 → 0
//  ↓       ↑
//  2 ──────┘      (ready at the start: 3 only)
let edges = [(3, 1), (1, 0), (3, 2), (2, 0)];
assert_eq!(topo_sort_kahn(4, &edges), Ok(vec![3, 1, 2, 0]));
let order = topo_sort_dfs(4, &edges).unwrap();   // [3, 2, 1, 0] is fine too
assert_eq!(order.len(), 4);

// No edges: Kahn returns the identity order.
assert_eq!(topo_sort_kahn(3, &[]), Ok(vec![0, 1, 2]));

// 0 → 1 → 2 → 0 is a cycle; 3 hangs off it.
let edges = [(0, 1), (1, 2), (2, 0), (2, 3)];
let Err(Cycle { vertices }) = topo_sort_kahn(4, &edges) else { panic!() };
assert_eq!(vertices.len(), 3);              // [0, 1, 2], [1, 2, 0] or [2, 0, 1]
assert!(topo_sort_dfs(4, &edges).is_err());

assert_eq!(topo_sort_dfs(2, &[(1, 1)]), Err(Cycle { vertices: vec![1] }));
```

## Constraints

- `0 <= n <= 2 * 10^5`, up to `10^6` edges, every endpoint `< n`.
- Dependency chains can be `2 * 10^5` vertices deep. A recursive DFS is
  allowed; the deep tests run on a thread with a large stack.

## Rust notes

- `BinaryHeap` is a max-heap; `BinaryHeap<Reverse<usize>>` pops the smallest
  vertex. `while let Some(Reverse(v)) = heap.pop()` unwraps it in the pattern.
- A small `#[derive(Clone, Copy, PartialEq, Eq)] enum Colour { White, Grey,
  Black }` reads better than magic numbers.
- An iterative DFS keeps `(vertex, next neighbor index)` frames on a `Vec`;
  that `Vec` is also exactly the current grey path, which makes the cycle a
  slice of it.
- `slice::reverse` / `Vec::split_off` are handy when turning a walked loop
  into the cycle's vertex list.
