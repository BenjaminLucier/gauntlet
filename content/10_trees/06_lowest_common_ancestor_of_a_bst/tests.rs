// Tests for Lowest Common Ancestor of a Binary Search Tree — shared by template.rs, solution.rs and your attempt.
use super::*;

const EX: &str = "[6,2,8,0,4,7,9,null,null,3,5]";

fn lca_val(root: Option<&TreeNode>, p: i32, q: i32) -> Option<i32> {
    lowest_common_ancestor(root, p, q).map(|n| n.val)
}

fn check(root: &str, p: i32, q: i32, expected: Option<i32>) {
    let tr = t(root);
    assert_eq!(lca_val(tr.as_deref(), p, q), expected, "root = {root}, p = {p}, q = {q}");
    assert_eq!(lca_val(tr.as_deref(), q, p), expected, "root = {root}, p = {q}, q = {p}");
}

/// Values on the search path from the root to `v`, or `None` if `v` is absent.
fn path(root: Option<&TreeNode>, v: i32) -> Option<Vec<i32>> {
    let (mut out, mut cur) = (vec![], root);
    while let Some(n) = cur {
        out.push(n.val);
        if v == n.val {
            return Some(out);
        }
        cur = if v < n.val { n.left.as_deref() } else { n.right.as_deref() };
    }
    None
}

fn brute_lca(root: Option<&TreeNode>, p: i32, q: i32) -> Option<i32> {
    let (a, b) = (path(root, p)?, path(root, q)?);
    a.iter().zip(&b).take_while(|(x, y)| x == y).last().map(|(x, _)| *x)
}

/// Height-balanced BST holding `offset + i * step` for `i` in `lo..hi`.
fn balanced_bst(lo: usize, hi: usize, step: i32, offset: i32) -> Tree {
    if lo >= hi {
        return None;
    }
    let mid = lo + (hi - lo) / 2;
    Some(Box::new(TreeNode {
        val: offset + mid as i32 * step,
        left: balanced_bst(lo, mid, step, offset),
        right: balanced_bst(mid + 1, hi, step, offset),
    }))
}

#[test]
fn example_1() {
    check(EX, 2, 8, Some(6));
}

#[test]
fn example_2() {
    check(EX, 2, 4, Some(2));
}

#[test]
fn example_3() {
    check("[2,1]", 2, 1, Some(2));
}

#[test]
fn two_node_trees() {
    check("[1,null,2]", 1, 2, Some(1));
    check("[-5,-6]", -6, -5, Some(-5));
}

#[test]
fn both_in_left_subtree() {
    check(EX, 0, 5, Some(2));
    check(EX, 3, 5, Some(4));
}

#[test]
fn both_in_right_subtree() {
    check(EX, 7, 9, Some(8));
}

#[test]
fn ancestor_is_one_of_the_nodes() {
    check(EX, 6, 3, Some(6));
    check(EX, 4, 3, Some(4));
    check(EX, 8, 9, Some(8));
}

#[test]
fn p_larger_than_q() {
    // Solutions that assume p < q go the wrong way here.
    let tr = t(EX);
    assert_eq!(lca_val(tr.as_deref(), 9, 7), Some(8));
    assert_eq!(lca_val(tr.as_deref(), 5, 0), Some(2));
}

#[test]
fn p_equal_to_q_is_that_node() {
    check(EX, 3, 3, Some(3));
    check(EX, 6, 6, Some(6));
    check(EX, 1, 1, None);
}

#[test]
fn empty_tree_has_no_ancestor() {
    assert!(lowest_common_ancestor(None, 1, 2).is_none());
}

#[test]
fn missing_value_gives_none() {
    check(EX, 2, 1, None); // 1 would sit under 0 but isn't there
    check(EX, 10, 9, None); // beyond the largest value
    check(EX, -1, 100, None); // both missing, and they split at the root
    check("[5]", 5, 6, None);
}

#[test]
fn missing_value_below_the_split_point() {
    // 3 and 5 split at node 4 in both trees, but the second tree has no 5:
    // a solution that stops at the split without checking reports 4 there.
    check(EX, 3, 5, Some(4));
    check("[6,2,8,0,4,7,9,null,null,3]", 3, 5, None);
    check(EX, 2, 1, None);
}

