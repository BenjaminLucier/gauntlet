## Description

A tree on nodes `0..n` had **one extra** undirected edge added between two
distinct nodes, giving exactly `n` edges. You receive those edges as `edges`,
where `(a, b)` connects `a` and `b`.

Return an edge that can be removed so the remaining `n - 1` edges form a tree.
If there are several such edges, return the one that appears **last** in the
input.

If the edges contain no cycle at all (the input is already a forest), return
`None`.

## Signature

```rust
pub fn find_redundant_connection(edges: &[(usize, usize)]) -> Option<(usize, usize)>
```

Edges are `(usize, usize)` pairs borrowed from the caller, and the answer is one
of those pairs, copied out. The puzzle guarantees a redundant edge, but the
function can't enforce that through its types — so instead of panicking or
inventing a sentinel on bad input it returns `Option`.

## Examples

```rust
assert_eq!(find_redundant_connection(&[(0, 1), (0, 2), (1, 2)]), Some((1, 2)));
// Removing any edge of the cycle 0-1-2-3-0 leaves a tree; (0, 3) is listed last.
assert_eq!(
    find_redundant_connection(&[(0, 1), (1, 2), (2, 3), (0, 3), (0, 4)]),
    Some((0, 3))
);
// Already a tree: nothing is redundant.
assert_eq!(find_redundant_connection(&[(0, 1), (1, 2)]), None);
```

## Constraints

- `n == edges.len()`, `3 <= n <= 1000` (for inputs that do have a cycle)
- `0 <= a < b < n`
- No repeated edges; the graph is connected

## Rust notes

- `n` isn't passed in, but it's `edges.len()` — or take the largest label
  plus one, which also copes with the cycle-free inputs that return `None`.
- Union-find reads well as a small struct whose `union(&mut self, a, b) -> bool`
  reports whether the two were in different sets.
- Iterate with `edges.iter().copied()` to get owned `(usize, usize)` pairs, and
  `Iterator::find` returns exactly the `Option` you need.
- Keep `find` iterative (path halving) so a long chain can't overflow the stack.
