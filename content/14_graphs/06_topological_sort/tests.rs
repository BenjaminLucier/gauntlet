// Tests for Topological Sort — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::HashSet;

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

/// Run `f` on a thread with a big stack, so recursive DFS solutions pass the
/// deep tests; panics (including "not yet implemented") are re-raised.
fn with_big_stack<F: FnOnce() + Send + 'static>(f: F) {
    let handle = std::thread::Builder::new()
        .stack_size(512 << 20)
        .spawn(f)
        .expect("spawn test thread");
    if let Err(panic) = handle.join() {
        std::panic::resume_unwind(panic);
    }
}

fn assert_topological(n: usize, edges: &[(usize, usize)], order: &[usize]) {
    assert_eq!(order.len(), n, "order {order:?} must list all {n} vertices");
    let mut pos = vec![usize::MAX; n];
    for (i, &v) in order.iter().enumerate() {
        assert!(v < n && pos[v] == usize::MAX, "order {order:?} isn't a permutation");
        pos[v] = i;
    }
    for &(u, v) in edges {
        assert!(pos[u] < pos[v], "edge ({u}, {v}) points backwards in {order:?}");
    }
}

fn assert_cycle(edges: &[(usize, usize)], cycle: &Cycle) {
    let vs = &cycle.vertices;
    assert!(!vs.is_empty(), "a cycle needs at least one vertex");
    let distinct: HashSet<usize> = vs.iter().copied().collect();
    assert_eq!(distinct.len(), vs.len(), "cycle {vs:?} repeats a vertex");
    let set: HashSet<(usize, usize)> = edges.iter().copied().collect();
    for i in 0..vs.len() {
        let (u, v) = (vs[i], vs[(i + 1) % vs.len()]);
        assert!(set.contains(&(u, v)), "cycle {vs:?} uses ({u}, {v}), which isn't an edge");
    }
}

/// Repeatedly take the smallest vertex with no remaining in-edges: O(V^2 + VE).
fn brute_smallest_order(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut done = vec![false; n];
    let mut order = Vec::new();
    while order.len() < n {
        let next = (0..n).find(|&v| !done[v] && edges.iter().all(|&(a, b)| b != v || done[a]))?;
        done[next] = true;
        order.push(next);
    }
    Some(order)
}

fn random_dag(rng: &mut Rng, n: usize, m: usize) -> Vec<(usize, usize)> {
    // Edges go forward in a random permutation, so the graph is acyclic but
    // the identity order usually isn't a valid answer.
    let mut perm: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        perm.swap(i, rng.below(i as u64 + 1) as usize);
    }
    (0..m)
        .map(|_| {
            let a = rng.below(n as u64) as usize;
            let b = rng.below(n as u64) as usize;
            (perm[a.min(b)], perm[a.max(b)])
        })
        .filter(|&(u, v)| u != v)
        .collect()
}

#[test]
fn readme_example() {
    let edges = [(3, 1), (1, 0), (3, 2), (2, 0)];
    assert_eq!(topo_sort_kahn(4, &edges), Ok(vec![3, 1, 2, 0]));
    let order = topo_sort_dfs(4, &edges).expect("a DAG");
    assert_topological(4, &edges, &order);
    assert_eq!(topo_sort_kahn(3, &[]), Ok(vec![0, 1, 2]));

    let edges = [(0, 1), (1, 2), (2, 0), (2, 3)];
    let cycle = topo_sort_kahn(4, &edges).expect_err("0 → 1 → 2 → 0");
    assert_eq!(cycle.vertices.len(), 3);
    assert_cycle(&edges, &cycle);
    let cycle = topo_sort_dfs(4, &edges).expect_err("0 → 1 → 2 → 0");
    assert_cycle(&edges, &cycle);
    assert_eq!(topo_sort_dfs(2, &[(1, 1)]), Err(Cycle { vertices: vec![1] }));
}

#[test]
fn empty_graph() {
    assert_eq!(topo_sort_kahn(0, &[]), Ok(vec![]));
    assert_eq!(topo_sort_dfs(0, &[]), Ok(vec![]));
}

#[test]
fn no_edges() {
    assert_eq!(topo_sort_kahn(4, &[]), Ok(vec![0, 1, 2, 3]));
    let order = topo_sort_dfs(4, &[]).expect("no edges, no cycle");
    assert_topological(4, &[], &order);
}

