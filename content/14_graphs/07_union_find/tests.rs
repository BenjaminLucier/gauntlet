// Tests for Union-Find — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::HashMap;

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

/// The model: an explicit set label per element; union relabels in O(n).
struct BruteSets {
    label: Vec<usize>,
}

impl BruteSets {
    fn new(n: usize) -> Self {
        BruteSets { label: (0..n).collect() }
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let (la, lb) = (self.label[a], self.label[b]);
        if la == lb {
            return false;
        }
        for l in &mut self.label {
            if *l == lb {
                *l = la;
            }
        }
        true
    }

    fn connected(&self, a: usize, b: usize) -> bool {
        self.label[a] == self.label[b]
    }

    fn size(&self, x: usize) -> usize {
        self.label.iter().filter(|&&l| l == self.label[x]).count()
    }

    fn count(&self) -> usize {
        let mut labels = self.label.clone();
        labels.sort_unstable();
        labels.dedup();
        labels.len()
    }
}

/// Every element's representative must be shared exactly by its set, and the
/// sizes and the count must agree with that grouping.
fn assert_consistent(uf: &mut UnionFind) {
    let n = uf.len();
    let mut members: HashMap<usize, usize> = HashMap::new();
    for x in 0..n {
        *members.entry(uf.find(x)).or_default() += 1;
    }
    assert_eq!(uf.count(), members.len(), "count() disagrees with the distinct roots");
    for x in 0..n {
        let root = uf.find(x);
        assert_eq!(uf.find(root), root, "the representative of {x} isn't its own representative");
        assert_eq!(uf.component_size(x), members[&root], "component_size({x})");
    }
}

#[test]
fn readme_example() {
    let mut uf = UnionFind::new(6);
    assert_eq!(uf.count(), 6);
    assert!(uf.union(0, 1));
    assert!(uf.union(2, 3));
    assert!(uf.union(1, 3));
    assert!(!uf.union(0, 2));
    assert!(uf.connected(0, 3));
    assert!(!uf.connected(0, 4));
    assert_eq!(uf.component_size(2), 4);
    assert_eq!(uf.component_size(5), 1);
    assert_eq!(uf.count(), 3);
    let r = uf.find(1);
    assert_eq!(uf.find(2), r);
    assert_eq!(uf.len(), 6);
}

#[test]
fn empty_structure() {
    let uf = UnionFind::new(0);
    assert!(uf.is_empty());
    assert_eq!(uf.len(), 0);
    assert_eq!(uf.count(), 0);
}

#[test]
fn starts_as_singletons() {
    let mut uf = UnionFind::new(5);
    assert!(!uf.is_empty());
    assert_eq!(uf.count(), 5);
    for x in 0..5 {
        assert_eq!(uf.find(x), x, "a singleton is its own representative");
        assert_eq!(uf.component_size(x), 1);
        assert!(uf.connected(x, x));
    }
    assert!(!uf.connected(0, 4));
}

#[test]
fn union_with_itself_changes_nothing() {
    let mut uf = UnionFind::new(3);
    assert!(!uf.union(1, 1));
    assert_eq!(uf.count(), 3);
    assert_eq!(uf.component_size(1), 1);
}

#[test]
fn union_reports_transitive_connections() {
    let mut uf = UnionFind::new(4);
    assert!(uf.union(0, 1));
    assert!(uf.union(1, 2));
    assert!(!uf.union(2, 0), "0 and 2 are connected through 1");
    assert!(!uf.union(0, 2));
    assert_eq!(uf.count(), 2);
    assert!(uf.union(3, 0));
    assert_eq!(uf.count(), 1);
    assert_eq!(uf.component_size(3), 4);
}

#[test]
fn union_argument_order_does_not_matter() {
    let mut a = UnionFind::new(4);
    let mut b = UnionFind::new(4);
    for (x, y) in [(0, 1), (2, 3), (0, 3)] {
        assert_eq!(a.union(x, y), b.union(y, x));
    }
    for x in 0..4 {
        assert_eq!(a.component_size(x), b.component_size(x));
    }
}

