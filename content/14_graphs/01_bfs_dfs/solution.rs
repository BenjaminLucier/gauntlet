// BFS and DFS — reference solution.
// BFS with a VecDeque, marking vertices when they're enqueued; DFS with a stack
// of (vertex, next neighbor index) frames, which replays recursive preorder
// exactly. Components and bipartiteness are BFS from every unseen vertex.
// Everything is O(V + E) time and O(V) extra space.

use std::collections::VecDeque;

pub fn bfs_order(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut seen = vec![false; adj.len()];
    let mut order = Vec::new();
    let mut queue = VecDeque::from([start]);
    seen[start] = true;
    while let Some(v) = queue.pop_front() {
        order.push(v);
        for &w in &adj[v] {
            if !seen[w] {
                seen[w] = true;
                queue.push_back(w);
            }
        }
    }
    order
}

pub fn dfs_order(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut seen = vec![false; adj.len()];
    let mut order = vec![start];
    seen[start] = true;
    // Each frame is a vertex and the index of the next neighbor to try.
    let mut stack = vec![(start, 0)];
    while let Some((v, next)) = stack.last_mut() {
        match adj[*v].get(*next) {
            Some(&w) => {
                *next += 1;
                if !seen[w] {
                    seen[w] = true;
                    order.push(w);
                    stack.push((w, 0));
                }
            }
            None => {
                stack.pop();
            }
        }
    }
    order
}

pub fn shortest_path_unweighted(adj: &[Vec<usize>], s: usize, t: usize) -> Option<Vec<usize>> {
    // parent[v] = Some(u) once v is discovered from u; s is its own parent.
    let mut parent: Vec<Option<usize>> = vec![None; adj.len()];
    parent[s] = Some(s);
    let mut queue = VecDeque::from([s]);
    while let Some(v) = queue.pop_front() {
        if v == t {
            break;
        }
        for &w in &adj[v] {
            if parent[w].is_none() {
                parent[w] = Some(v);
                queue.push_back(w);
            }
        }
    }
    parent[t]?;
    let mut path: Vec<usize> =
        std::iter::successors(Some(t), |&v| (v != s).then(|| parent[v].expect("on the BFS tree")))
            .collect();
    path.reverse();
    Some(path)
}

fn undirected_adjacency(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    adj
}

pub fn connected_components(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let adj = undirected_adjacency(n, edges);
    let mut comp: Vec<Option<usize>> = vec![None; n];
    let mut next_id = 0;
    let mut queue = VecDeque::new();
    for root in 0..n {
        if comp[root].is_some() {
            continue;
        }
        comp[root] = Some(next_id);
        queue.push_back(root);
        while let Some(v) = queue.pop_front() {
            for &w in &adj[v] {
                if comp[w].is_none() {
                    comp[w] = Some(next_id);
                    queue.push_back(w);
                }
            }
        }
        next_id += 1;
    }
    comp.into_iter()
        .map(|c| c.expect("every vertex is labelled by its root's BFS"))
        .collect()
}

pub fn is_bipartite(n: usize, edges: &[(usize, usize)]) -> bool {
    let adj = undirected_adjacency(n, edges);
    let mut colour: Vec<Option<bool>> = vec![None; n];
    let mut queue = VecDeque::new();
    for root in 0..n {
        if colour[root].is_some() {
            continue;
        }
        colour[root] = Some(false);
        queue.push_back(root);
        while let Some(v) = queue.pop_front() {
            let c = colour[v].expect("queued vertices are coloured");
            for &w in &adj[v] {
                match colour[w] {
                    None => {
                        colour[w] = Some(!c);
                        queue.push_back(w);
                    }
                    Some(cw) if cw == c => return false,
                    Some(_) => {}
                }
            }
        }
    }
    true
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
