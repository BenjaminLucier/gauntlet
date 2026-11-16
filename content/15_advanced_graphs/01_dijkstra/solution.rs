// Dijkstra's Algorithm — reference solution.
// Lazy-deletion Dijkstra: a min-heap of (distance, vertex) entries, skipping any
// popped entry that's worse than the vertex's current distance. Sums use
// checked_add, so an overflowing path is never relaxed. O((V + E) log V) time,
// O(V + E) space; shortest_path also records each vertex's parent.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Distances from `source`, plus the predecessor of each reached vertex on a
/// shortest path (`None` for the source and for unreached vertices).
fn run(adj: &[Vec<(usize, u64)>], source: usize) -> (Vec<Option<u64>>, Vec<Option<usize>>) {
    let mut dist: Vec<Option<u64>> = vec![None; adj.len()];
    let mut parent = vec![None; adj.len()];
    let mut heap = BinaryHeap::from([Reverse((0u64, source))]);
    dist[source] = Some(0);
    while let Some(Reverse((d, v))) = heap.pop() {
        if dist[v].is_some_and(|best| d > best) {
            continue; // stale: v was already settled at a smaller distance
        }
        for &(w, weight) in &adj[v] {
            // An overflowing sum is not a path at all, so it relaxes nothing.
            let Some(nd) = d.checked_add(weight) else {
                continue;
            };
            if dist[w].is_none_or(|old| nd < old) {
                dist[w] = Some(nd);
                parent[w] = Some(v);
                heap.push(Reverse((nd, w)));
            }
        }
    }
    (dist, parent)
}

pub fn dijkstra(adj: &[Vec<(usize, u64)>], source: usize) -> Vec<Option<u64>> {
    run(adj, source).0
}

pub fn shortest_path(adj: &[Vec<(usize, u64)>], s: usize, t: usize) -> Option<(u64, Vec<usize>)> {
    let (dist, parent) = run(adj, s);
    let total = dist[t]?;
    let mut path: Vec<usize> = std::iter::successors(Some(t), |&v| {
        (v != s).then(|| parent[v].expect("reached vertices have a parent"))
    })
    .collect();
    path.reverse();
    Some((total, path))
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
