// Tests for Clone Graph — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::{BTreeMap, HashSet};
use std::rc::Weak;

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

/// Builds a graph from a 1-based adjacency list; returns every node (index i = val i+1).
fn build(adj: &[Vec<i32>]) -> Vec<NodeRef> {
    let nodes: Vec<NodeRef> = (1..=adj.len() as i32).map(Node::new).collect();
    for (node, neighbours) in nodes.iter().zip(adj) {
        for &v in neighbours {
            let neighbour = Rc::clone(&nodes[(v - 1) as usize]);
            node.borrow_mut().neighbors.push(neighbour);
        }
    }
    nodes
}

/// Walks the graph by pointer from `start`; returns adjacency by value and all distinct nodes.
fn walk(start: &NodeRef) -> (BTreeMap<i32, Vec<i32>>, Vec<NodeRef>) {
    let mut seen: HashSet<*const RefCell<Node>> = HashSet::from([Rc::as_ptr(start)]);
    let mut nodes = Vec::new();
    let mut adj = BTreeMap::new();
    let mut stack = vec![Rc::clone(start)];
    while let Some(node) = stack.pop() {
        {
            let b = node.borrow();
            let vals: Vec<i32> = b.neighbors.iter().map(|x| x.borrow().val).collect();
            assert!(
                adj.insert(b.val, vals).is_none(),
                "value {} appears on two different nodes: some node was copied twice",
                b.val
            );
            for neighbour in &b.neighbors {
                if seen.insert(Rc::as_ptr(neighbour)) {
                    stack.push(Rc::clone(neighbour));
                }
            }
        }
        nodes.push(node);
    }
    (adj, nodes)
}

/// Breaks the `Rc` cycles so the test doesn't leak.
fn free(nodes: &[NodeRef]) {
    for node in nodes {
        node.borrow_mut().neighbors.clear();
    }
}

fn check(adj: &[Vec<i32>]) {
    let originals = build(adj);
    let expected: BTreeMap<i32, Vec<i32>> = (1..).zip(adj.iter().cloned()).collect();
    let copy = clone_graph(&originals[0]);
    let (got, copies) = walk(&copy);
    assert_eq!(got, expected, "clone has a different structure; input = {adj:?}");
    assert_eq!(copy.borrow().val, 1, "returned node should be the copy of node 1");
    let original_ptrs: HashSet<_> = originals.iter().map(Rc::as_ptr).collect();
    for c in &copies {
        assert!(
            !original_ptrs.contains(&Rc::as_ptr(c)),
            "clone shares node {} with the original",
            c.borrow().val
        );
    }
    let (after, _) = walk(&originals[0]);
    assert_eq!(after, expected, "the original graph was modified");
    free(&copies);
    free(&originals);
}

fn undirected(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<i32>> {
    let mut adj = vec![Vec::new(); n];
    for &(a, b) in edges {
        adj[a - 1].push(b as i32);
        adj[b - 1].push(a as i32);
    }
    adj
}

#[test]
fn example_four_cycle() {
    check(&[vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]]);
}

#[test]
fn example_readme() {
    let nodes: Vec<NodeRef> = (1..=4).map(Node::new).collect();
    for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
        nodes[a].borrow_mut().neighbors.push(Rc::clone(&nodes[b]));
        nodes[b].borrow_mut().neighbors.push(Rc::clone(&nodes[a]));
    }
    let copy = clone_graph(&nodes[0]);
    assert_eq!(copy.borrow().val, 1);
    assert!(!Rc::ptr_eq(&copy, &nodes[0]));
    let two = Rc::clone(&copy.borrow().neighbors[0]);
    assert_eq!(two.borrow().val, 2);
    assert!(Rc::ptr_eq(&two.borrow().neighbors[0], &copy));
    let (_, copies) = walk(&copy);
    free(&copies);
    free(&nodes);
}

#[test]
fn single_node() {
    check(&[vec![]]);
    let lonely = Node::new(7);
    let copy = clone_graph(&lonely);
    assert_eq!(copy.borrow().val, 7);
    assert!(copy.borrow().neighbors.is_empty());
    assert!(!Rc::ptr_eq(&copy, &lonely));
}

#[test]
fn usable_as_a_map_function() {
    let none: Option<&NodeRef> = None;
    assert!(none.map(clone_graph).is_none());
    let graphs = [Node::new(1), Node::new(2)];
    let copies: Vec<NodeRef> = graphs.iter().map(clone_graph).collect();
    let vals: Vec<i32> = copies.iter().map(|c| c.borrow().val).collect();
    assert_eq!(vals, [1, 2]);
}

#[test]
fn two_nodes_pointing_at_each_other() {
    // The smallest cycle: forgetting the visited map loops forever.
    check(&[vec![2], vec![1]]);
}

#[test]
fn triangle() {
    check(&[vec![2, 3], vec![1, 3], vec![1, 2]]);
}