#[test]
fn returns_a_borrow_of_the_real_node() {
    let tr = t(EX);
    let node = lowest_common_ancestor(tr.as_deref(), 3, 5).expect("both present");
    // It's the node inside the tree, with its whole subtree attached.
    assert_eq!(level(Some(node)), parse("[4,3,5]"));
    let expected = tr.as_deref().and_then(|r| r.left.as_deref()).and_then(|n| n.right.as_deref());
    assert!(std::ptr::eq(node, expected.expect("node 4 exists")));
}

#[test]
fn product_overflow_trap() {
    // (root - p) * (root - q) overflows i32 with values near ±10^9.
    let root = "[0,-1000000000,1000000000,null,-999999999,999999999]";
    check(root, -1_000_000_000, 1_000_000_000, Some(0));
    check(root, -999_999_999, -1_000_000_000, Some(-1_000_000_000));
    check(root, 999_999_999, 1_000_000_000, Some(1_000_000_000));
    check(root, -999_999_999, 999_999_999, Some(0));
    check(root, -999_999_998, 999_999_999, None);
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(52);
    for _ in 0..400 {
        let n = 2 + rng.below(40);
        let (lo, hi) = if rng.below(2) == 0 { (-60, 60) } else { (-1_000_000_000, 1_000_000_000) };
        let tr = rand_bst(&mut rng, n, lo, hi);
        let vals = inorder(tr.as_deref());
        let i = rng.below(n);
        let j = (i + 1 + rng.below(n - 1)) % n;
        let (p, mut q) = (vals[i], vals[j]);
        if rng.below(4) == 0 {
            q = rng.range(lo, hi); // usually absent
        }
        let expected = brute_lca(tr.as_deref(), p, q);
        assert_eq!(
            lca_val(tr.as_deref(), p, q),
            expected,
            "root = {}, p = {p}, q = {q}",
            show(tr.as_deref())
        );
    }
}

#[test]
fn deep_chains() {
    big_stack(|| {
        // Increasing right chain 0..2000: LCA of i < j is i.
        let vals: Vec<i32> = (0..2000).collect();
        let tr = chain(&vals, |_| false);
        assert_eq!(lca_val(tr.as_deref(), 1998, 1999), Some(1998));
        assert_eq!(lca_val(tr.as_deref(), 1999, 0), Some(0));
        assert_eq!(lca_val(tr.as_deref(), 1000, 1500), Some(1000));
        assert_eq!(lca_val(tr.as_deref(), 1000, 2000), None);
        // Decreasing left chain.
        let vals: Vec<i32> = (0..2000).rev().collect();
        let tr = chain(&vals, |_| true);
        assert_eq!(lca_val(tr.as_deref(), 0, 1), Some(1));
    });
}

#[test]
fn large_balanced_bst_many_queries() {
    // 2400 queries on a balanced BST of 10^5 nodes. Searching the whole tree
    // (the general binary-tree LCA, or checking that p and q exist with a full
    // traversal) visits up to 10^5 nodes per query: ~2 * 10^8 in all. Following
    // the search paths is ~17 nodes per query.
    let n = 100_000;
    let tr = balanced_bst(0, n, 20_000, -1_000_000_000);
    let mut rng = Rng(5252);
    for _ in 0..1_200 {
        let p = -1_000_000_000 + rng.below(n) as i32 * 20_000;
        let mut q = p;
        while q == p {
            q = -1_000_000_000 + rng.below(n) as i32 * 20_000;
        }
        let expected = brute_lca(tr.as_deref(), p, q);
        assert!(expected.is_some());
        assert_eq!(lca_val(tr.as_deref(), p, q), expected, "p = {p}, q = {q}");
        assert_eq!(lca_val(tr.as_deref(), p, q + 1), None, "p = {p}, q = {}", q + 1);
    }
}

// ------------------------------------------------------------------ helpers --

use std::collections::VecDeque;

fn leaf(val: i32) -> Box<TreeNode> {
    Box::new(TreeNode { val, left: None, right: None })
}