#[test]
fn sizes_add_when_sets_merge() {
    let mut uf = UnionFind::new(10);
    for x in 1..4 {
        uf.union(0, x); // {0, 1, 2, 3}
    }
    for x in 5..10 {
        uf.union(x, 4); // {4, …, 9}
    }
    assert_eq!(uf.component_size(2), 4);
    assert_eq!(uf.component_size(9), 6);
    // Merging through non-root members must still add the whole sets.
    assert!(uf.union(3, 7));
    for x in 0..10 {
        assert_eq!(uf.component_size(x), 10);
    }
    assert_eq!(uf.count(), 1);
}

#[test]
fn representative_is_stable_between_unions() {
    let mut uf = UnionFind::new(8);
    for (a, b) in [(0, 1), (2, 3), (1, 3), (4, 5), (6, 4)] {
        uf.union(a, b);
    }
    let reps: Vec<usize> = (0..8).map(|x| uf.find(x)).collect();
    let again: Vec<usize> = (0..8).map(|x| uf.find(x)).collect();
    assert_eq!(reps, again, "find must not change answers on its own");
    assert_consistent(&mut uf);
}

#[test]
fn random_operations_match_brute_force_labels() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for round in 0..20 {
        let n = 1 + rng.below(40) as usize;
        let mut uf = UnionFind::new(n);
        let mut model = BruteSets::new(n);
        for step in 0..1_000 {
            let a = rng.below(n as u64) as usize;
            let b = rng.below(n as u64) as usize;
            let msg = format!("round {round}, n = {n}, step {step}, a = {a}, b = {b}");
            match rng.below(4) {
                0 => assert_eq!(uf.union(a, b), model.union(a, b), "union: {msg}"),
                1 => assert_eq!(uf.connected(a, b), model.connected(a, b), "connected: {msg}"),
                2 => assert_eq!(uf.component_size(a), model.size(a), "size: {msg}"),
                _ => assert_eq!(uf.count(), model.count(), "count: {msg}"),
            }
        }
        assert_consistent(&mut uf);
    }
}

#[test]
fn long_chain_built_by_unions() {
    // A naive union (no size rule) makes 0 → 1 → 2 → … one long chain; without
    // path compression every later find(0) walks all of it, ~n^2 = 2 * 10^8
    // steps here.
    let n = 15_000;
    let mut uf = UnionFind::new(n);
    for i in 0..n - 1 {
        assert!(uf.union(i, i + 1));
    }
    assert_eq!(uf.count(), 1);
    for i in 0..n {
        assert!(uf.connected(0, i));
    }
    assert_eq!(uf.component_size(0), n);
}

#[test]
fn long_chain_built_in_reverse() {
    let n = 15_000;
    let mut uf = UnionFind::new(n);
    for i in (1..n).rev() {
        assert!(uf.union(i, i - 1));
    }
    for i in (0..n).rev() {
        assert!(uf.connected(i, n - 1));
    }
    assert_eq!(uf.component_size(n / 2), n);
}

#[test]
fn large_star_of_merges() {
    // Always merging a fresh singleton into the big set. Union by size keeps
    // hanging the singleton under the big root; hanging the big root under the
    // singleton instead builds a chain, ~n^2 / 2 = 6 * 10^8 steps here.
    let n = 35_000;
    let mut uf = UnionFind::new(n);
    for i in 1..n {
        assert!(uf.union(i, 0));
        assert_eq!(uf.component_size(i), i + 1);
    }
    assert_eq!(uf.count(), 1);
}

#[test]
fn many_random_operations() {
    let n = 200_000;
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut uf = UnionFind::new(n);
    let mut merges = 0;
    let mut hits = 0;
    for _ in 0..200_000 {
        let a = rng.below(n as u64) as usize;
        let b = rng.below(n as u64) as usize;
        if rng.below(2) == 0 {
            merges += usize::from(uf.union(a, b));
        } else {
            hits += usize::from(uf.connected(a, b));
        }
    }
    assert_eq!(uf.count(), n - merges, "every successful union removes one set");
    assert!(hits > 0, "some random pairs must end up connected");
    assert_consistent(&mut uf);
}
