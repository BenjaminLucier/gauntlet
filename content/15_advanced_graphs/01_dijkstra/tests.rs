// Tests for Dijkstra's Algorithm — shared by template.rs, solution.rs and your attempt.
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

type Adj = Vec<Vec<(usize, u64)>>;

fn random_graph(rng: &mut Rng, n: usize, m: usize, mut weight: impl FnMut(&mut Rng) -> u64) -> Adj {
    let mut adj = vec![Vec::new(); n];
    for _ in 0..m {
        let u = rng.below(n as u64) as usize;
        let v = rng.below(n as u64) as usize;
        let w = weight(rng);
        adj[u].push((v, w));
    }
    adj
}

/// Bellman-Ford-style relaxation with the same overflow rule: O(V * E).
fn brute_distances(adj: &[Vec<(usize, u64)>], s: usize) -> Vec<Option<u64>> {
    let mut dist: Vec<Option<u64>> = vec![None; adj.len()];
    dist[s] = Some(0);
    for _ in 0..adj.len() {
        for u in 0..adj.len() {
            let Some(du) = dist[u] else { continue };
            for &(v, w) in &adj[u] {
                if let Some(nd) = du.checked_add(w)
                    && dist[v].is_none_or(|old| nd < old)
                {
                    dist[v] = Some(nd);
                }
            }
        }
    }
    dist
}

/// The path must follow real edges from `s` to `t` and add up to `total`.
fn assert_valid_path(adj: &[Vec<(usize, u64)>], s: usize, t: usize, total: u64, path: &[usize]) {
    assert_eq!(path.first(), Some(&s), "path {path:?} must start at {s}");
    assert_eq!(path.last(), Some(&t), "path {path:?} must end at {t}");
    let mut sum = 0u64;
    for pair in path.windows(2) {
        let cheapest = adj[pair[0]]
            .iter()
            .filter(|&&(v, _)| v == pair[1])
            .map(|&(_, w)| w)
            .min()
            .unwrap_or_else(|| panic!("{pair:?} in {path:?} isn't an edge"));
        sum = sum
            .checked_add(cheapest)
            .unwrap_or_else(|| panic!("path {path:?} overflows u64"));
    }
    assert_eq!(sum, total, "path {path:?} has length {sum}, but the distance is {total}");
}

#[test]
fn readme_example() {
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
    let adj = vec![vec![(1, u64::MAX)], vec![(2, 1)], vec![]];
    assert_eq!(dijkstra(&adj, 0), [Some(0), Some(u64::MAX), None]);
}

#[test]
fn single_vertex() {
    let adj: Adj = vec![vec![]];
    assert_eq!(dijkstra(&adj, 0), [Some(0)]);
    assert_eq!(shortest_path(&adj, 0, 0), Some((0, vec![0])));
}

#[test]
fn unreachable_vertices_are_none() {
    // 2 and 3 have edges, but only out of their own corner.
    let adj = vec![vec![(1, 5)], vec![], vec![(3, 1)], vec![(0, 1)]];
    assert_eq!(dijkstra(&adj, 0), [Some(0), Some(5), None, None]);
    assert_eq!(shortest_path(&adj, 1, 0), None);
    assert_eq!(shortest_path(&adj, 2, 1), Some((7, vec![2, 3, 0, 1])));
}

#[test]
fn cheap_long_path_beats_expensive_short_one() {
    let adj = vec![vec![(4, 100), (1, 1)], vec![(2, 1)], vec![(3, 1)], vec![(4, 1)], vec![]];
    assert_eq!(dijkstra(&adj, 0), [Some(0), Some(1), Some(2), Some(3), Some(4)]);
    assert_eq!(shortest_path(&adj, 0, 4), Some((4, vec![0, 1, 2, 3, 4])));
}

