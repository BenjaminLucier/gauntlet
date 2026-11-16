// Tests for Tree Traversals — shared by template.rs, solution.rs and your attempt.
use super::*;

/// All three depth-first orders, from both the recursive and the iterative
/// versions, checked against each other and against `pre`, `ino`, `post`.
fn check_dfs(root: Option<&TreeNode>, pre: &[i32], ino: &[i32], post: &[i32]) {
    let input = show(root);
    assert_eq!(preorder(root), pre, "preorder of {input}");
    assert_eq!(inorder(root), ino, "inorder of {input}");
    assert_eq!(postorder(root), post, "postorder of {input}");
    assert_eq!(preorder_iter(root), pre, "preorder_iter of {input}");
    assert_eq!(inorder_iter(root), ino, "inorder_iter of {input}");
    assert_eq!(postorder_iter(root), post, "postorder_iter of {input}");
}

fn check_levels(input: &str, expected: &[&[i32]]) {
    let expected: Vec<Vec<i32>> = expected.iter().map(|r| r.to_vec()).collect();
    assert_eq!(level_order(t(input).as_deref()), expected, "input {input}");
}

#[derive(Clone, Copy)]
enum Order {
    Pre,
    In,
    Post,
}

fn brute_order(n: Option<&TreeNode>, order: Order, out: &mut Vec<i32>) {
    if let Some(n) = n {
        if let Order::Pre = order {
            out.push(n.val);
        }
        brute_order(n.left.as_deref(), order, out);
        if let Order::In = order {
            out.push(n.val);
        }
        brute_order(n.right.as_deref(), order, out);
        if let Order::Post = order {
            out.push(n.val);
        }
    }
}

fn brute(n: Option<&TreeNode>, order: Order) -> Vec<i32> {
    let mut out = vec![];
    brute_order(n, order, &mut out);
    out
}

fn brute_levels(n: Option<&TreeNode>, depth: usize, out: &mut Vec<Vec<i32>>) {
    if let Some(n) = n {
        if out.len() == depth {
            out.push(vec![]);
        }
        out[depth].push(n.val);
        brute_levels(n.left.as_deref(), depth + 1, out);
        brute_levels(n.right.as_deref(), depth + 1, out);
    }
}

#[test]
fn readme_example() {
    let tree = t("[1,2,3,4,5,null,6]");
    let root = tree.as_deref();
    assert_eq!(preorder(root), [1, 2, 4, 5, 3, 6]);
    assert_eq!(inorder(root), [4, 2, 5, 1, 3, 6]);
    assert_eq!(postorder(root), [4, 5, 2, 6, 3, 1]);
    assert_eq!(inorder_iter(root), inorder(root));
    assert_eq!(level_order(root), [vec![1], vec![2, 3], vec![4, 5, 6]]);
    assert!(preorder(None).is_empty());
    assert!(level_order(None).is_empty());
    check_dfs(root, &[1, 2, 4, 5, 3, 6], &[4, 2, 5, 1, 3, 6], &[4, 5, 2, 6, 3, 1]);
}

#[test]
fn empty_tree() {
    check_dfs(None, &[], &[], &[]);
    assert!(level_order(None).is_empty());
}

#[test]
fn single_node() {
    let tr = t("[7]");
    check_dfs(tr.as_deref(), &[7], &[7], &[7]);
    check_levels("[7]", &[&[7]]);
}

#[test]
fn small_left_chain() {
    let tr = t("[1,2,null,3]");
    check_dfs(tr.as_deref(), &[1, 2, 3], &[3, 2, 1], &[3, 2, 1]);
    check_levels("[1,2,null,3]", &[&[1], &[2], &[3]]);
}

#[test]
fn small_right_chain() {
    let tr = t("[1,null,2,null,3]");
    check_dfs(tr.as_deref(), &[1, 2, 3], &[1, 2, 3], &[3, 2, 1]);
    check_levels("[1,null,2,null,3]", &[&[1], &[2], &[3]]);
}

#[test]
fn inorder_of_a_bst_is_sorted() {
    let tr = t("[4,2,6,1,3,5,7]");
    let sorted: Vec<i32> = (1..=7).collect();
    assert_eq!(inorder(tr.as_deref()), sorted);
    assert_eq!(inorder_iter(tr.as_deref()), sorted);
}

