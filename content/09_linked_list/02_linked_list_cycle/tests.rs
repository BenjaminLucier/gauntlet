// Tests for Linked List Cycle — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::HashSet;

/// Node i sits at arena index i and links to i + 1; the tail links back to `pos`.
fn build(vals: &[i32], pos: Option<usize>) -> (Vec<Node>, Option<usize>) {
    let n = vals.len();
    let mut nodes: Vec<Node> = vals
        .iter()
        .enumerate()
        .map(|(i, &val)| Node { val, next: (i + 1 < n).then_some(i + 1) })
        .collect();
    if let Some(last) = nodes.last_mut() {
        last.next = pos;
    }
    (nodes, (n > 0).then_some(0))
}

/// Same list, but the arena is shuffled so list order != arena order, and
/// `extra` unreachable junk nodes (forming their own cycle) are appended.
fn build_shuffled(
    vals: &[i32],
    pos: Option<usize>,
    rng: &mut Rng,
    extra: usize,
) -> (Vec<Node>, Option<usize>) {
    let (nodes, head) = build(vals, pos);
    let n = nodes.len();
    let mut perm: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        let j = rng.below(i as u64 + 1) as usize;
        perm.swap(i, j);
    }
    // perm[i] = new index of old node i
    let mut out = vec![Node { val: 0, next: None }; n];
    for (i, node) in nodes.iter().enumerate() {
        out[perm[i]] = Node { val: node.val, next: node.next.map(|j| perm[j]) };
    }
    for k in 0..extra {
        let idx = out.len();
        let next = if k + 1 < extra { idx + 1 } else { n };
        out.push(Node { val: rng.below(1000) as i32, next: Some(next) });
    }
    (out, head.map(|h| perm[h]))
}

/// Obviously-correct O(n)-memory oracle.
fn brute_has_cycle(nodes: &[Node], head: Option<usize>) -> bool {
    let mut seen = HashSet::new();
    let mut cur = head;
    while let Some(i) = cur {
        if !seen.insert(i) {
            return true;
        }
        cur = nodes[i].next;
    }
    false
}

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

fn check(vals: &[i32], pos: Option<usize>) {
    let (nodes, head) = build(vals, pos);
    assert_eq!(has_cycle(&nodes, head), pos.is_some(), "vals = {vals:?}, pos = {pos:?}");
}

#[test]
fn example_1() {
    let (nodes, head) = build(&[3, 2, 0, -4], Some(1));
    assert!(has_cycle(&nodes, head));
}

#[test]
fn example_2() {
    let (nodes, head) = build(&[1, 2], Some(0));
    assert!(has_cycle(&nodes, head));
}

#[test]
fn example_3() {
    let (nodes, head) = build(&[1], None);
    assert!(!has_cycle(&nodes, head));
}

#[test]
fn empty_list() {
    assert!(!has_cycle(&[], None));
}

#[test]
fn no_head_in_non_empty_arena() {
    // The arena has a cycle, but the list itself is empty.
    let nodes = [Node { val: 1, next: Some(0) }];
    assert!(!has_cycle(&nodes, None));
}

#[test]
fn single_node_self_loop() {
    check(&[1], Some(0));
}

#[test]
fn tail_points_to_itself() {
    check(&[1, 2, 3, 4, 5], Some(4));
}

#[test]
fn two_nodes_no_cycle() {
    check(&[1, 2], None);
}

#[test]
fn odd_and_even_lengths_without_cycle() {
    // The fast cursor must stop cleanly whether it lands on the last node or past it.
    for n in 1..=8 {
        let vals: Vec<i32> = (0..n).collect();
        check(&vals, None);
    }
}

#[test]
fn every_cycle_entry_point() {
    for n in 1..=9 {
        let vals: Vec<i32> = (0..n).collect();
        for pos in 0..n as usize {
            check(&vals, Some(pos));
        }
    }
}

#[test]
fn equal_values_are_not_a_cycle() {
    // Comparing values instead of node identity reports a false cycle.
    check(&[7, 7, 7, 7, 7], None);
    check(&[1, 2, 1, 2, 1, 2], None);
}

#[test]
fn head_not_at_index_zero() {
    // Arena: [c, b, a], list is a -> b -> c.
    let mut nodes = vec![
        Node { val: 3, next: None },
        Node { val: 2, next: Some(0) },
        Node { val: 1, next: Some(1) },
    ];
    assert!(!has_cycle(&nodes, Some(2)));
    nodes[0].next = Some(1);
    assert!(has_cycle(&nodes, Some(2)));
}

#[test]
fn unreachable_cycle_is_ignored() {
    // Nodes 3..5 form a cycle, but the list from head (0 -> 1 -> 2) does not.
    // Scanning the whole arena instead of following `next` from head gets this wrong.
    let nodes = [
        Node { val: 0, next: Some(1) },
        Node { val: 1, next: Some(2) },
        Node { val: 2, next: None },
        Node { val: 3, next: Some(4) },
        Node { val: 4, next: Some(5) },
        Node { val: 5, next: Some(3) },
    ];
    assert!(!has_cycle(&nodes, Some(0)));
    assert!(has_cycle(&nodes, Some(3)));
}

#[test]
fn works_on_arrays_and_vecs() {
    let arr = [Node { val: 0, next: Some(1) }, Node { val: 1, next: Some(0) }];
    assert!(has_cycle(&arr, Some(0)));
    let v: Vec<Node> = arr.to_vec();
    assert!(has_cycle(&v, Some(1)));
    // The arena is only borrowed: still usable afterwards.
    assert_eq!(v.len(), 2);
}

#[test]
fn rho_shape_long_tail_short_loop() {
    let vals: Vec<i32> = (0..1000).collect();
    check(&vals, Some(997));
}

#[test]
fn random_shuffled_arena_against_brute_force() {
    let mut rng = Rng(0xA5A5_5A5A_1234_4321);
    for _ in 0..500 {
        let n = rng.below(26) as usize;
        let vals: Vec<i32> = (0..n).map(|_| rng.below(11) as i32 - 5).collect();
        let pos = if n == 0 || rng.below(n as u64 + 1) == 0 {
            None
        } else {
            Some(rng.below(n as u64) as usize)
        };
        let extra = if n == 0 { 0 } else { rng.below(5) as usize };
        let (nodes, head) = build_shuffled(&vals, pos, &mut rng, extra);
        let expected = brute_has_cycle(&nodes, head);
        assert_eq!(expected, pos.is_some());
        assert_eq!(
            has_cycle(&nodes, head),
            expected,
            "vals = {vals:?}, pos = {pos:?}, arena = {nodes:?}, head = {head:?}"
        );
    }
}

#[test]
fn long_lists_with_and_without_a_cycle() {
    // Six lists of 10^4 nodes (values repeat every 7), with no cycle or with
    // the tail looping back near the head, the middle or itself. Re-walking
    // from the head to ask "seen this node before?" is n^2 / 2 = 5 * 10^7
    // steps per list; two pointers take about 2n.
    let n = 10_000;
    let vals: Vec<i32> = (0..n as i32).map(|i| i % 7).collect();
    let (mut nodes, head) = build(&vals, None);
    for pos in [None, Some(0), Some(1), Some(n / 2), Some(n - 2), Some(n - 1)] {
        nodes[n - 1].next = pos;
        assert_eq!(has_cycle(&nodes, head), pos.is_some(), "n = {n}, tail links to {pos:?}");
    }
}