#[test]
fn a_vertex_is_final_when_popped_not_when_pushed() {
    // Marking 1 as done when it's first discovered (at 10) misses 0 → 2 → 1.
    let adj = vec![vec![(1, 10), (2, 1)], vec![(3, 1)], vec![(1, 1)], vec![]];
    assert_eq!(dijkstra(&adj, 0), [Some(0), Some(2), Some(1), Some(3)]);
    assert_eq!(shortest_path(&adj, 0, 3), Some((3, vec![0, 2, 1, 3])));
}

#[test]
fn zero_weight_edges() {
    let adj = vec![vec![(1, 0)], vec![(2, 0), (0, 0)], vec![(3, 5)], vec![(2, 0)]];
    assert_eq!(dijkstra(&adj, 0), [Some(0), Some(0), Some(0), Some(5)]);
    let (d, path) = shortest_path(&adj, 0, 2).expect("reachable");
    assert_eq!(d, 0);
    assert_eq!(path, [0, 1, 2], "a zero-weight cycle must not end up in the path");
}

#[test]
fn parallel_edges_use_the_cheapest() {
    let adj = vec![vec![(1, 5), (1, 2), (1, 9)], vec![(2, 1), (2, 0)], vec![]];
    assert_eq!(dijkstra(&adj, 0), [Some(0), Some(2), Some(2)]);
    assert_eq!(shortest_path(&adj, 0, 2), Some((2, vec![0, 1, 2])));
}

#[test]
fn self_loops_change_nothing() {
    let adj = vec![vec![(0, 0), (1, 3)], vec![(1, 7)]];
    assert_eq!(dijkstra(&adj, 0), [Some(0), Some(3)]);
    assert_eq!(shortest_path(&adj, 0, 1), Some((3, vec![0, 1])));
}

#[test]
fn overflowing_paths_do_not_exist() {
    let max = u64::MAX;
    // 0 → 1 → 3 overflows; 0 → 2 → 3 is exactly u64::MAX and is the answer.
    let adj = vec![vec![(1, 1), (2, max - 5)], vec![(3, max)], vec![(3, 5)], vec![]];
    assert_eq!(dijkstra(&adj, 0), [Some(0), Some(1), Some(max - 5), Some(max)]);
    assert_eq!(shortest_path(&adj, 0, 3), Some((max, vec![0, 2, 3])));
    // Two huge edges in a row: unreachable, not saturated to u64::MAX.
    let adj = vec![vec![(1, max / 2 + 1)], vec![(2, max / 2 + 1)], vec![]];
    assert_eq!(dijkstra(&adj, 0), [Some(0), Some(max / 2 + 1), None]);
    assert_eq!(shortest_path(&adj, 0, 2), None);
}

#[test]
fn overflow_does_not_hide_a_later_valid_route() {
    let max = u64::MAX;
    // 2 is first seen through an overflowing sum, then reached properly.
    let adj = vec![vec![(1, max - 1), (3, 10)], vec![(2, 5)], vec![], vec![(2, max - 20)]];
    assert_eq!(dijkstra(&adj, 0), [Some(0), Some(max - 1), Some(max - 10), Some(10)]);
}

#[test]
fn random_graphs_match_bellman_ford() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for _ in 0..300 {
        let n = 1 + rng.below(9) as usize;
        let m = rng.below(25) as usize;
        let adj = random_graph(&mut rng, n, m, |r| r.below(6)); // lots of zeros and ties
        let s = rng.below(n as u64) as usize;
        let msg = format!("adj = {adj:?}, source = {s}");
        let expected = brute_distances(&adj, s);
        assert_eq!(dijkstra(&adj, s), expected, "{msg}");
        for (t, &d) in expected.iter().enumerate() {
            match (shortest_path(&adj, s, t), d) {
                (Some((total, path)), Some(d)) => {
                    assert_eq!(total, d, "{msg}: t = {t}");
                    assert_valid_path(&adj, s, t, total, &path);
                }
                (None, None) => {}
                (got, d) => panic!("{msg}: path to {t} = {got:?}, distance {d:?}"),
            }
        }
    }
}