#[test]
fn postorder_is_not_reversed_preorder() {
    // Reversing a plain preorder gives [3, 2, 1]; postorder is [2, 3, 1].
    let tr = t("[1,2,3]");
    check_dfs(tr.as_deref(), &[1, 2, 3], &[2, 1, 3], &[2, 3, 1]);
}

#[test]
fn stack_pushes_right_child_before_left() {
    // Pushing left first would emit the right subtree first.
    let tr = t("[1,2,3,4,5,6,7]");
    check_dfs(
        tr.as_deref(),
        &[1, 2, 4, 5, 3, 6, 7],
        &[4, 2, 5, 1, 6, 3, 7],
        &[4, 5, 2, 6, 7, 3, 1],
    );
}

#[test]
fn inorder_iter_resumes_at_the_right_child() {
    // 2 has only a right child; 3's left child 5 must come before 3.
    let tr = t("[1,2,3,null,4,5]");
    check_dfs(tr.as_deref(), &[1, 2, 4, 3, 5], &[2, 4, 1, 5, 3], &[4, 2, 5, 3, 1]);
}

#[test]
fn level_order_skips_gaps_and_mixes_cousins() {
    check_levels("[1,2,3,null,4,null,5]", &[&[1], &[2, 3], &[4, 5]]);
    // 4 is under 2 and 5 under 3, but they sit on the same level.
    check_levels("[1,2,3,4,null,null,5,6,null,null,7]", &[&[1], &[2, 3], &[4, 5], &[6, 7]]);
    check_levels("[3,9,20,null,null,15,7]", &[&[3], &[9, 20], &[15, 7]]);
}

#[test]
fn negatives_duplicates_and_extremes() {
    let tr = t("[0,-1000000000,1000000000,-1000000000,null,1000000000]");
    let (lo, hi) = (-1_000_000_000, 1_000_000_000);
    check_dfs(tr.as_deref(), &[0, lo, lo, hi, hi], &[lo, lo, 0, hi, hi], &[lo, lo, hi, hi, 0]);
    check_levels(
        "[0,-1000000000,1000000000,-1000000000,null,1000000000]",
        &[&[0], &[lo, hi], &[lo, hi]],
    );
}

#[test]
fn tree_is_only_borrowed() {
    // Read-only: the caller still owns the tree and can traverse it again.
    let tr = t("[1,2,3]");
    let first = preorder_iter(tr.as_deref());
    assert_eq!(preorder_iter(tr.as_deref()), first);
    assert_eq!(level_order(tr.as_deref()), [vec![1], vec![2, 3]]);
    assert_eq!(tr, t("[1,2,3]"));
    // A subtree is just another `&TreeNode`.
    let right = tr.as_deref().and_then(|n| n.right.as_deref());
    check_dfs(right, &[3], &[3], &[3]);
    assert_eq!(level_order(right), [vec![3]]);
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(53);
    for _ in 0..400 {
        let n = rng.below(60);
        let tr = rand_tree(&mut rng, n, &mut |r| r.range(-1000, 1000));
        let root = tr.as_deref();
        check_dfs(
            root,
            &brute(root, Order::Pre),
            &brute(root, Order::In),
            &brute(root, Order::Post),
        );
        let mut levels = vec![];
        brute_levels(root, 0, &mut levels);
        assert_eq!(level_order(root), levels, "level_order of {}", show(root));
    }
}

#[test]
fn recursive_versions_on_a_deep_zigzag() {
    // Run with a big stack: recursion is fine here, the tree is 5 000 deep.
    big_stack(|| {
        let vals: Vec<i32> = (0..5000).collect();
        let tr = Deep(chain(&vals, |i| i.is_multiple_of(3)));
        let root = tr.0.as_deref();
        assert_eq!(preorder(root), brute(root, Order::Pre));
        assert_eq!(inorder(root), brute(root, Order::In));
        assert_eq!(postorder(root), brute(root, Order::Post));
    });
}

const DEEP: i32 = 100_000;

#[test]
fn iterative_versions_on_a_100k_left_chain() {
    moderate_stack(|| {
        let vals: Vec<i32> = (0..DEEP).collect();
        let tr = Deep(chain(&vals, |_| true));
        let root = tr.0.as_deref();
        let rev: Vec<i32> = vals.iter().rev().copied().collect();
        assert_eq!(preorder_iter(root), vals);
        assert_eq!(inorder_iter(root), rev);
        assert_eq!(postorder_iter(root), rev);
    });
}

