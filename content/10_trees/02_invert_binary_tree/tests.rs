// Tests for Invert Binary Tree — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(input: &str, expected: &str) {
    let got = invert_tree(t(input));
    assert_eq!(show(got.as_deref()), show(t(expected).as_deref()), "input {input}");
    assert_eq!(got, t(expected), "input {input}");
}

fn brute_mirror(n: Option<&TreeNode>) -> Tree {
    n.map(|n| {
        Box::new(TreeNode {
            val: n.val,
            left: brute_mirror(n.right.as_deref()),
            right: brute_mirror(n.left.as_deref()),
        })
    })
}

#[test]
fn example_1() {
    check("[4,2,7,1,3,6,9]", "[4,7,2,9,6,3,1]");
}

#[test]
fn example_2() {
    check("[2,1,3]", "[2,3,1]");
}

#[test]
fn example_3_empty() {
    assert_eq!(invert_tree(None), None);
}

#[test]
fn single_node() {
    check("[1]", "[1]");
}

#[test]
fn only_left_child_moves_right() {
    check("[1,2]", "[1,null,2]");
}

#[test]
fn only_right_child_moves_left() {
    check("[1,null,2]", "[1,2]");
}

#[test]
fn must_recurse_not_just_swap_root() {
    // Swapping only the root's children leaves the grandchildren unmirrored.
    check("[1,2,3,4,5,6,7]", "[1,3,2,7,6,5,4]");
}

#[test]
fn lopsided_tree() {
    check("[1,2,null,3,4,null,null,5]", "[1,null,2,4,3,null,5]");
}

#[test]
fn negatives_and_duplicates() {
    check("[-1,-1,0,-100,null,100,-1]", "[-1,0,-1,-1,100,null,-100]");
}

#[test]
fn inverting_twice_is_identity() {
    let mut rng = Rng(0x46);
    for _ in 0..200 {
        let n = rng.below(30);
        let orig = rand_tree(&mut rng, n, &mut |r| r.range(-100, 100));
        let input = show(orig.as_deref());
        let twice = invert_tree(invert_tree(orig.clone()));
        assert_eq!(twice, orig, "input {input}");
    }
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(0x4646_4646);
    for _ in 0..300 {
        let n = rng.below(40);
        let orig = rand_tree(&mut rng, n, &mut |r| r.range(-100, 100));
        let expected = brute_mirror(orig.as_deref());
        let input = show(orig.as_deref());
        assert_eq!(invert_tree(orig), expected, "input {input}");
    }
}

#[test]
fn result_is_a_plain_owned_tree() {
    // The returned tree is independent of anything else: it can be mutated
    // and compared like any other value.
    let mut got = invert_tree(t("[1,2,3]"));
    if let Some(root) = got.as_deref_mut() {
        root.val = 10;
    }
    assert_eq!(got, t("[10,3,2]"));
}

#[test]
fn deep_left_chain_becomes_right_chain() {
    big_stack(|| {
        let vals: Vec<i32> = (0..2000).map(|i| i % 201 - 100).collect();
        let got = invert_tree(chain(&vals, |_| true));
        assert_eq!(level(got.as_deref()), level(chain(&vals, |_| false).as_deref()));
    });
}

#[test]
fn deep_zigzag_chain() {
    big_stack(|| {
        let vals: Vec<i32> = (0..2000).map(|i| i % 100).collect();
        let got = invert_tree(chain(&vals, |i| i % 2 == 0));
        assert_eq!(level(got.as_deref()), level(chain(&vals, |i| i % 2 == 1).as_deref()));
    });
}

#[test]
fn large_complete_tree() {
    let vals = || (0..100_000).map(|i| i % 201 - 100);
    let expected = level(brute_mirror(complete(vals()).as_deref()).as_deref());
    assert_eq!(level(invert_tree(complete(vals())).as_deref()), expected);
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
