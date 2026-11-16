// Tests for BFS and DFS — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn random_edges(rng: &mut Rng, n: usize, m: usize) -> Vec<(usize, usize)> {
    (0..m)
        .map(|_| (rng.below(n as u64) as usize, rng.below(n as u64) as usize))
        .collect()
}

fn directed(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj[u].push(v);
    }
    adj
}

/// The recursive DFS the statement describes (only used on small graphs).
fn brute_dfs(adj: &[Vec<usize>], v: usize, seen: &mut [bool], out: &mut Vec<usize>) {
    seen[v] = true;
    out.push(v);
    for &w in &adj[v] {
        if !seen[w] {
            brute_dfs(adj, w, seen, out);
        }
    }
}

/// Edge-count distances by repeated relaxation: O(V * E), fine for small graphs.
fn brute_distances(adj: &[Vec<usize>], s: usize) -> Vec<Option<usize>> {
    let mut dist = vec![None; adj.len()];
    dist[s] = Some(0);
    for _ in 0..adj.len() {
        for u in 0..adj.len() {
            let Some(du) = dist[u] else { continue };
            for &w in &adj[u] {
                if dist[w].is_none_or(|dw| du + 1 < dw) {
                    dist[w] = Some(du + 1);
                }
            }
        }
    }
    dist
}

/// Label propagation until nothing changes, then renumber by lowest vertex.
fn brute_components(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut label: Vec<usize> = (0..n).collect();
    let mut changed = true;
    while changed {
        changed = false;
        for &(u, v) in edges {
            let m = label[u].min(label[v]);
            if label[u] != m || label[v] != m {
                label[u] = m;
                label[v] = m;
                changed = true;
            }
        }
    }
    // label[v] is now the lowest vertex of v's component.
    let mut id = vec![usize::MAX; n];
    let mut next = 0;
    for v in 0..n {
        if label[v] == v {
            id[v] = next;
            next += 1;
        }
    }
    (0..n).map(|v| id[label[v]]).collect()
}

/// Try all 2^n colourings.
fn brute_bipartite(n: usize, edges: &[(usize, usize)]) -> bool {
    (0u32..1 << n).any(|mask| edges.iter().all(|&(u, v)| (mask >> u & 1) != (mask >> v & 1)))
}

fn assert_valid_path(adj: &[Vec<usize>], path: &[usize], s: usize, t: usize, len: usize) {
    assert_eq!(path.first(), Some(&s), "path {path:?} must start at {s}");
    assert_eq!(path.last(), Some(&t), "path {path:?} must end at {t}");
    assert_eq!(path.len(), len + 1, "path {path:?} isn't a shortest one");
    for pair in path.windows(2) {
        assert!(adj[pair[0]].contains(&pair[1]), "{pair:?} in {path:?} isn't an edge");
    }
}

#[test]
fn readme_example() {
    let adj = vec![vec![1, 2], vec![3, 4], vec![4], vec![], vec![], vec![0]];
    assert_eq!(bfs_order(&adj, 0), [0, 1, 2, 3, 4]);
    assert_eq!(dfs_order(&adj, 0), [0, 1, 3, 4, 2]);
    let p = shortest_path_unweighted(&adj, 0, 4).expect("4 is reachable");
    assert_valid_path(&adj, &p, 0, 4, 2);
    assert_eq!(shortest_path_unweighted(&adj, 0, 5), None);
    assert_eq!(shortest_path_unweighted(&adj, 5, 3), Some(vec![5, 0, 1, 3]));
    assert_eq!(connected_components(5, &[(2, 1), (3, 0)]), [0, 1, 1, 0, 2]);
    assert!(is_bipartite(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]));
    assert!(!is_bipartite(3, &[(0, 1), (1, 2), (2, 0)]));
}

#[test]
fn single_vertex() {
    let adj = vec![vec![]];
    assert_eq!(bfs_order(&adj, 0), [0]);
    assert_eq!(dfs_order(&adj, 0), [0]);
    assert_eq!(shortest_path_unweighted(&adj, 0, 0), Some(vec![0]));
    assert_eq!(connected_components(1, &[]), [0]);
    assert!(is_bipartite(1, &[]));
}

#[test]
fn traversals_follow_list_order_not_vertex_numbers() {
    let adj = vec![vec![3, 1, 2], vec![], vec![], vec![2]];
    assert_eq!(bfs_order(&adj, 0), [0, 3, 1, 2]);
    assert_eq!(dfs_order(&adj, 0), [0, 3, 2, 1]);
}

