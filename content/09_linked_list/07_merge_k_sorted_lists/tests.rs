// Tests for Merge k Sorted Lists — shared by template.rs, solution.rs and your attempt.
use super::*;

fn list(vals: &[i32]) -> List {
    vals.iter()
        .rev()
        .fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}

fn lists(vs: &[Vec<i32>]) -> Vec<List> {
    vs.iter().map(|v| list(v)).collect()
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

fn brute_merge(vs: &[Vec<i32>]) -> Vec<i32> {
    let mut all: Vec<i32> = vs.iter().flatten().copied().collect();
    all.sort_unstable();
    all
}

fn check(vs: &[Vec<i32>]) {
    let shown = if vs.len() > 20 {
        format!("<{} lists>", vs.len())
    } else {
        format!("{vs:?}")
    };
    assert_eq!(to_vec(merge_k_lists(lists(vs))), brute_merge(vs), "lists = {shown}");
}

/// Checks that the result consists of exactly the input nodes, values untouched.
fn check_nodes_moved(vs: &[Vec<i32>]) {
    let ls = lists(vs);
    let mut expected: Vec<_> = ls.iter().flat_map(|l| addrs(l.as_deref())).collect();
    expected.sort_unstable();
    let out = merge_k_lists(ls);
    let mut got = addrs(out.as_deref());
    got.sort_unstable();
    assert_eq!(got, expected, "result should consist of exactly the input nodes");
    assert_eq!(to_vec(out), brute_merge(vs));
}

#[test]
fn example_1() {
    let out = merge_k_lists(lists(&[vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]]));
    assert_eq!(to_vec(out), [1, 1, 2, 3, 4, 4, 5, 6]);
}

#[test]
fn example_2_no_lists() {
    assert_eq!(merge_k_lists(vec![]), None);
}

#[test]
fn example_3_one_empty_list() {
    assert_eq!(merge_k_lists(vec![None]), None);
}

#[test]
fn many_empty_lists() {
    assert_eq!(merge_k_lists(vec![None, None, None, None]), None);
}

#[test]
fn empties_mixed_with_non_empty() {
    check(&[vec![], vec![3], vec![], vec![1, 2], vec![]]);
}

#[test]
fn single_list_unchanged() {
    check(&[vec![-3, -1, 0, 0, 7]]);
    // Structurally identical to the input.
    assert_eq!(merge_k_lists(vec![list(&[1, 2, 3])]), list(&[1, 2, 3]));
}

#[test]
fn two_lists() {
    check(&[vec![1, 3, 5], vec![2, 4, 6, 8, 10]]);
}

#[test]
fn many_single_node_lists_reverse_order() {
    let vs: Vec<Vec<i32>> = (0..20).rev().map(|i| vec![i]).collect();
    check(&vs);
}

#[test]
fn duplicates_everywhere() {
    check(&[vec![1, 1, 1], vec![1, 1], vec![0, 1, 2], vec![1]]);
}

#[test]
fn negatives_and_bounds() {
    check(&[vec![-10_000, 0, 10_000], vec![-10_000, -10_000], vec![10_000]]);
}

#[test]
fn disjoint_ranges() {
    check(&[vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]]);
}

#[test]
fn reuses_input_nodes() {
    check_nodes_moved(&[vec![1, 5], vec![2, 3], vec![4]]);
    check_nodes_moved(&[vec![2, 2], vec![], vec![2, 2, 2]]);
}

#[test]
fn random_against_brute_force() {
    let mut rng = Rng(0x7777_1234_ABCD_0001);
    for _ in 0..300 {
        let k = rng.range(0, 8) as usize;
        let vs: Vec<Vec<i32>> = (0..k)
            .map(|_| {
                let len = rng.range(0, 6) as usize;
                let mut v: Vec<i32> = (0..len).map(|_| rng.range(-20, 20)).collect();
                v.sort_unstable();
                v
            })
            .collect();
        check(&vs);
    }
}

#[test]
fn large_many_short_lists() {
    // k = 2500 random lists of 2 nodes.
    let mut rng = Rng(0x0BAD_5EED_0000_0042);
    let vs: Vec<Vec<i32>> = (0..2500)
        .map(|_| {
            let mut v = vec![rng.range(-10_000, 10_000), rng.range(-10_000, 10_000)];
            v.sort_unstable();
            v
        })
        .collect();
    check(&vs);
}

#[test]
fn large_ten_thousand_lists_mostly_empty() {
    let vs: Vec<Vec<i32>> = (0..10_000)
        .map(|i| if i % 2 == 0 { vec![10_000 - i] } else { vec![] })
        .collect();
    check(&vs);
}

#[test]
fn large_few_long_lists_nodes_moved() {
    let vs: Vec<Vec<i32>> = (0..4)
        .map(|j| (0..1250).map(|i| i * 4 + j - 5000).collect())
        .collect();
    check_nodes_moved(&vs);
}

#[test]
fn ten_thousand_single_node_lists_merged_repeatedly() {
    // k = 10^4 single-node lists in ascending order, merged 4 times (the
    // result is split back into single nodes for the next round). Merging the
    // lists one at a time into an accumulator walks all of it each time,
    // N * k / 2 = 5 * 10^7 steps per round; scanning every list's head for
    // each output node is N * k = 10^8. A heap or pairwise merging is about
    // N * log2(k) = 1.4 * 10^5.
    let k = 10_000;
    let mut ls: Vec<List> = (0..k).map(|i| list(&[i - 5_000])).collect();
    for round in 0..4 {
        let mut out = merge_k_lists(std::mem::take(&mut ls));
        let mut expected = -5_000;
        while let Some(mut node) = out {
            out = node.next.take();
            assert_eq!(node.val, expected, "k = {k}, round {round}");
            expected += 1;
            ls.push(Some(node));
        }
        assert_eq!(ls.len(), k as usize, "k = {k}, round {round}: nodes lost");
    }
}
