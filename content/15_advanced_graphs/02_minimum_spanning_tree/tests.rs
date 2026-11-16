// Tests for Minimum Spanning Tree — shared by template.rs, solution.rs and your attempt.
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

fn adjacency(n: usize, edges: &[(usize, usize, u64)]) -> Vec<Vec<(usize, u64)>> {
    let mut adj = vec![Vec::new(); n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
        if u != v {
            adj[v].push((u, w));
        }
    }
    adj
}

/// Simple union-find without balancing, for the checks below only.
fn root(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

fn is_connected(n: usize, edges: &[(usize, usize, u64)]) -> bool {
    let mut parent: Vec<usize> = (0..n).collect();
    let mut components = n;
    for &(u, v, _) in edges {
        let (a, b) = (root(&mut parent, u), root(&mut parent, v));
        if a != b {
            parent[a] = b;
            components -= 1;
        }
    }
    components <= 1
}

/// `tree` must be `n - 1` input edges (as a multiset) that connect every
/// vertex and weigh `total`.
fn assert_spanning_tree(n: usize, edges: &[(usize, usize, u64)], total: u64, tree: &[(usize, usize, u64)]) {
    assert_eq!(tree.len(), n.saturating_sub(1), "tree {tree:?} for n={n}");
    let mut pool = edges.to_vec();
    for e in tree {
        let i = pool
            .iter()
            .position(|x| x == e)
            .unwrap_or_else(|| panic!("tree edge {e:?} is not in the input (or used twice): {edges:?}"));
        pool.swap_remove(i);
    }
    assert!(is_connected(n, tree), "tree {tree:?} does not span {n} vertices");
    assert_eq!(
        tree.iter().map(|&(_, _, w)| w).sum::<u64>(),
        total,
        "tree {tree:?}"
    );
}

/// Tries every subset of `n - 1` edges.
fn brute_mst(n: usize, edges: &[(usize, usize, u64)]) -> Option<u64> {
    if n <= 1 {
        return Some(0);
    }
    let m = edges.len();
    (0u32..1 << m)
        .filter(|mask| mask.count_ones() as usize == n - 1)
        .filter_map(|mask| {
            let chosen: Vec<_> = (0..m).filter(|i| mask >> i & 1 == 1).map(|i| edges[i]).collect();
            is_connected(n, &chosen).then(|| chosen.iter().map(|&(_, _, w)| w).sum())
        })
        .min()
}

fn random_graph(rng: &mut Rng, n: usize, m: usize, max_w: u64) -> Vec<(usize, usize, u64)> {
    (0..m)
        .map(|_| {
            let u = rng.below(n as u64) as usize;
            let v = rng.below(n as u64) as usize;
            (u, v, rng.below(max_w + 1))
        })
        .collect()
}

const README_EDGES: [(usize, usize, u64); 5] = [(0, 1, 1), (0, 2, 4), (1, 2, 2), (1, 3, 5), (2, 3, 3)];

#[test]
fn readme_kruskal() {
    let (total, mut tree) = kruskal(4, &README_EDGES).unwrap();
    assert_eq!(total, 6);
    // The tree's edge order is unspecified.
    tree.sort_unstable();
    assert_eq!(tree, [(0, 1, 1), (1, 2, 2), (2, 3, 3)]);
}

#[test]
fn readme_prim() {
    let adj = vec![
        vec![(1, 1), (2, 4)],
        vec![(0, 1), (2, 2), (3, 5)],
        vec![(0, 4), (1, 2), (3, 3)],
        vec![(1, 5), (2, 3)],
    ];
    assert_eq!(prim(&adj), Some(6));
    assert_eq!(adjacency(4, &README_EDGES), adj);
}

#[test]
fn readme_small_cases() {
    assert_eq!(kruskal(3, &[(0, 1, 7)]), None);
    assert_eq!(kruskal(1, &[]), Some((0, vec![])));
    assert_eq!(prim(&[]), Some(0));
}

#[test]
fn zero_and_one_vertex() {
    assert_eq!(kruskal(0, &[]), Some((0, vec![])));
    assert_eq!(kruskal(1, &[(0, 0, 9)]), Some((0, vec![])));
    assert_eq!(prim(&[vec![]]), Some(0));
    assert_eq!(prim(&[vec![(0, 9)]]), Some(0));
}

#[test]
fn disconnected_is_none() {
    let edges = [(0, 1, 1), (2, 3, 1)];
    assert_eq!(kruskal(4, &edges), None);
    assert_eq!(prim(&adjacency(4, &edges)), None);
    // Two vertices, no edges.
    assert_eq!(kruskal(2, &[]), None);
    assert_eq!(prim(&adjacency(2, &[])), None);
}

#[test]
fn isolated_last_vertex_is_none() {
    let edges = [(0, 1, 1), (1, 2, 1), (2, 0, 1), (0, 1, 1)];
    assert_eq!(kruskal(4, &edges), None);
    assert_eq!(prim(&adjacency(4, &edges)), None);
}

#[test]
fn self_loops_are_never_chosen() {
    let edges = [(0, 0, 0), (1, 1, 0), (0, 1, 10)];
    assert_eq!(kruskal(2, &edges), Some((10, vec![(0, 1, 10)])));
    assert_eq!(prim(&adjacency(2, &edges)), Some(10));
}

#[test]
fn parallel_edges_use_the_cheapest() {
    let edges = [(0, 1, 9), (1, 0, 2), (0, 1, 5), (1, 2, 4), (2, 1, 1)];
    let (total, mut tree) = kruskal(3, &edges).unwrap();
    assert_eq!(total, 3);
    assert_spanning_tree(3, &edges, total, &tree);
    // Only one minimum tree exists; its edges keep their input endpoint order.
    tree.sort_unstable();
    assert_eq!(tree, [(1, 0, 2), (2, 1, 1)]);
    assert_eq!(prim(&adjacency(3, &edges)), Some(3));
}

#[test]
fn lightest_edges_can_form_a_cycle() {
    // The three lightest edges form a triangle: the third must be skipped.
    let edges = [(0, 1, 1), (1, 2, 1), (2, 0, 1), (2, 3, 7)];
    let (total, tree) = kruskal(4, &edges).unwrap();
    assert_eq!(total, 9);
    assert_spanning_tree(4, &edges, total, &tree);
    assert_eq!(prim(&adjacency(4, &edges)), Some(9));
}

#[test]
fn prim_skips_stale_heap_entries() {
    // Vertex 2 is first pushed at weight 100, later at weight 1. Counting the
    // stale entry (or adding a vertex twice) gives a wrong total.
    let edges = [(0, 2, 100), (0, 1, 1), (1, 2, 1), (2, 3, 1)];
    assert_eq!(prim(&adjacency(4, &edges)), Some(3));
    assert_eq!(kruskal(4, &edges).map(|(t, _)| t), Some(3));
}

#[test]
fn ties_and_zero_weights() {
    let edges = [(0, 1, 0), (1, 2, 0), (2, 3, 2), (3, 0, 2), (1, 3, 2), (0, 2, 0)];
    let (total, tree) = kruskal(4, &edges).unwrap();
    assert_eq!(total, 2);
    assert_spanning_tree(4, &edges, total, &tree);
    assert_eq!(prim(&adjacency(4, &edges)), Some(2));
}

#[test]
fn large_weights_sum_past_u32() {
    let n = 10;
    let edges: Vec<_> = (0..n - 1).map(|i| (i, i + 1, 1_000_000_000)).collect();
    assert_eq!(kruskal(n, &edges).map(|(t, _)| t), Some(9_000_000_000));
    assert_eq!(prim(&adjacency(n, &edges)), Some(9_000_000_000));
}

#[test]
fn random_small_graphs_match_brute_force() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    let mut connected = 0;
    for _ in 0..300 {
        let n = rng.below(7) as usize;
        let m = if n == 0 { 0 } else { rng.below(11) as usize };
        let edges = random_graph(&mut rng, n.max(1), m, 6);
        let want = brute_mst(n, &edges);
        let got = kruskal(n, &edges);
        assert_eq!(
            got.as_ref().map(|&(t, _)| t),
            want,
            "kruskal: n={n} edges={edges:?}"
        );
        if let Some((total, tree)) = got {
            connected += 1;
            assert_spanning_tree(n, &edges, total, &tree);
        }
        assert_eq!(prim(&adjacency(n, &edges)), want, "prim: n={n} edges={edges:?}");
    }
    assert!(connected > 50, "the generator should produce connected graphs");
}