#[test]
fn traversals_only_follow_edges_forwards() {
    let adj = vec![vec![1], vec![2], vec![], vec![1]]; // 3 → 1 but not 1 → 3
    assert_eq!(bfs_order(&adj, 1), [1, 2]);
    assert_eq!(dfs_order(&adj, 1), [1, 2]);
    assert_eq!(bfs_order(&adj, 3), [3, 1, 2]);
    assert_eq!(shortest_path_unweighted(&adj, 1, 3), None);
}

#[test]
fn cycles_self_loops_and_parallel_edges_are_visited_once() {
    let adj = vec![vec![0, 1, 1, 2], vec![0, 2], vec![2, 1, 0]];
    assert_eq!(bfs_order(&adj, 0), [0, 1, 2]);
    assert_eq!(dfs_order(&adj, 0), [0, 1, 2]);
    assert_eq!(bfs_order(&adj, 2), [2, 1, 0]);
    assert_eq!(dfs_order(&adj, 2), [2, 1, 0]);
}

#[test]
fn dfs_is_not_bfs_with_a_stack() {
    // Pushing all of a vertex's neighbors at once and marking them on push
    // yields 0, 1, 2, 5, 3: vertex 3 gets claimed by 0 before 2 can reach it.
    let adj = vec![vec![1, 3], vec![2], vec![3, 5], vec![], vec![], vec![]];
    assert_eq!(dfs_order(&adj, 0), [0, 1, 2, 3, 5]);
    assert_eq!(bfs_order(&adj, 0), [0, 1, 3, 2, 5]);
}

#[test]
fn dfs_backtracks_to_the_right_neighbor() {
    //      0
    //    / | \
    //   1  4  6
    //  / \  \
    // 2   3  5
    let adj = vec![vec![1, 4, 6], vec![2, 3], vec![], vec![], vec![5], vec![], vec![]];
    assert_eq!(dfs_order(&adj, 0), [0, 1, 2, 3, 4, 5, 6]);
    assert_eq!(bfs_order(&adj, 0), [0, 1, 4, 6, 2, 3, 5]);
}

#[test]
fn shortest_path_prefers_fewer_edges_over_list_order() {
    // The first neighbor tried leads down a long detour.
    let adj = vec![vec![1, 4], vec![2], vec![3], vec![5], vec![5], vec![]];
    assert_eq!(shortest_path_unweighted(&adj, 0, 5), Some(vec![0, 4, 5]));
    assert_eq!(dfs_order(&adj, 0), [0, 1, 2, 3, 5, 4]);
}

#[test]
fn shortest_path_to_self_is_just_the_vertex() {
    let adj = vec![vec![1], vec![0]];
    assert_eq!(shortest_path_unweighted(&adj, 1, 1), Some(vec![1]));
}

#[test]
fn components_without_edges_are_singletons() {
    assert_eq!(connected_components(4, &[]), [0, 1, 2, 3]);
    assert_eq!(connected_components(3, &[(1, 1), (2, 2)]), [0, 1, 2]);
}

#[test]
fn components_are_numbered_by_lowest_vertex() {
    // The edge to vertex 5 comes first, but {0, 4} still gets id 0.
    let edges = [(5, 3), (4, 0), (2, 5), (1, 6)];
    assert_eq!(connected_components(7, &edges), [0, 1, 2, 2, 0, 2, 1]);
}

#[test]
fn bipartite_edge_cases() {
    assert!(is_bipartite(3, &[]), "no edges");
    assert!(!is_bipartite(2, &[(1, 1)]), "a self-loop joins a vertex to itself");
    assert!(is_bipartite(2, &[(0, 1), (1, 0), (0, 1)]), "parallel edges");
    // The odd cycle lives in a component that doesn't contain vertex 0.
    assert!(!is_bipartite(6, &[(0, 1), (2, 3), (3, 4), (4, 2)]));
    // A path explored in an order that reaches a vertex from both sides.
    assert!(is_bipartite(5, &[(4, 3), (0, 1), (2, 3), (1, 2)]));
}