#[test]
fn path_graph() {
    check(&undirected(6, &[(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)]));
}

#[test]
fn star_centered_elsewhere() {
    // Node 1 is a leaf; everything goes through node 3.
    check(&undirected(6, &[(3, 1), (3, 2), (3, 4), (3, 5), (3, 6)]));
}

#[test]
fn start_from_a_middle_node() {
    // The returned node must be the copy of the node passed in, not of node 1.
    let originals = build(&undirected(3, &[(1, 2), (2, 3)]));
    let copy = clone_graph(&originals[1]);
    assert_eq!(copy.borrow().val, 2);
    let vals: Vec<i32> = copy.borrow().neighbors.iter().map(|n| n.borrow().val).collect();
    assert_eq!(vals, [1, 3]);
    let (_, copies) = walk(&copy);
    assert_eq!(copies.len(), 3);
    free(&copies);
    free(&originals);
}

#[test]
fn shared_neighbor_copied_once() {
    // Nodes 2 and 3 both point at 4: their copies must share one copy of 4.
    let originals = build(&undirected(4, &[(1, 2), (1, 3), (2, 4), (3, 4)]));
    let copy = clone_graph(&originals[0]);
    let two = Rc::clone(&copy.borrow().neighbors[0]);
    let three = Rc::clone(&copy.borrow().neighbors[1]);
    let four_via_two = Rc::clone(&two.borrow().neighbors[1]);
    let four_via_three = Rc::clone(&three.borrow().neighbors[1]);
    assert_eq!(four_via_two.borrow().val, 4);
    assert!(Rc::ptr_eq(&four_via_two, &four_via_three), "node 4 was copied twice");
    let back = Rc::clone(&two.borrow().neighbors[0]);
    assert!(Rc::ptr_eq(&back, &copy), "2's neighbour 1 should be the returned copy of 1");
    let (_, copies) = walk(&copy);
    free(&copies);
    free(&originals);
}

#[test]
fn neighbor_order_preserved() {
    check(&[vec![4, 2, 3], vec![1], vec![1], vec![1]]);
}

#[test]
fn clone_keeps_no_handles_to_the_original() {
    // After the copy is gone, the originals' reference counts are back where
    // they started: the clone didn't stash or leak handles to them.
    let originals = build(&undirected(5, &[(1, 2), (2, 3), (3, 4), (4, 5), (5, 1), (1, 3)]));
    let before: Vec<usize> = originals.iter().map(Rc::strong_count).collect();
    let copy = clone_graph(&originals[0]);
    let (_, copies) = walk(&copy);
    free(&copies);
    drop(copies);
    drop(copy);
    let after: Vec<usize> = originals.iter().map(Rc::strong_count).collect();
    assert_eq!(before, after);
    free(&originals);
}

#[test]
fn copy_is_freed_once_cycles_are_broken() {
    // The function must not leak extra strong handles to the copies either.
    let originals = build(&[vec![2, 3], vec![1, 3], vec![1, 2]]);
    let copy = clone_graph(&originals[0]);
    let (_, copies) = walk(&copy);
    let weak: Vec<Weak<RefCell<Node>>> = copies.iter().map(Rc::downgrade).collect();
    free(&copies);
    drop(copies);
    drop(copy);
    assert!(weak.iter().all(|w| w.upgrade().is_none()), "some copied node was leaked");
    free(&originals);
}

#[test]
fn randomized_connected_graphs() {
    let mut rng = Rng(0x0123_4567_89AB_CDEF);
    for _ in 0..200 {
        let n = 1 + rng.below(15) as usize;
        let mut edges = Vec::new();
        let mut present = HashSet::new();
        for v in 2..=n {
            let u = 1 + rng.below(v as u64 - 1) as usize;
            edges.push((u, v));
            present.insert((u.min(v), u.max(v)));
        }
        let extra = rng.below(n as u64 * 2);
        for _ in 0..extra {
            let a = 1 + rng.below(n as u64) as usize;
            let b = 1 + rng.below(n as u64) as usize;
            if a != b && present.insert((a.min(b), a.max(b))) {
                edges.push((a, b));
            }
        }
        check(&undirected(n, &edges));
    }
}

#[test]
fn large_complete_graph() {
    let n = 100;
    let edges: Vec<(usize, usize)> = (1..=n)
        .flat_map(|a| (a + 1..=n).map(move |b| (a, b)))
        .collect();
    check(&undirected(n, &edges));
}

#[test]
fn large_cycle() {
    let n = 100;
    let mut edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
    edges.push((n, 1));
    check(&undirected(n, &edges));
}

#[test]
fn long_path_needs_no_deep_recursion() {
    // 20,000 nodes in a line: a recursive clone may overflow the test thread's stack.
    let n = 20_000;
    let edges: Vec<(usize, usize)> = (1..n).map(|i| (i, i + 1)).collect();
    check(&undirected(n, &edges));
}