#[test]
fn random_huge_weights_match_bellman_ford() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..300 {
        let n = 1 + rng.below(8) as usize;
        let m = rng.below(20) as usize;
        // Weights near u64::MAX / 3: paths of 3+ edges tend to overflow.
        let adj = random_graph(&mut rng, n, m, |r| u64::MAX / 3 - r.below(4));
        let s = rng.below(n as u64) as usize;
        let msg = format!("adj = {adj:?}, source = {s}");
        let expected = brute_distances(&adj, s);
        assert_eq!(dijkstra(&adj, s), expected, "{msg}");
        for (t, &d) in expected.iter().enumerate() {
            let got = shortest_path(&adj, s, t);
            assert_eq!(got.as_ref().map(|p| p.0), d, "{msg}: t = {t}");
            if let Some((total, path)) = got {
                assert_valid_path(&adj, s, t, total, &path);
            }
        }
    }
}

#[test]
fn hub_improved_many_times() {
    // Hub vertex h is improved m times before it's popped, so it sits in the
    // heap m times; h has d outgoing edges. Rescanning them for every stale
    // copy costs m * d ~ 5.6 * 10^7 relaxations per call, against d when stale
    // copies are skipped. With 1.5 * 10^4 vertices, an O(V^2) scan for the
    // closest vertex is ~2.3 * 10^8 steps per call.
    let m = 7_500;
    let d = 7_500;
    let h = m + 1;
    let n = h + 1 + d;
    let mut adj: Adj = vec![Vec::new(); n];
    for i in 1..=m {
        adj[0].push((i, i as u64));
        adj[i].push((h, (3 * m - 2 * i) as u64)); // via i, h is at 3m - i
    }
    adj[h] = (h + 1..n).map(|leaf| (leaf, 1)).collect();
    let dist = dijkstra(&adj, 0);
    assert_eq!(dist[h], Some(2 * m as u64));
    assert!(dist[h + 1..].iter().all(|&x| x == Some(2 * m as u64 + 1)));
    let (total, path) = shortest_path(&adj, 0, n - 1).expect("leaves are reachable");
    assert_eq!(total, 2 * m as u64 + 1);
    assert_eq!(path, [0, m, h, n - 1]);
}

#[test]
fn large_random_graph() {
    // 5000 vertices, 2.5 * 10^4 edges: an O(V^2) "scan for the closest
    // vertex" Dijkstra does ~2.5 * 10^7 steps here.
    let n = 5_000;
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);
    let adj = random_graph(&mut rng, n, 5 * n, |r| 1 + r.below(1_000_000_000));
    let dist = dijkstra(&adj, 0);
    assert_eq!(dist[0], Some(0));
    // Shortest distances are exactly the labels where no edge can improve a
    // vertex and every reached vertex but the source has a tight incoming edge
    // (positive weights rule out circular support).
    let mut tight = vec![false; n];
    for (u, edges) in adj.iter().enumerate() {
        let Some(du) = dist[u] else { continue };
        for &(v, w) in edges {
            let via = du + w;
            let dv = dist[v].unwrap_or_else(|| panic!("{v} is reachable from {u}"));
            assert!(dv <= via, "edge {u} → {v} improves {v}");
            tight[v] |= dv == via;
        }
    }
    let reached = dist.iter().filter(|d| d.is_some()).count();
    assert!(reached > n / 2);
    for v in 1..n {
        assert_eq!(dist[v].is_some(), tight[v], "vertex {v}: distance {:?}", dist[v]);
    }
}

#[test]
fn long_chain_path_reconstruction() {
    let n = 5000;
    let mut adj: Adj = (0..n).map(|v| if v + 1 < n { vec![(v + 1, 7)] } else { vec![] }).collect();
    adj[0].push((n - 1, 7 * n as u64)); // a shortcut that's slightly too long
    let (total, path) = shortest_path(&adj, 0, n - 1).expect("end of the chain");
    assert_eq!(total, 7 * (n as u64 - 1));
    assert_eq!(path, (0..n).collect::<Vec<_>>());
}
