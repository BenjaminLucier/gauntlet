// Tests for Redundant Connection — shared by template.rs, solution.rs and your attempt.
use super::*;

use std::collections::HashSet;

type Edge = (usize, usize);

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

fn shuffle<T>(rng: &mut Rng, v: &mut [T]) {
    for i in (1..v.len()).rev() {
        v.swap(i, rng.below(i as u64 + 1) as usize);
    }
}

/// Is the graph on 0..n with these edges connected?
fn connected(n: usize, edges: &[Edge]) -> bool {
    let mut adj = vec![Vec::new(); n];
    for &(a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut seen = vec![false; n];
    let mut stack = vec![0];
    seen[0] = true;
    let mut count = 1;
    while let Some(u) = stack.pop() {
        for &v in &adj[u] {
            if !seen[v] {
                seen[v] = true;
                count += 1;
                stack.push(v);
            }
        }
    }
    count == n
}

fn without(edges: &[Edge], i: usize) -> Vec<Edge> {
    let mut rest = edges.to_vec();
    rest.remove(i);
    rest
}

/// Try removing each edge from last to first; n-1 connected edges form a tree.
fn brute_redundant(edges: &[Edge]) -> Edge {
    let n = edges.len();
    (0..n)
        .rev()
        .find(|&i| connected(n, &without(edges, i)))
        .map(|i| edges[i])
        .expect("input always has a redundant edge")
}

/// A random tree on 0..n plus one extra edge (a < b), shuffled.
fn random_input(rng: &mut Rng, n: usize) -> Vec<Edge> {
    let mut perm: Vec<usize> = (0..n).collect();
    shuffle(rng, &mut perm);
    let mut set = HashSet::new();
    let mut edges = Vec::new();
    for i in 1..n {
        let j = rng.below(i as u64) as usize;
        let e = (perm[i].min(perm[j]), perm[i].max(perm[j]));
        set.insert(e);
        edges.push(e);
    }
    loop {
        let a = rng.below(n as u64) as usize;
        let b = rng.below(n as u64) as usize;
        let e = (a.min(b), a.max(b));
        if a != b && set.insert(e) {
            edges.push(e);
            break;
        }
    }
    shuffle(rng, &mut edges);
    edges
}

#[test]
fn example_1() {
    assert_eq!(find_redundant_connection(&[(0, 1), (0, 2), (1, 2)]), Some((1, 2)));
}

#[test]
fn example_2() {
    assert_eq!(
        find_redundant_connection(&[(0, 1), (1, 2), (2, 3), (0, 3), (0, 4)]),
        Some((0, 3))
    );
}

#[test]
fn example_3_tree_has_no_redundant_edge() {
    assert_eq!(find_redundant_connection(&[(0, 1), (1, 2)]), None);
}

#[test]
fn empty_input_is_none() {
    assert_eq!(find_redundant_connection(&[]), None);
}

#[test]
fn forest_is_none() {
    assert_eq!(find_redundant_connection(&[(0, 1), (2, 3), (4, 5), (3, 4)]), None);
}

#[test]
fn triangle_in_another_order() {
    // Answer depends on input order, not on the edge's values.
    assert_eq!(find_redundant_connection(&[(1, 2), (0, 2), (0, 1)]), Some((0, 1)));
}

#[test]
fn last_edge_in_input_is_not_on_cycle() {
    // (3, 4) is listed last but removing it disconnects 4.
    assert_eq!(
        find_redundant_connection(&[(0, 1), (1, 2), (0, 2), (3, 4), (2, 3)]),
        Some((0, 2))
    );
}

#[test]
fn cycle_not_containing_node_zero() {
    assert_eq!(
        find_redundant_connection(&[(0, 1), (1, 2), (2, 3), (3, 4), (2, 4)]),
        Some((2, 4))
    );
}

#[test]
fn cycle_through_whole_graph() {
    assert_eq!(
        find_redundant_connection(&[(0, 1), (1, 2), (2, 3), (3, 4), (0, 4)]),
        Some((0, 4))
    );
}

#[test]
fn cycle_completed_by_merging_two_trees() {
    // Two separate subtrees are joined, then a later edge closes the cycle.
    assert_eq!(
        find_redundant_connection(&[(0, 1), (2, 3), (1, 3), (4, 5), (0, 5), (2, 5)]),
        Some((2, 5))
    );
}

#[test]
fn smallest_input_each_order() {
    // n = 3: every edge is on the triangle, so the answer is always the last one.
    let tri = [(0, 1), (0, 2), (1, 2)];
    for perm in [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]] {
        let edges: Vec<Edge> = perm.iter().map(|&i| tri[i]).collect();
        assert_eq!(find_redundant_connection(&edges), Some(edges[2]), "edges = {edges:?}");
    }
}

#[test]
fn randomized_vs_remove_and_check() {
    let mut rng = Rng(0x7777_1111_AAAA_5555);
    for _ in 0..400 {
        let n = 3 + rng.below(10) as usize;
        let edges = random_input(&mut rng, n);
        let expected = brute_redundant(&edges);
        assert_eq!(find_redundant_connection(&edges), Some(expected), "edges = {edges:?}");
    }
}

#[test]
fn medium_random_vs_remove_and_check() {
    let mut rng = Rng(8080);
    for _ in 0..3 {
        let edges = random_input(&mut rng, 150);
        let expected = brute_redundant(&edges);
        assert_eq!(find_redundant_connection(&edges), Some(expected));
    }
}

#[test]
fn large_full_cycle_closing_edge_first() {
    // (0, 999) comes first; the last cycle edge in input order is (998, 999).
    let mut edges = vec![(0, 999)];
    edges.extend((0..999).map(|i| (i, i + 1)));
    assert_eq!(find_redundant_connection(&edges), Some((998, 999)));
}

#[test]
fn large_random() {
    let mut rng = Rng(4242);
    let edges = random_input(&mut rng, 1000);
    let got = find_redundant_connection(&edges).expect("input has a cycle");
    let idx = edges
        .iter()
        .position(|&x| x == got)
        .expect("returned edge must be from the input");
    assert!(connected(1000, &without(&edges, idx)), "removing {got:?} does not leave a tree");
    // No later edge may also work.
    for i in idx + 1..edges.len() {
        assert!(
            !connected(1000, &without(&edges, i)),
            "{:?} appears later and is also removable",
            edges[i]
        );
    }
}