/// Build a tree from level order (`None` = missing child), e.g.
/// `tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])`.
fn tree(vals: &[Option<i32>]) -> Tree {
    let mut root = Some(leaf(vals.first().copied().flatten()?));
    {
        let mut queue: VecDeque<&mut TreeNode> = VecDeque::new();
        queue.extend(root.as_deref_mut());
        let mut rest = vals[1..].iter();
        while let Some(n) = queue.pop_front() {
            let Some(&l) = rest.next() else { break };
            let r = rest.next().copied().flatten();
            n.left = l.map(leaf);
            n.right = r.map(leaf);
            queue.extend(n.left.as_deref_mut());
            queue.extend(n.right.as_deref_mut());
        }
    }
    root
}

/// Shorthand: `t("[3,9,20,null,null,15,7]")`.
fn t(s: &str) -> Tree {
    tree(&parse(s))
}

fn parse(s: &str) -> Vec<Option<i32>> {
    let s = s.trim().trim_start_matches('[').trim_end_matches(']');
    if s.trim().is_empty() {
        return vec![];
    }
    s.split(',')
        .map(|x| match x.trim() {
            "null" => None,
            x => Some(x.parse().expect("bad number in tree literal")),
        })
        .collect()
}

/// Tree -> level order, trailing `None`s trimmed. Iterative, so deep trees are fine.
fn level(root: Option<&TreeNode>) -> Vec<Option<i32>> {
    let mut out = vec![];
    let mut queue = VecDeque::from([root]);
    while let Some(n) = queue.pop_front() {
        match n {
            Some(n) => {
                out.push(Some(n.val));
                queue.push_back(n.left.as_deref());
                queue.push_back(n.right.as_deref());
            }
            None => out.push(None),
        }
    }
    while out.last() == Some(&None) {
        out.pop();
    }
    out
}

/// Tree -> "[3,9,20,null,null,15,7]" for assert messages.
fn show(root: Option<&TreeNode>) -> String {
    let parts: Vec<String> = level(root)
        .iter()
        .map(|v| v.map_or("null".to_string(), |v| v.to_string()))
        .collect();
    format!("[{}]", parts.join(","))
}

/// A path-shaped tree: node i+1 hangs left of node i if `go_left(i)`, else right.
/// Built bottom-up, so no recursion.
fn chain(vals: &[i32], go_left: impl Fn(usize) -> bool) -> Tree {
    let (&last, init) = vals.split_last()?;
    let mut below = leaf(last);
    for (i, &v) in init.iter().enumerate().rev() {
        let mut n = leaf(v);
        if go_left(i) {
            n.left = Some(below);
        } else {
            n.right = Some(below);
        }
        below = n;
    }
    Some(below)
}

/// Run `f` on a thread with a big stack, so recursive solutions survive deep trees.
fn big_stack(f: impl FnOnce() + Send + 'static) {
    let h = std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(f)
        .expect("spawn test thread");
    if let Err(e) = h.join() {
        std::panic::resume_unwind(e);
    }
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

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    /// Uniform in `lo..=hi`.
    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        let span = (i64::from(hi) - i64::from(lo) + 1) as u64;
        (i64::from(lo) + (self.next() % span) as i64) as i32
    }
}

/// Insert into a BST (no-op on duplicates). Iterative.
fn bst_insert(root: &mut Tree, v: i32) {
    let mut slot = root;
    while let Some(n) = slot {
        if v == n.val {
            return;
        }
        slot = if v < n.val { &mut n.left } else { &mut n.right };
    }
    *slot = Some(leaf(v));
}

/// Random BST with `n` distinct values in `lo..=hi` (needs `hi - lo + 1 >= n`).
fn rand_bst(rng: &mut Rng, n: usize, lo: i32, hi: i32) -> Tree {
    let mut root = None;
    let mut seen = std::collections::HashSet::new();
    while seen.len() < n {
        let v = rng.range(lo, hi);
        if seen.insert(v) {
            bst_insert(&mut root, v);
        }
    }
    root
}

/// Inorder values (iterative).
fn inorder(root: Option<&TreeNode>) -> Vec<i32> {
    let (mut out, mut stack, mut cur) = (vec![], vec![], root);
    while cur.is_some() || !stack.is_empty() {
        while let Some(n) = cur {
            stack.push(n);
            cur = n.left.as_deref();
        }
        let n = stack.pop().expect("stack is non-empty here");
        out.push(n.val);
        cur = n.right.as_deref();
    }
    out
}