#[test]
fn iterative_versions_on_a_100k_right_chain() {
    moderate_stack(|| {
        let vals: Vec<i32> = (0..DEEP).collect();
        let tr = Deep(chain(&vals, |_| false));
        let root = tr.0.as_deref();
        let rev: Vec<i32> = vals.iter().rev().copied().collect();
        assert_eq!(preorder_iter(root), vals);
        assert_eq!(inorder_iter(root), vals);
        assert_eq!(postorder_iter(root), rev);
    });
}

#[test]
fn iterative_versions_on_a_100k_zigzag() {
    moderate_stack(|| {
        let vals: Vec<i32> = (0..DEEP).map(|i| i * 7 % 1000 - 500).collect();
        let go_left = |i: usize| i.is_multiple_of(2);
        let tr = Deep(chain(&vals, go_left));
        let root = tr.0.as_deref();
        // On a path, inorder puts a node after its child if the child hangs left.
        let mut ino = VecDeque::from([vals[vals.len() - 1]]);
        for (i, &v) in vals.iter().enumerate().rev().skip(1) {
            if go_left(i) {
                ino.push_back(v);
            } else {
                ino.push_front(v);
            }
        }
        let rev: Vec<i32> = vals.iter().rev().copied().collect();
        assert_eq!(preorder_iter(root), vals);
        assert_eq!(inorder_iter(root), Vec::from(ino));
        assert_eq!(postorder_iter(root), rev);
    });
}

#[test]
fn level_order_on_a_100k_chain() {
    moderate_stack(|| {
        let vals: Vec<i32> = (0..DEEP).collect();
        let tr = Deep(chain(&vals, |i| i % 5 < 2));
        let got = level_order(tr.0.as_deref());
        let expected: Vec<Vec<i32>> = vals.iter().map(|&v| vec![v]).collect();
        assert_eq!(got, expected);
    });
}

#[test]
fn large_complete_tree() {
    let n = 100_000;
    let tr = complete((0..n).map(|i| i % 2001 - 1000));
    let root = tr.as_deref();
    for (order, got, got_iter) in [
        (Order::Pre, preorder(root), preorder_iter(root)),
        (Order::In, inorder(root), inorder_iter(root)),
        (Order::Post, postorder(root), postorder_iter(root)),
    ] {
        let expected = brute(root, order);
        assert_eq!(got, expected);
        assert_eq!(got_iter, expected);
    }
    let levels = level_order(root);
    let mut expected = vec![];
    let mut start = 0;
    while start < n {
        let end = (2 * start + 1).min(n);
        expected.push((start..end).map(|i| i % 2001 - 1000).collect::<Vec<_>>());
        start = end;
    }
    assert_eq!(levels.len(), 17);
    assert_eq!(levels, expected);
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

/// Owns a possibly very deep tree and takes it apart iteratively when dropped
/// (also while a failing test unwinds): the default recursive drop of a
/// 100 000-node path would overflow the stack.
struct Deep(Tree);

impl Drop for Deep {
    fn drop(&mut self) {
        let mut stack: Vec<Box<TreeNode>> = self.0.take().into_iter().collect();
        while let Some(mut node) = stack.pop() {
            stack.extend(node.left.take());
            stack.extend(node.right.take());
        }
    }
}

/// Complete tree holding `vals` in level order.
fn complete(vals: impl IntoIterator<Item = i32>) -> Tree {
    tree(&vals.into_iter().map(Some).collect::<Vec<_>>())
}

/// Run `f` on a thread with a fixed, moderate 1 MiB stack, so the 100k-deep
/// tests behave the same whatever stack the test harness gives its own
/// threads (`RUST_MIN_STACK`): an iterative walk needs almost none of it, a
/// recursive one runs out long before the bottom. Panics (including
/// "not yet implemented") are re-raised on the test's thread. Note that Rust
/// cannot catch a stack overflow: it still aborts the whole test binary, so
/// if the run stops with "has overflowed its stack", one of the `_iter`
/// functions or `level_order` is recursing.
fn moderate_stack(f: impl FnOnce() + Send + 'static) {
    let h = std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(f)
        .expect("spawn test thread");
    if let Err(e) = h.join() {
        std::panic::resume_unwind(e);
    }
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
