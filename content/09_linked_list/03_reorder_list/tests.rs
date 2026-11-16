// Tests for Reorder List — shared by template.rs, solution.rs and your attempt.
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

/// Two-pointer oracle on a Vec.
fn brute_reorder(vals: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, vals.len());
    let mut out = Vec::with_capacity(vals.len());
    while i < j {
        out.push(vals[i]);
        i += 1;
        if i < j {
            j -= 1;
            out.push(vals[j]);
        }
    }
    out
}

fn check(vals: &[i32]) {
    assert_eq!(to_vec(reorder_list(list(vals))), brute_reorder(vals), "input {vals:?}");
}

/// Checks that the result is made of exactly the input nodes, each keeping its value.
fn check_nodes_moved(vals: &[i32]) {
    let head = list(vals);
    let before: HashMap<*const ListNode, i32> = addrs(head.as_deref()).into_iter().collect();
    let out = reorder_list(head);
    let after = addrs(out.as_deref());
    assert_eq!(after.len(), vals.len(), "node count changed");
    for (addr, val) in after {
        assert_eq!(
            before.get(&addr),
            Some(&val),
            "node at {addr:?} now holds {val} — move nodes, don't rewrite values or allocate"
        );
    }
    assert_eq!(to_vec(out), brute_reorder(vals));
}

#[test]
fn example_1() {
    assert_eq!(to_vec(reorder_list(list(&[1, 2, 3, 4]))), [1, 4, 2, 3]);
}

#[test]
fn example_2() {
    assert_eq!(to_vec(reorder_list(list(&[1, 2, 3, 4, 5]))), [1, 5, 2, 4, 3]);
}

#[test]
fn empty_list() {
    assert_eq!(reorder_list(None), None);
}

#[test]
fn single_node() {
    check(&[1]);
}

#[test]
fn two_nodes_unchanged() {
    check(&[1, 2]);
}

#[test]
fn three_nodes() {
    check(&[1, 2, 3]);
}

#[test]
fn six_nodes_even() {
    check(&[1, 2, 3, 4, 5, 6]);
}

#[test]
fn seven_nodes_odd() {
    check(&[1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn duplicate_values() {
    check(&[5, 5, 1, 1, 5]);
    check(&[1000, 1000, 1000, 1000]);
}

#[test]
fn every_length_up_to_twenty() {
    for n in 0..=20 {
        let vals: Vec<i32> = (1..=n).collect();
        check(&vals);
    }
}

#[test]
fn nodes_are_moved_not_values_rewritten() {
    // Each node must keep its own value; only the links may change.
    let vals: Vec<i32> = (1..=9).collect();
    check_nodes_moved(&vals);
    let vals: Vec<i32> = (1..=10).collect();
    check_nodes_moved(&vals);
}

#[test]
fn tail_is_terminated() {
    // The middle node ends the list; a leftover link would create a cycle or duplicates.
    let out = reorder_list(list(&[1, 2, 3, 4, 5, 6]));
    assert_eq!(to_vec(out), [1, 6, 2, 5, 3, 4]);
}

#[test]
fn works_through_a_mut_slot() {
    // The by-value API composes with `Option::take` when you only have `&mut List`.
    let mut holder = list(&[1, 2, 3, 4]);
    let slot = &mut holder;
    *slot = reorder_list(slot.take());
    assert_eq!(holder, list(&[1, 4, 2, 3]));
}

#[test]
fn random_against_brute_force() {
    let mut rng = Rng(0x5EED_0F0E_0E0A_0B0C);
    for _ in 0..300 {
        let n = rng.range(0, 40) as usize;
        let vals: Vec<i32> = (0..n).map(|_| rng.range(1, 1000)).collect();
        check(&vals);
    }
}

#[test]
fn large_even_and_odd() {
    let vals: Vec<i32> = (0..5000).map(|i| i % 1000 + 1).collect();
    check(&vals);
    check(&vals[..4999]);
}

#[test]
fn large_nodes_are_moved() {
    let vals: Vec<i32> = (1..=5000).collect();
    check_nodes_moved(&vals);
}

#[test]
fn reorder_a_long_list_repeatedly() {
    // Reorder a 5000-node list 50 times, feeding each result back in.
    // Walking to the tail for every node moved is ~n^2 / 4 = 6 * 10^6 steps
    // per call, ~3 * 10^8 in all; split, reverse and merge is ~2n per call.
    let n = 5000;
    let rounds = 50;
    let mut model: Vec<i32> = (1..=n).collect();
    let mut head = list(&model);
    for _ in 0..rounds {
        head = reorder_list(head);
        model = brute_reorder(&model);
    }
    assert_eq!(to_vec(head), model, "n = {n} after {rounds} reorders");
}