#[test]
fn random_medium_graphs_kruskal_and_prim_agree() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..30 {
        let n = 50 + rng.below(150) as usize;
        let m = n + rng.below(4 * n as u64) as usize;
        let edges = random_graph(&mut rng, n, m, 20);
        let k = kruskal(n, &edges);
        assert_eq!(
            k.as_ref().map(|&(t, _)| t),
            prim(&adjacency(n, &edges)),
            "n={n} edges={edges:?}"
        );
        if let Some((total, tree)) = k {
            assert_spanning_tree(n, &edges, total, &tree);
        }
    }
}

#[test]
fn union_find_star_edge_orders() {
    // Two stars of 4.5 * 10^4 vertices, edges in increasing weight order, the
    // hub listed first in one and second in the other. A union-find with
    // neither path compression nor balancing, which always hangs the same side
    // under the other, builds a chain through the hub in one orientation, and
    // every later `find` walks it: ~10^9 steps. Path compression or union by
    // size keeps the walks short; using both gives the near-constant bound.
    let star = |n: usize, hub: usize, hub_first: bool| -> Vec<Edge> {
        (0..n)
            .filter(|&v| v != hub)
            .enumerate()
            .map(|(i, v)| if hub_first { (hub, v, i as u64) } else { (v, hub, i as u64) })
            .collect()
    };
    let n = 45_000;
    for (hub, hub_first) in [(0, true), (12_345, false)] {
        let edges = star(n, hub, hub_first);
        let expected: u64 = edges.iter().map(|&(_, _, w)| w).sum();
        let (total, tree) = kruskal(n, &edges).unwrap_or_else(|| panic!("star with hub {hub}, n = {n}"));
        assert_eq!(total, expected, "star with hub {hub}, n = {n}");
        assert_eq!(tree.len(), n - 1, "star with hub {hub}, n = {n}");
    }
    let edges = star(2000, 0, true);
    let expected: u64 = edges.iter().map(|&(_, _, w)| w).sum();
    assert_eq!(prim(&adjacency(2000, &edges)), Some(expected), "star with hub 0, n = 2000");
}

#[test]
fn large_random_graph() {
    // 1.1 * 10^4 vertices, ~2.2 * 10^4 edges. A Prim that scans every vertex
    // (O(n^2)) or every edge (O(n * m)) for the cheapest next edge does
    // ~10^8 steps here; a heap does ~3 * 10^5.
    let n = 11_000;
    let mut rng = Rng(0x1234_5678_9ABC_DEF1);
    // A random spanning tree first, so the graph is connected.
    let mut edges: Vec<_> = (1..n)
        .map(|v| (rng.below(v as u64) as usize, v, rng.below(1_000_000_001)))
        .collect();
    edges.extend(random_graph(&mut rng, n, n, 1_000_000_000));
    let (total, tree) = kruskal(n, &edges).unwrap();
    assert_eq!(prim(&adjacency(n, &edges)), Some(total));
    assert_eq!(tree.len(), n - 1);
    assert!(is_connected(n, &tree));
    assert_eq!(tree.iter().map(|&(_, _, w)| w).sum::<u64>(), total);
}