#[test]
fn kahn_takes_the_smallest_ready_vertex_not_the_oldest() {
    // A FIFO queue gives [2, 3, 0, 1]: 0 becomes ready after 3 was queued.
    let edges = [(2, 0), (3, 1)];
    assert_eq!(topo_sort_kahn(4, &edges), Ok(vec![2, 0, 3, 1]));
    let edges = [(4, 0), (4, 1), (3, 2)];
    assert_eq!(topo_sort_kahn(5, &edges), Ok(vec![3, 2, 4, 0, 1]));
}

#[test]
fn reversed_chain() {
    let edges: Vec<(usize, usize)> = (1..6).map(|v| (v, v - 1)).collect();
    assert_eq!(topo_sort_kahn(6, &edges), Ok(vec![5, 4, 3, 2, 1, 0]));
    assert_eq!(topo_sort_dfs(6, &edges), Ok(vec![5, 4, 3, 2, 1, 0]), "only one order exists");
}

#[test]
fn parallel_edges_count_twice_in_the_in_degree() {
    let edges = [(0, 1), (0, 1), (1, 2), (1, 2), (1, 2)];
    assert_eq!(topo_sort_kahn(3, &edges), Ok(vec![0, 1, 2]));
    assert_eq!(topo_sort_dfs(3, &edges), Ok(vec![0, 1, 2]));
}

#[test]
fn self_loop_is_a_one_vertex_cycle() {
    let edges = [(0, 1), (2, 2), (1, 2)];
    assert_eq!(topo_sort_kahn(3, &edges), Err(Cycle { vertices: vec![2] }));
    assert_eq!(topo_sort_dfs(3, &edges), Err(Cycle { vertices: vec![2] }));
}

#[test]
fn two_vertex_cycle() {
    let edges = [(1, 0), (0, 1)];
    for result in [topo_sort_kahn(2, &edges), topo_sort_dfs(2, &edges)] {
        let cycle = result.expect_err("0 ⇄ 1");
        assert_eq!(cycle.vertices.len(), 2);
        assert_cycle(&edges, &cycle);
    }
}

#[test]
fn cycle_downstream_of_a_dag_part() {
    // 0 → 1 → 2 → 3 → 4 → 2: the cycle is 2, 3, 4; vertices 0 and 1 are fine.
    let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 2), (4, 5)];
    for result in [topo_sort_kahn(6, &edges), topo_sort_dfs(6, &edges)] {
        let cycle = result.expect_err("2 → 3 → 4 → 2");
        assert_cycle(&edges, &cycle);
        let mut vs = cycle.vertices.clone();
        vs.sort_unstable();
        assert_eq!(vs, [2, 3, 4], "vertices leading into the cycle aren't part of it");
    }
}

#[test]
fn kahn_cycle_ignores_vertices_merely_downstream_of_it() {
    // 5 is stuck behind the cycle (it never reaches in-degree 0) but is not on it.
    let edges = [(3, 5), (1, 3), (3, 1), (5, 4)];
    let cycle = topo_sort_kahn(6, &edges).expect_err("1 ⇄ 3");
    assert_cycle(&edges, &cycle);
}

#[test]
fn grey_versus_black_is_not_a_cycle() {
    // A diamond: DFS reaches 3 twice, the second time it's already finished.
    let edges = [(0, 1), (0, 2), (1, 3), (2, 3)];
    let order = topo_sort_dfs(4, &edges).expect("a diamond is a DAG");
    assert_topological(4, &edges, &order);
    // A cross edge to an earlier DFS tree.
    let edges = [(0, 1), (2, 1), (2, 0)];
    let order = topo_sort_dfs(3, &edges).expect("still a DAG");
    assert_topological(3, &edges, &order);
}

#[test]
fn random_dags_match_brute_force() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for _ in 0..300 {
        let n = 1 + rng.below(9) as usize;
        let m = rng.below(20) as usize;
        let edges = random_dag(&mut rng, n, m);
        let msg = format!("n = {n}, edges = {edges:?}");
        let expected = brute_smallest_order(n, &edges).expect("generated a DAG");
        assert_eq!(topo_sort_kahn(n, &edges), Ok(expected), "{msg}");
        let order = topo_sort_dfs(n, &edges).unwrap_or_else(|c| panic!("{msg}: {c:?}"));
        assert_topological(n, &edges, &order);
    }
}

