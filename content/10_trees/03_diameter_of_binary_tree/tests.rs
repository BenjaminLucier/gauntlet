// Tests for Diameter of Binary Tree — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(input: &str, expected: usize) {
    assert_eq!(diameter_of_binary_tree(t(input).as_deref()), expected, "input {input}");
}

fn brute_height(n: Option<&TreeNode>) -> usize {
    n.map_or(0, |n| {
        1 + brute_height(n.left.as_deref()).max(brute_height(n.right.as_deref()))
    })
}

/// O(n^2): try every node as the bend point.
fn brute_diameter(n: Option<&TreeNode>) -> usize {
    n.map_or(0, |n| {
        let (l, r) = (n.left.as_deref(), n.right.as_deref());
        let here = brute_height(l) + brute_height(r);
        here.max(brute_diameter(l)).max(brute_diameter(r))
    })
}

#[test]
fn example_1() {
    check("[1,2,3,4,5]", 3);
}

#[test]
fn example_2() {
    check("[1,2]", 1);
}

#[test]
fn empty_tree_is_zero() {
    assert_eq!(diameter_of_binary_tree(None), 0);
}

#[test]
fn single_node_is_zero() {
    check("[1]", 0);
}

#[test]
fn counts_edges_not_nodes() {
    check("[1,2,3]", 2);
}

#[test]
fn path_not_through_root() {
    // Longest path bends at node 2: 5 -> 3 -> 2 -> 4 -> 6 (4 edges);
    // anything through the root is at most 3 edges.
    check("[1,2,null,3,4,5,null,null,6]", 4);
}

#[test]
fn deep_subtree_hanging_off_one_side() {
    check("[1,null,2,3,4,5,null,null,6,7,null,null,8]", 6);
}

#[test]
fn straight_chain() {
    check("[1,2,null,3,null,4]", 3);
    check("[1,null,2,null,3,null,4]", 3);
}

#[test]
fn values_do_not_matter() {
    check("[-100,-100,-100,-100,-100]", 3);
}

#[test]
fn full_tree() {
    check("[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]", 6);
}

#[test]
fn works_on_a_borrowed_subtree() {
    let tr = t("[1,2,null,3,4,5,null,null,6]");
    let left = tr.as_deref().and_then(|n| n.left.as_deref());
    assert_eq!(diameter_of_binary_tree(left), 4);
    assert_eq!(diameter_of_binary_tree(tr.as_deref()), 4);
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(48);
    for _ in 0..300 {
        let n = rng.below(60);
        let tr = rand_tree(&mut rng, n, &mut |r| r.range(-100, 100));
        let expected = brute_diameter(tr.as_deref());
        assert_eq!(
            diameter_of_binary_tree(tr.as_deref()),
            expected,
            "input {}",
            show(tr.as_deref())
        );
    }
}

#[test]
fn deep_chain() {
    big_stack(|| {
        let vals: Vec<i32> = (0..2000).map(|i| i % 100).collect();
        assert_eq!(diameter_of_binary_tree(chain(&vals, |_| true).as_deref()), 1999);
        assert_eq!(diameter_of_binary_tree(chain(&vals, |i| i % 2 == 0).as_deref()), 1999);
    });
}

#[test]
fn two_deep_arms() {
    // Root with a left arm of 1000 and a right arm of 999 nodes.
    big_stack(|| {
        let root = TreeNode {
            val: 0,
            left: chain(&[1; 1000], |_| true),
            right: chain(&[2; 999], |_| false),
        };
        assert_eq!(diameter_of_binary_tree(Some(&root)), 1999);
    });
}

#[test]
fn long_skewed_paths() {
    // Two paths of 10^4 nodes (straight left, zigzag). Recomputing the height
    // below every node is ~n^2 / 2 = 5 * 10^7 steps per tree; one DFS is 10^4.
    big_stack(|| {
        let n = 10_000;
        let vals: Vec<i32> = (0..n).map(|i| i % 100).collect();
        let left = chain(&vals, |_| true);
        let zigzag = chain(&vals, |i| i % 2 == 0);
        for (name, tr) in [("left", &left), ("zigzag", &zigzag)] {
            assert_eq!(diameter_of_binary_tree(tr.as_deref()), n as usize - 1, "n = {n}, {name} path");
        }
    });
}

#[test]
fn large_complete_tree() {
    // 10^4 nodes: 13 full levels plus a partial 14th level on the left.
    let tr = complete((0..10_000).map(|i| i % 100));
    let expected = brute_diameter(tr.as_deref());
    assert_eq!(diameter_of_binary_tree(tr.as_deref()), expected);
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

/// Complete tree holding `vals` in level order.
fn complete(vals: impl IntoIterator<Item = i32>) -> Tree {
    tree(&vals.into_iter().map(Some).collect::<Vec<_>>())
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
