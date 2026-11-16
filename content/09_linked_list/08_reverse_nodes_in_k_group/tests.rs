// Tests for Reverse Nodes in k-Group — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::HashMap;

fn list(vals: &[i32]) -> List {
    vals.iter()
        .rev()
        .fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}

/// Consumes the list iteratively (a long `Box` chain would otherwise drop recursively).
fn to_vec(mut head: List) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(mut node) = head {
        out.push(node.val);
        assert!(out.len() <= 200_000, "list too long — did you create a cycle?");
        head = node.next.take();
    }
    out
}

/// Node addresses paired with their values, in list order.
fn addrs(mut cur: Option<&ListNode>) -> Vec<(*const ListNode, i32)> {
    let mut out = Vec::new();
    while let Some(node) = cur {
        out.push((node as *const ListNode, node.val));
        cur = node.next.as_deref();
    }
    out
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

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() % (hi - lo + 1) as u64) as i32
    }
}

fn brute_reverse_k(vals: &[i32], k: usize) -> Vec<i32> {
    if k == 0 {
        return vals.to_vec();
    }
    let mut out = Vec::with_capacity(vals.len());
    for chunk in vals.chunks(k) {
        if chunk.len() == k {
            out.extend(chunk.iter().rev());
        } else {
            out.extend(chunk);
        }
    }
    out
}

fn check(vals: &[i32], k: usize) {
    let shown = if vals.len() > 40 {
        format!("<{} values>", vals.len())
    } else {
        format!("{vals:?}")
    };
    assert_eq!(
        to_vec(reverse_k_group(list(vals), k)),
        brute_reverse_k(vals, k),
        "head = {shown}, k = {k}"
    );
}

/// Checks that only links changed: every node keeps its value, none added or lost.
fn check_nodes_moved(vals: &[i32], k: usize) {
    let head = list(vals);
    let before: HashMap<*const ListNode, i32> = addrs(head.as_deref()).into_iter().collect();
    let out = reverse_k_group(head, k);
    let after = addrs(out.as_deref());
    assert_eq!(after.len(), vals.len(), "node count changed (k = {k})");
    for (addr, val) in after {
        assert_eq!(
            before.get(&addr),
            Some(&val),
            "node at {addr:?} now holds {val} — move nodes, don't rewrite values (k = {k})"
        );
    }
    assert_eq!(to_vec(out), brute_reverse_k(vals, k));
}

#[test]
fn example_1() {
    assert_eq!(to_vec(reverse_k_group(list(&[1, 2, 3, 4, 5]), 2)), [2, 1, 4, 3, 5]);
}

#[test]
fn example_2() {
    assert_eq!(to_vec(reverse_k_group(list(&[1, 2, 3, 4, 5]), 3)), [3, 2, 1, 4, 5]);
}

#[test]
fn empty_list() {
    assert_eq!(reverse_k_group(None, 2), None);
    assert_eq!(reverse_k_group(None, 0), None);
}

#[test]
fn single_node() {
    check(&[7], 1);
    check(&[7], 2);
}

#[test]
fn k_equals_1_is_identity() {
    check(&[1, 2, 3, 4, 5], 1);
}

#[test]
fn k_zero_is_identity_not_a_panic() {
    // `len / k` or `chunks(k)` with k = 0 panics.
    assert_eq!(to_vec(reverse_k_group(list(&[1, 2, 3]), 0)), [1, 2, 3]);
}

#[test]
fn k_equals_n_reverses_everything() {
    check(&[1, 2, 3, 4, 5], 5);
}

#[test]
fn k_larger_than_n_is_identity() {
    check(&[1, 2, 3], 4);
    check(&[1, 2, 3], 10_000);
}

#[test]
fn length_multiple_of_k() {
    check(&[1, 2, 3, 4, 5, 6], 2);
    check(&[1, 2, 3, 4, 5, 6], 3);
}

#[test]
fn leftover_group_is_not_reversed() {
    // Reversing the short final group too is a common mistake.
    check(&[1, 2, 3, 4, 5, 6, 7, 8], 3);
    check(&[1, 2, 3, 4], 3);
}

#[test]
fn k_is_n_minus_1() {
    check(&[1, 2, 3, 4, 5, 6], 5);
}

#[test]
fn duplicates_and_zeros() {
    check(&[0, 0, 1, 1, 0, 0, 1], 2);
    check(&[1000, 0, 1000, 0], 4);
}

#[test]
fn every_k_for_small_list() {
    let vals: Vec<i32> = (1..=11).collect();
    for k in 0..=12 {
        check(&vals, k);
    }
}

#[test]
fn nodes_are_moved_not_values_rewritten() {
    let vals: Vec<i32> = (0..10).collect();
    for k in [2, 3, 10] {
        check_nodes_moved(&vals, k);
    }
}

#[test]
fn random_against_brute_force() {
    let mut rng = Rng(0x4B47_524F_5550_0045);
    for _ in 0..400 {
        let n = rng.range(0, 30);
        let k = rng.range(0, n + 2) as usize;
        let vals: Vec<i32> = (0..n).map(|_| rng.range(0, 1000)).collect();
        check(&vals, k);
    }
}

#[test]
fn large_various_k() {
    let vals: Vec<i32> = (0..5000).map(|i| i % 1001).collect();
    for k in [1, 2, 3, 7, 64, 2500, 4999, 5000] {
        check(&vals, k);
    }
}

#[test]
fn large_nodes_are_moved() {
    let vals: Vec<i32> = (0..5000).collect();
    check_nodes_moved(&vals, 7);
}

#[test]
fn reverse_a_long_list_repeatedly() {
    // Reverse a 5000-node list in groups 72 times, feeding each result back
    // in, with k alternating between 2 and 5000. Recounting the rest of the
    // list before every group costs ~n^2 / (2k) steps per call, and walking to
    // a group's end for every node moved costs ~n * k / 2: either way
    // ~10^7 steps per call with the right k, where one pass is ~2n.
    // Every 4 rounds undo each other, since n is even.
    let n = 5000;
    let rounds = 72;
    let vals: Vec<i32> = (0..n as i32).collect();
    let mut head = list(&vals);
    let mut model = vals.clone();
    for round in 0..rounds {
        let k = [2, n][round % 2];
        head = reverse_k_group(head, k);
        if round < 4 {
            model = brute_reverse_k(&model, k);
        }
    }
    assert_eq!(model, vals, "four rounds should undo each other");
    assert_eq!(to_vec(head), vals, "n = {n} after {rounds} rounds");
}
