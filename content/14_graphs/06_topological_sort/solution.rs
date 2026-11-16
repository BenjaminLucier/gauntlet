// Topological Sort — reference solution.
// Kahn: in-degrees plus a min-heap of ready vertices, O(V log V + E); if it
// stalls, walk leftover predecessors until a vertex repeats to get a cycle.
// DFS: iterative three-colour search, O(V + E); the frame stack is the grey path,
// so an edge into a grey vertex yields the cycle as a slice of it.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// A directed cycle: edges `v0 → v1 → … → vk → v0`, all vertices distinct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycle {
    pub vertices: Vec<usize>,
}

fn adjacency(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj[u].push(v);
    }
    adj
}

pub fn topo_sort_kahn(n: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, Cycle> {
    let adj = adjacency(n, edges);
    let mut in_degree = vec![0usize; n];
    for &(_, v) in edges {
        in_degree[v] += 1;
    }
    let mut ready: BinaryHeap<Reverse<usize>> = (0..n)
        .filter(|&v| in_degree[v] == 0)
        .map(Reverse)
        .collect();
    let mut order = Vec::with_capacity(n);
    while let Some(Reverse(u)) = ready.pop() {
        order.push(u);
        for &v in &adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                ready.push(Reverse(v));
            }
        }
    }
    if order.len() == n {
        Ok(order)
    } else {
        Err(leftover_cycle(n, edges, &in_degree))
    }
}

/// Every vertex Kahn couldn't output still has an edge from another such
/// vertex. Following those edges backwards must eventually repeat a vertex.
fn leftover_cycle(n: usize, edges: &[(usize, usize)], in_degree: &[usize]) -> Cycle {
    let stuck = |v: usize| in_degree[v] > 0;
    let mut pred = vec![None; n];
    for &(u, v) in edges {
        if stuck(u) && stuck(v) {
            pred[v] = Some(u);
        }
    }
    let start = (0..n).find(|&v| stuck(v)).expect("Kahn stalled, so some vertex is stuck");
    let mut position = vec![None; n];
    let mut walk = Vec::new();
    let mut v = start;
    while position[v].is_none() {
        position[v] = Some(walk.len());
        walk.push(v);
        v = pred[v].expect("a stuck vertex has a stuck predecessor");
    }
    let first = position[v].expect("the loop exits on a revisit");
    let mut vertices = walk.split_off(first);
    vertices.reverse(); // the walk followed edges backwards
    Cycle { vertices }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Colour {
    White,
    Grey,
    Black,
}

pub fn topo_sort_dfs(n: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, Cycle> {
    let adj = adjacency(n, edges);
    let mut colour = vec![Colour::White; n];
    let mut postorder = Vec::with_capacity(n);
    let mut stack: Vec<(usize, usize)> = Vec::new();
    for root in 0..n {
        if colour[root] != Colour::White {
            continue;
        }
        colour[root] = Colour::Grey;
        stack.push((root, 0));
        while let Some((v, next)) = stack.last_mut() {
            let v = *v;
            let Some(&w) = adj[v].get(*next) else {
                colour[v] = Colour::Black;
                postorder.push(v);
                stack.pop();
                continue;
            };
            *next += 1;
            match colour[w] {
                Colour::White => {
                    colour[w] = Colour::Grey;
                    stack.push((w, 0));
                }
                Colour::Grey => {
                    // The stack is the grey path root → … → w → … → v.
                    let from = stack
                        .iter()
                        .position(|&(x, _)| x == w)
                        .expect("grey vertices are on the stack");
                    let vertices = stack[from..].iter().map(|&(x, _)| x).collect();
                    return Err(Cycle { vertices });
                }
                Colour::Black => {}
            }
        }
    }
    postorder.reverse();
    Ok(postorder)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