#[test]
fn random_graphs_with_cycles_report_real_cycles() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut cyclic = 0;
    for _ in 0..500 {
        let n = 1 + rng.below(8) as usize;
        let m = rng.below(14) as usize;
        let edges: Vec<(usize, usize)> = (0..m)
            .map(|_| (rng.below(n as u64) as usize, rng.below(n as u64) as usize))
            .collect();
        let msg = format!("n = {n}, edges = {edges:?}");
        let expected = brute_smallest_order(n, &edges);
        match (topo_sort_kahn(n, &edges), &expected) {
            (Ok(order), Some(e)) => assert_eq!(&order, e, "{msg}"),
            (Err(cycle), None) => assert_cycle(&edges, &cycle),
            (got, _) => panic!("{msg}: Kahn returned {got:?}, expected {expected:?}"),
        }
        match (topo_sort_dfs(n, &edges), &expected) {
            (Ok(order), Some(_)) => assert_topological(n, &edges, &order),
            (Err(cycle), None) => {
                cyclic += 1;
                assert_cycle(&edges, &cycle);
            }
            (got, _) => panic!("{msg}: DFS returned {got:?}, expected {expected:?}"),
        }
    }
    assert!(cyclic > 100, "the generator should produce plenty of cycles");
}

#[test]
fn large_random_dag() {
    with_big_stack(|| {
        let n = 5000;
        let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);
        let mut edges = random_dag(&mut rng, n, 3 * n);
        // Make the order unique: chain all vertices in topological order first.
        let order = topo_sort_kahn(n, &edges).expect("generated a DAG");
        assert_topological(n, &edges, &order);
        edges.extend(order.windows(2).map(|w| (w[0], w[1])));
        assert_eq!(topo_sort_kahn(n, &edges), Ok(order.clone()));
        assert_eq!(topo_sort_dfs(n, &edges), Ok(order), "a Hamiltonian path fixes the order");
    });
}

#[test]
fn wide_ready_set() {
    // Edges v → n-1-v for v < n/2: ~n/2 vertices are ready at every step.
    // Finding the smallest by scanning them (or all n vertices) is
    // ~n^2 / 2 = 3 * 10^8 steps; a min-heap pops it in O(log V).
    let n = 24_000;
    let edges: Vec<(usize, usize)> = (0..n / 2).map(|v| (v, n - 1 - v)).collect();
    let order = topo_sort_kahn(n, &edges);
    assert!(order == Ok((0..n).collect()), "{} edges v → n-1-v, n = {n}", n / 2);
}

#[test]
fn long_cycle_is_reported_whole() {
    with_big_stack(|| {
        let n = 200_000;
        let edges: Vec<(usize, usize)> = (0..n).map(|v| (v, (v + 1) % n)).collect();
        for result in [topo_sort_kahn(n, &edges), topo_sort_dfs(n, &edges)] {
            let cycle = result.expect_err("one big cycle");
            assert_eq!(cycle.vertices.len(), n);
            assert_cycle(&edges, &cycle);
        }
    });
}

#[test]
fn deep_chain_with_a_cycle_at_the_bottom() {
    with_big_stack(|| {
        // The depth matters to DFS; Kahn's queue is flat, so it gets a shorter
        // chain of the same shape.
        let chain = |n: usize| {
            let mut edges: Vec<(usize, usize)> = (1..n).map(|v| (v - 1, v)).collect();
            edges.push((n - 1, n - 3));
            edges
        };
        let (deep, short) = (200_000, 10_000);
        let (deep_edges, short_edges) = (chain(deep), chain(short));
        let results = [
            (short, &short_edges, topo_sort_kahn(short, &short_edges)),
            (deep, &deep_edges, topo_sort_dfs(deep, &deep_edges)),
        ];
        for (n, edges, result) in results {
            let cycle = result.expect_err("n-3 → n-2 → n-1 → n-3");
            assert_eq!(cycle.vertices.len(), 3, "chain of {n}");
            assert_cycle(edges, &cycle);
        }
    });
}