#[test]
fn random_traversals_match_recursive_and_relaxation_oracles() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for _ in 0..300 {
        let n = 1 + rng.below(10) as usize;
        let m = rng.below(25) as usize;
        let edges = random_edges(&mut rng, n, m);
        let adj = directed(n, &edges);
        let s = rng.below(n as u64) as usize;
        let msg = format!("adj = {adj:?}, start = {s}");

        let mut expected = Vec::new();
        brute_dfs(&adj, s, &mut vec![false; n], &mut expected);
        assert_eq!(dfs_order(&adj, s), expected, "{msg}");

        let dist = brute_distances(&adj, s);
        let order = bfs_order(&adj, s);
        assert_eq!(order.first(), Some(&s), "{msg}");
        let mut sorted = order.clone();
        sorted.sort_unstable();
        let reachable: Vec<usize> = (0..n).filter(|&v| dist[v].is_some()).collect();
        assert_eq!(sorted, reachable, "{msg}: BFS must visit each reachable vertex once");
        let levels: Vec<usize> = order.iter().map(|&v| dist[v].expect("reachable")).collect();
        assert!(levels.is_sorted(), "{msg}: BFS order {order:?} has levels {levels:?}");

        for (t, &d) in dist.iter().enumerate() {
            match (shortest_path_unweighted(&adj, s, t), d) {
                (Some(path), Some(d)) => assert_valid_path(&adj, &path, s, t, d),
                (None, None) => {}
                (got, d) => panic!("{msg}: path to {t} = {got:?}, distance {d:?}"),
            }
        }
    }
}

#[test]
fn random_components_and_bipartiteness_match_brute_force() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..400 {
        let n = 1 + rng.below(10) as usize;
        let m = rng.below(12) as usize;
        let edges = random_edges(&mut rng, n, m);
        let msg = format!("n = {n}, edges = {edges:?}");
        assert_eq!(connected_components(n, &edges), brute_components(n, &edges), "{msg}");
        assert_eq!(is_bipartite(n, &edges), brute_bipartite(n, &edges), "{msg}");
    }
}

#[test]
fn long_path_does_not_overflow_the_stack() {
    // A recursive DFS would need 200 000 nested calls.
    let n = 200_000;
    let adj: Vec<Vec<usize>> = (0..n).map(|v| if v + 1 < n { vec![v + 1] } else { vec![] }).collect();
    let expected: Vec<usize> = (0..n).collect();
    assert_eq!(dfs_order(&adj, 0), expected);
    assert_eq!(bfs_order(&adj, 0), expected);
    let path = shortest_path_unweighted(&adj, 0, n - 1).expect("end of the path");
    assert_eq!(path, expected);
    let edges: Vec<(usize, usize)> = (1..n).map(|v| (v - 1, v)).collect();
    assert_eq!(connected_components(n, &edges), vec![0; n]);
    assert!(is_bipartite(n, &edges));
}

#[test]
fn deep_dfs_with_back_edges() {
    // Every vertex first points back at 0, then onward: the stack must
    // resume each frame at the right neighbor, 200 000 frames deep.
    let n = 200_000;
    let adj: Vec<Vec<usize>> = (0..n)
        .map(|v| if v + 1 < n { vec![0, v + 1] } else { vec![0] })
        .collect();
    let expected: Vec<usize> = (0..n).collect();
    assert_eq!(dfs_order(&adj, 0), expected);
}

#[test]
fn large_random_graph() {
    let n = 50_000;
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);
    let edges = random_edges(&mut rng, n, 2 * n);
    let adj = directed(n, &edges);
    let order = bfs_order(&adj, 0);
    let mut seen = vec![false; n];
    for &v in &order {
        assert!(!seen[v], "{v} visited twice");
        seen[v] = true;
    }
    let mut dfs = dfs_order(&adj, 0);
    dfs.sort_unstable();
    let mut bfs = order;
    bfs.sort_unstable();
    assert_eq!(dfs, bfs, "BFS and DFS reach the same vertices");
    let comp = connected_components(n, &edges);
    for &(u, v) in &edges {
        assert_eq!(comp[u], comp[v]);
    }
    assert_eq!(comp[0], 0);
    // Odd cycles are everywhere in a random graph with 2 edges per vertex.
    assert!(!is_bipartite(n, &edges));
}

#[test]
fn star_with_a_wide_frontier() {
    // A star: the center's 1.5 * 10^5 - 1 neighbors are all queued at once.
    // `Vec::remove(0)` as the queue's pop shifts the whole frontier every
    // time, ~10^10 element moves per traversal; `VecDeque::pop_front` is O(1).
    let n = 150_000;
    let center = n / 3;
    let leaves: Vec<usize> = (0..n).filter(|&v| v != center).collect();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    adj[center] = leaves.clone();
    let order = bfs_order(&adj, center);
    assert!(order[0] == center && order[1..] == leaves[..], "star with center {center}, n = {n}");
    let last = leaves[n - 2];
    let path = shortest_path_unweighted(&adj, center, last);
    assert_eq!(path, Some(vec![center, last]), "star with center {center}, n = {n}");
}
