// Tests for Construct Binary Tree from Preorder and Inorder Traversal — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(pre: &[i32], ino: &[i32], expected: &str) {
    let got = build_tree(pre, ino).expect("consistent traversals");
    assert_eq!(show(got.as_deref()), expected, "preorder {pre:?}, inorder {ino:?}");
    assert_eq!(got, t(expected), "preorder {pre:?}, inorder {ino:?}");
}

fn preorder(n: Option<&TreeNode>, out: &mut Vec<i32>) {
    if let Some(n) = n {
        out.push(n.val);
        preorder(n.left.as_deref(), out);
        preorder(n.right.as_deref(), out);
    }
}

/// Traversals of `tr`, then rebuild and compare.
fn round_trip(tr: &Tree) {
    let mut pre = vec![];
    preorder(tr.as_deref(), &mut pre);
    let ino = inorder(tr.as_deref());
    let got = build_tree(&pre, &ino);
    assert_eq!(got.as_ref(), Ok(tr), "preorder {pre:?}, inorder {ino:?}");
}

/// Random tree of `n` nodes with distinct values from -3000..=3000.
fn rand_unique_tree(rng: &mut Rng, n: usize) -> Tree {
    let mut pool: Vec<i32> = (-3000..=3000).collect();
    for i in 0..n {
        let j = i + rng.below(pool.len() - i);
        pool.swap(i, j);
    }
    let mut it = pool.into_iter();
    rand_tree(rng, n, &mut |_| it.next().expect("pool has 6001 values"))
}

/// O(n^2) slice-splitting rebuild, for traversals of distinct values with
/// equal lengths. Succeeds exactly when the traversals describe one tree.
fn brute_build(pre: &[i32], ino: &[i32]) -> Result<Tree, BuildError> {
    let Some((&val, rest)) = pre.split_first() else { return Ok(None) };
    let k = ino.iter().position(|&v| v == val).ok_or(BuildError::Inconsistent)?;
    let left = brute_build(&rest[..k], &ino[..k])?;
    let right = brute_build(&rest[k..], &ino[k + 1..])?;
    Ok(Some(Box::new(TreeNode { val, left, right })))
}

#[test]
fn example_1() {
    check(&[3, 9, 20, 15, 7], &[9, 3, 15, 20, 7], "[3,9,20,null,null,15,7]");
}

#[test]
fn example_2() {
    check(&[-1], &[-1], "[-1]");
}

#[test]
fn empty_traversals_give_empty_tree() {
    assert_eq!(build_tree(&[], &[]), Ok(None));
}

#[test]
fn two_nodes_left_vs_right() {
    // Same preorder, different inorder -> different shapes.
    check(&[1, 2], &[2, 1], "[1,2]");
    check(&[1, 2], &[1, 2], "[1,null,2]");
}

#[test]
fn left_skewed() {
    check(&[4, 3, 2, 1], &[1, 2, 3, 4], "[4,3,null,2,null,1]");
}

#[test]
fn right_skewed() {
    check(&[1, 2, 3, 4], &[1, 2, 3, 4], "[1,null,2,null,3,null,4]");
}

#[test]
fn full_tree() {
    check(&[1, 2, 4, 5, 3, 6, 7], &[4, 2, 5, 1, 6, 3, 7], "[1,2,3,4,5,6,7]");
}

#[test]
fn values_are_not_sorted_or_positive() {
    check(&[0, -3000, 3000, 7, -5], &[3000, -3000, 0, -5, 7], "[0,-3000,7,3000,null,-5]");
}

#[test]
fn zigzag() {
    check(&[1, 2, 3, 4], &[2, 4, 3, 1], "[1,2,null,null,3,4]");
}

#[test]
fn err_length_mismatch() {
    assert_eq!(
        build_tree(&[1, 2], &[1]),
        Err(BuildError::LengthMismatch { preorder: 2, inorder: 1 })
    );
    assert_eq!(
        build_tree(&[], &[5]),
        Err(BuildError::LengthMismatch { preorder: 0, inorder: 1 })
    );
}

