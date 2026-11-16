// Tests for Remove Nth Node From End of List — shared by template.rs, solution.rs and your attempt.
use super::*;

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

fn brute_remove(vals: &[i32], n: usize) -> Vec<i32> {
    let mut v = vals.to_vec();
    if (1..=vals.len()).contains(&n) {
        v.remove(vals.len() - n);
    }
    v
}

fn check(vals: &[i32], n: usize) {
    assert_eq!(
        to_vec(remove_nth_from_end(list(vals), n)),
        brute_remove(vals, n),
        "head = {vals:?}, n = {n}"
    );
}

#[test]
fn example_1() {
    assert_eq!(to_vec(remove_nth_from_end(list(&[1, 2, 3, 4, 5]), 2)), [1, 2, 3, 5]);
}

#[test]
fn example_2_single_node_removed() {
    assert_eq!(remove_nth_from_end(list(&[1]), 1), None);
}

#[test]
fn example_3() {
    assert_eq!(to_vec(remove_nth_from_end(list(&[1, 2]), 1)), [1]);
}

#[test]
fn remove_head_of_two() {
    assert_eq!(to_vec(remove_nth_from_end(list(&[1, 2]), 2)), [2]);
}

#[test]
fn remove_head_n_equals_length() {
    // The classic trap: no "previous" node exists for the head.
    check(&[10, 20, 30, 40, 50], 5);
}

#[test]
fn remove_tail() {
    check(&[10, 20, 30, 40, 50], 1);
}

#[test]
fn remove_middle() {
    check(&[10, 20, 30, 40, 50], 3);
}

#[test]
fn every_position_of_small_list() {
    let vals = [0, 1, 2, 3, 4, 5, 6];
    for n in 1..=vals.len() {
        check(&vals, n);
    }
}

#[test]
fn duplicate_values_removes_correct_position() {
    // Removing "by value" instead of by position fails here.
    check(&[7, 7, 3, 7, 7], 2);
    check(&[5, 5, 5], 3);
    check(&[0, 100, 0, 100], 1);
}

#[test]
fn n_zero_removes_nothing() {
    assert_eq!(to_vec(remove_nth_from_end(list(&[1, 2, 3]), 0)), [1, 2, 3]);
}

#[test]
fn n_past_the_end_removes_nothing() {
    // `len - n` underflows here: a usize subtraction must not panic.
    assert_eq!(to_vec(remove_nth_from_end(list(&[1, 2, 3]), 4)), [1, 2, 3]);
    assert_eq!(to_vec(remove_nth_from_end(list(&[1, 2, 3]), 10_000)), [1, 2, 3]);
}

#[test]
fn empty_list_stays_empty() {
    assert_eq!(remove_nth_from_end(None, 0), None);
    assert_eq!(remove_nth_from_end(None, 1), None);
}

#[test]
fn other_nodes_are_kept_not_copied() {
    let head = list(&[1, 2, 3, 4, 5]);
    let mut expected = addrs(head.as_deref());
    expected.remove(3); // 2nd from the end
    let out = remove_nth_from_end(head, 2);
    assert_eq!(addrs(out.as_deref()), expected, "surviving nodes should be the original ones");
    to_vec(out);
}

#[test]
fn random_against_brute_force() {
    let mut rng = Rng(0x0123_4567_89AB_CDEF);
    for _ in 0..400 {
        let len = rng.range(0, 30) as usize;
        let vals: Vec<i32> = (0..len).map(|_| rng.range(0, 100)).collect();
        let n = rng.range(0, len as i32 + 2) as usize;
        check(&vals, n);
    }
}

#[test]
fn large_remove_head_and_tail() {
    let vals: Vec<i32> = (0..5000).map(|i| i % 101).collect();
    check(&vals, 5000);
    check(&vals, 1);
    check(&vals, 2500);
    check(&vals, 5001);
}
