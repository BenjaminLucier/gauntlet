## Description

Dijkstra's algorithm computes shortest paths from one source in a graph whose
edge weights are **non-negative**. It settles vertices in order of distance:
the closest unsettled vertex can't be improved later, because every other
route to it would start further away and edges never subtract.

The graph is directed, as adjacency lists of weighted edges: `adj[v]` holds
`(w, weight)` for each edge `v → w`. Implement:

- `dijkstra(adj, source)`: for every vertex, `Some(distance)` from `source`,
  or `None` if it's unreachable. `source` itself is at distance `0`.
- `shortest_path(adj, s, t)`: `Some((distance, path))` where `path` is
  `[s, …, t]` and its edges' weights sum to `distance`; `None` if `t` is
  unreachable. When several shortest paths exist, any one is accepted. The
  path from `s` to itself is `(0, [s])`.

Weights go all the way up to `u64::MAX`, so a sum can overflow. A path whose
total length doesn't fit in a `u64` **does not count**: if every route to a
vertex is that long, the vertex is `None`. (Saturating at `u64::MAX` would be
wrong — it would report a real-looking distance that no path has.)

Graphs can have zero-weight edges, self-loops, cycles and parallel edges
(several edges `v → w` with different weights).

## Restrictions

- Use a binary heap (`BinaryHeap`), not an `O(V)` scan for the closest vertex
  and not Bellman-Ford-style repeated relaxation.

## Signature

```rust
pub fn dijkstra(adj: &[Vec<(usize, u64)>], source: usize) -> Vec<Option<u64>>;
pub fn shortest_path(adj: &[Vec<(usize, u64)>], s: usize, t: usize) -> Option<(u64, Vec<usize>)>;
```

Distances are `u64` because weights can't be negative and sums of many large
weights need the room; "unreachable" is `None`, never `u64::MAX` or `-1`.

## Examples

```rust
// 0 →1→ 1 →2→ 2 →3→ 3, plus a direct 0 →4→ 2.
// 4 → 0 exists, but nothing leads to 4.
let adj = vec![
    vec![(2, 4), (1, 1)],
    vec![(2, 2)],
    vec![(3, 3)],
    vec![],
    vec![(0, 1)],
];
assert_eq!(dijkstra(&adj, 0), [Some(0), Some(1), Some(3), Some(6), None]);
assert_eq!(shortest_path(&adj, 0, 3), Some((6, vec![0, 1, 2, 3])));
assert_eq!(shortest_path(&adj, 0, 4), None);
assert_eq!(shortest_path(&adj, 2, 2), Some((0, vec![2])));

// Overflow: 0 → 1 costs u64::MAX, so 0 → 1 → 2 doesn't fit.
let adj = vec![vec![(1, u64::MAX)], vec![(2, 1)], vec![]];
assert_eq!(dijkstra(&adj, 0), [Some(0), Some(u64::MAX), None]);
```

## Constraints

- `1 <= V <= 10^5`, up to `5 * 10^5` edges; `0 <= weight <= u64::MAX`.
- `source`, `s`, `t` and every edge target are `< V`.

## Rust notes

- `BinaryHeap` is a max-heap; `Reverse((dist, vertex))` turns it into a
  min-heap ordered by distance, with the vertex as a tie-breaker.
- `while let Some(Reverse((d, v))) = heap.pop()` destructures in the pattern.
- `u64::checked_add` returns `None` on overflow;
  `let Some(nd) = d.checked_add(w) else { continue };` skips that edge.
- `Option<u64>` compares with `None < Some(_)`, which is the wrong way round
  for "unreachable is infinitely far" — compare explicitly, e.g. with
  `dist[w].is_none_or(|old| nd < old)`.