#[test]
fn err_duplicate_values() {
    assert_eq!(build_tree(&[1, 1], &[1, 1]), Err(BuildError::DuplicateValue(1)));
    // Duplicated in only one of the traversals.
    assert_eq!(build_tree(&[2, 1, 3], &[1, 2, 1]), Err(BuildError::DuplicateValue(1)));
    assert_eq!(build_tree(&[2, 2, 3], &[3, 2, 1]), Err(BuildError::DuplicateValue(2)));
}

#[test]
fn err_value_missing_from_inorder() {
    assert_eq!(build_tree(&[1, 2, 3], &[2, 1, 4]), Err(BuildError::Inconsistent));
    assert_eq!(build_tree(&[7], &[8]), Err(BuildError::Inconsistent));
}

#[test]
fn err_same_values_but_no_such_tree() {
    // Root 1 puts 3 on the left and 2 on the right, but preorder visits 2 first.
    assert_eq!(build_tree(&[1, 2, 3], &[3, 1, 2]), Err(BuildError::Inconsistent));
    // Only wrong deep down: the root splits fine, but in the left subtree
    // preorder says 4 comes before 5 while inorder puts 5 left of 2.
    assert_eq!(build_tree(&[1, 2, 4, 5, 3, 6, 7], &[5, 2, 4, 1, 6, 3, 7]), Err(BuildError::Inconsistent));
}

#[test]
fn randomized_round_trip() {
    let mut rng = Rng(58);
    for _ in 0..300 {
        let n = 1 + rng.below(40);
        round_trip(&rand_unique_tree(&mut rng, n));
    }
}

#[test]
fn randomized_permutations_vs_brute_force() {
    // Random pairs of permutations: mostly inconsistent, some valid.
    let mut rng = Rng(0x5858);
    let (mut ok, mut bad) = (0, 0);
    for _ in 0..2000 {
        let n = 1 + rng.below(5);
        let mut pre: Vec<i32> = (1..=n as i32).collect();
        let mut ino = pre.clone();
        for v in [&mut pre, &mut ino] {
            for i in (1..v.len()).rev() {
                v.swap(i, rng.below(i + 1));
            }
        }
        let expected = brute_build(&pre, &ino);
        if expected.is_ok() {
            ok += 1;
        } else {
            bad += 1;
        }
        assert_eq!(build_tree(&pre, &ino), expected, "preorder {pre:?}, inorder {ino:?}");
    }
    assert!(ok > 100 && bad > 100, "ok {ok}, bad {bad}");
}

#[test]
fn deep_chains() {
    big_stack(|| {
        let vals: Vec<i32> = (-1500..1500).collect();
        round_trip(&chain(&vals, |_| true));
        round_trip(&chain(&vals, |_| false));
        round_trip(&chain(&vals, |i| i % 2 == 0));
        round_trip(&chain(&vals, |i| (i / 7) % 2 == 0));
    });
}

#[test]
fn deep_chain_inconsistent_at_the_bottom() {
    big_stack(|| {
        let vals: Vec<i32> = (-1500..1500).collect();
        let tr = chain(&vals, |_| false);
        let mut pre = vec![];
        preorder(tr.as_deref(), &mut pre);
        let mut ino = inorder(tr.as_deref());
        // Inorder of the last three nodes says 1499 is left of 1497, but
        // preorder says 1498 is 1497's first child.
        ino.truncate(2997);
        ino.extend([1499, 1497, 1498]);
        assert_eq!(build_tree(&pre, &ino), Err(BuildError::Inconsistent));
    });
}

#[test]
fn large_random_tree() {
    big_stack(|| {
        let mut rng = Rng(5858);
        for _ in 0..3 {
            round_trip(&rand_unique_tree(&mut rng, 3000));
        }
    });
}

#[test]
fn large_complete_tree() {
    round_trip(&complete(-1500..1500));
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
