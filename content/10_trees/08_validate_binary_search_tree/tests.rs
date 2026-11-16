// Tests for Validate Binary Search Tree — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(input: &str, expected: bool) {
    assert_eq!(is_valid_bst(t(input).as_deref()), expected, "input {input}");
}

fn subtree_vals(n: Option<&TreeNode>, out: &mut Vec<i32>) {
    if let Some(n) = n {
        out.push(n.val);
        subtree_vals(n.left.as_deref(), out);
        subtree_vals(n.right.as_deref(), out);
    }
}

/// O(n^2): straight from the definition.
fn brute_valid(n: Option<&TreeNode>) -> bool {
    n.is_none_or(|n| {
        let (mut l, mut r) = (vec![], vec![]);
        subtree_vals(n.left.as_deref(), &mut l);
        subtree_vals(n.right.as_deref(), &mut r);
        l.iter().all(|&x| x < n.val)
            && r.iter().all(|&x| x > n.val)
            && brute_valid(n.left.as_deref())
            && brute_valid(n.right.as_deref())
    })
}

#[test]
fn example_1() {
    check("[2,1,3]", true);
}

#[test]
fn example_2() {
    check("[5,1,4,null,null,3,6]", false);
}

#[test]
fn example_3_only_comparing_with_children_is_wrong() {
    check("[5,4,6,null,null,3,7]", false);
}

#[test]
fn empty_tree_is_valid() {
    assert!(is_valid_bst(None));
}

#[test]
fn grandchild_breaks_left_bound() {
    // 6 is in 5's left subtree via 3's right child.
    check("[5,3,8,1,6]", false);
    check("[5,3,8,1,4]", true);
}

#[test]
fn single_node() {
    check("[0]", true);
}

#[test]
fn duplicates_are_invalid() {
    check("[1,1]", false);
    check("[1,null,1]", false);
    check("[2,2,2]", false);
    check("[3,1,5,null,3]", false); // equal to an ancestor, not the parent
}

#[test]
fn i32_extremes_alone() {
    check("[2147483647]", true);
    check("[-2147483648]", true);
}

#[test]
fn i32_extremes_as_children() {
    // Using i32::MIN / i32::MAX as "no bound" with strict comparisons fails these.
    check("[-2147483648,null,2147483647]", true);
    check("[2147483647,-2147483648]", true);
    check("[0,-2147483648,2147483647]", true);
}

#[test]
fn i32_extremes_duplicated() {
    // Using i32::MIN / MAX as inclusive sentinels lets these slip through.
    check("[-2147483648,-2147483648]", false);
    check("[2147483647,null,2147483647]", false);
}

#[test]
fn deep_violation() {
    check("[10,5,15,3,7,12,20,1,4,6,11]", false); // 11 is 7's right child, inside 10's left subtree
    check("[10,5,15,3,7,12,20,1,4,6,8]", true);
}

#[test]
fn subtrees_can_be_checked_directly() {
    // Example 2's right subtree [4,3,6] is a valid BST on its own.
    let tr = t("[5,1,4,null,null,3,6]");
    let right = tr.as_deref().and_then(|n| n.right.as_deref());
    assert!(is_valid_bst(right));
    assert!(!is_valid_bst(tr.as_deref()));
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(56);
    let (mut yes, mut no) = (0, 0);
    for i in 0..600 {
        let n = 1 + rng.below(30);
        let tr = match i % 3 {
            0 => rand_tree(&mut rng, n, &mut |r| r.range(-4, 4)),
            1 => rand_bst(&mut rng, n, -50, 50),
            _ => {
                // A valid BST with one value nudged: often only subtly broken.
                let mut lv = level(rand_bst(&mut rng, n, -50, 50).as_deref());
                let k = rng.below(lv.len());
                if let Some(v) = lv[k] {
                    lv[k] = Some(v + rng.range(-15, 15));
                }
                tree(&lv)
            }
        };
        let expected = brute_valid(tr.as_deref());
        if expected {
            yes += 1;
        } else {
            no += 1;
        }
        let input = show(tr.as_deref());
        assert_eq!(is_valid_bst(tr.as_deref()), expected, "input {input}");
    }
    assert!(yes > 100 && no > 100);
}

#[test]
fn deep_chains() {
    big_stack(|| {
        let up: Vec<i32> = (0..2000).collect();
        assert!(is_valid_bst(chain(&up, |_| false).as_deref()));
        let down: Vec<i32> = (0..2000).rev().collect();
        assert!(is_valid_bst(chain(&down, |_| true).as_deref()));
        // Zigzag closing in on 1000 from both sides: valid.
        let zig: Vec<i32> = (0..2000).map(|i| if i % 2 == 0 { i / 2 } else { 2000 - i / 2 }).collect();
        assert!(is_valid_bst(chain(&zig, |i| i % 2 == 1).as_deref()));
        // Last node escapes the window set by the root.
        let mut bad = zig.clone();
        bad[1999] = -1;
        assert!(!is_valid_bst(chain(&bad, |i| i % 2 == 1).as_deref()));
    });
}

#[test]
fn long_valid_paths() {
    // Three valid paths of 8000 nodes. Checking every node against all of its
    // subtree costs ~n^2 / 2 = 3.2 * 10^7 steps per path; one DFS is 8000.
    big_stack(|| {
        let n = 8000;
        let up: Vec<i32> = (0..n).collect();
        let down: Vec<i32> = (0..n).rev().collect();
        let zig: Vec<i32> = (0..n).map(|i| if i % 2 == 0 { i / 2 } else { n - i / 2 }).collect();
        let trees = [chain(&up, |_| false), chain(&down, |_| true), chain(&zig, |i| i % 2 == 1)];
        for (shape, tr) in ["up", "down", "zigzag"].iter().zip(&trees) {
            assert!(is_valid_bst(tr.as_deref()), "{shape} path of {n} nodes");
        }
    });
}

#[test]
fn large_bst_valid_and_with_one_bad_leaf() {
    let mut rng = Rng(5656);
    let mut tr = rand_bst(&mut rng, 10_000, -1_000_000_000, 1_000_000_000);
    assert!(is_valid_bst(tr.as_deref()));
    // Break the deepest-leftmost leaf: make it larger than the root.
    let mut cur = tr.as_deref_mut().expect("tree has 10_000 nodes");
    let root_val = cur.val;
    loop {
        if cur.left.is_some() {
            cur = cur.left.as_deref_mut().expect("checked is_some");
        } else if cur.right.is_some() {
            cur = cur.right.as_deref_mut().expect("checked is_some");
        } else {
            break;
        }
    }
    cur.val = root_val + 1;
    assert!(!is_valid_bst(tr.as_deref()));
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

/// Random shape with `n` nodes; values drawn from `val` in preorder.
fn rand_tree(rng: &mut Rng, n: usize, val: &mut dyn FnMut(&mut Rng) -> i32) -> Tree {
    if n == 0 {
        return None;
    }
    let mut node = leaf(val(rng));
    let left = rng.below(n);
    node.left = rand_tree(rng, left, val);
    node.right = rand_tree(rng, n - 1 - left, val);
    Some(node)
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
