// Tests for Subtree of Another Tree — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(root: &str, sub: &str, expected: bool) {
    let (r, s) = (t(root), t(sub));
    assert_eq!(is_subtree(r.as_deref(), s.as_deref()), expected, "root = {root}, sub = {sub}");
}

fn all_nodes(root: Option<&TreeNode>) -> Vec<&TreeNode> {
    let mut out = vec![];
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if let Some(n) = n {
            stack.push(n.left.as_deref());
            stack.push(n.right.as_deref());
            out.push(n);
        }
    }
    out
}

/// Compare the level order of every subtree with the target's.
fn brute_is_subtree(root: Option<&TreeNode>, sub: Option<&TreeNode>) -> bool {
    if sub.is_none() {
        return true;
    }
    let target = level(sub);
    all_nodes(root).into_iter().any(|n| level(Some(n)) == target)
}

#[test]
fn example_1() {
    check("[3,4,5,1,2]", "[4,1,2]", true);
}

#[test]
fn example_2() {
    check("[3,4,5,1,2,null,null,null,null,0]", "[4,1,2]", false);
}

#[test]
fn empty_sub_is_always_a_subtree() {
    assert!(is_subtree(t("[1,2,3]").as_deref(), None));
    assert!(is_subtree(t("[1]").as_deref(), None));
    assert!(is_subtree(None, None));
}

#[test]
fn nothing_non_empty_is_inside_the_empty_tree() {
    assert!(!is_subtree(None, t("[1]").as_deref()));
    assert!(!is_subtree(None, t("[0,1,2]").as_deref()));
}

#[test]
fn single_equal_nodes() {
    check("[1]", "[1]", true);
    check("[1]", "[2]", false);
}

#[test]
fn whole_tree_is_its_own_subtree() {
    check("[1,2,3,4,null,5]", "[1,2,3,4,null,5]", true);
}

#[test]
fn sub_is_a_leaf() {
    check("[1,2,3,4,null,5]", "[5]", true);
    check("[1,2,3,4,null,5]", "[3]", false); // 3 is not a leaf in root
}

#[test]
fn sub_bigger_than_root() {
    check("[1]", "[1,2]", false);
}

#[test]
fn match_is_not_at_the_first_equal_value() {
    // The root's value 4 matches but its subtree doesn't; its left child does.
    check("[4,4,null,1,2]", "[4,1,2]", true);
    check("[1,1,null,1,null,1,2]", "[1,null,2]", false);
    check("[1,1,null,1,null,1,null,null,2]", "[1,null,2]", true);
}

#[test]
fn matching_prefix_is_not_enough() {
    // sub matches the top of the subtree at 2 but root has more below.
    check("[1,2,3,4,5]", "[2,4]", false);
    check("[1,2,3,4,5]", "[2,4,5]", true);
}

#[test]
fn naive_string_search_trap() {
    // Concatenating values without delimiters makes "12" contain "2".
    check("[12]", "[2]", false);
    check("[1,22]", "[2]", false);
    check("[-2]", "[2]", false);
}

#[test]
fn shape_matters_not_just_values() {
    check("[3,4,5,1]", "[4,null,1]", false);
    check("[3,4,5,null,1]", "[4,null,1]", true);
}

#[test]
fn borrowed_subtree_of_the_same_tree() {
    // Both arguments may borrow from one tree.
    let tr = t("[1,2,3,4,5,6,7]");
    let right = tr.as_deref().and_then(|n| n.right.as_deref());
    assert!(is_subtree(tr.as_deref(), right));
    assert!(!is_subtree(right, tr.as_deref()));
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(51);
    let (mut yes, mut no) = (0, 0);
    for _ in 0..400 {
        let n = 1 + rng.below(30);
        let root = rand_tree(&mut rng, n, &mut |r| r.range(0, 2));
        let sub = if rng.below(2) == 0 {
            // a copy of a real subtree, sometimes with one value tweaked
            let nodes = all_nodes(root.as_deref());
            let mut lv = level(Some(nodes[rng.below(nodes.len())]));
            if rng.below(3) == 0 {
                let i = rng.below(lv.len());
                if lv[i].is_some() {
                    lv[i] = Some(rng.range(0, 2));
                }
            }
            tree(&lv)
        } else {
            let m = 1 + rng.below(4);
            rand_tree(&mut rng, m, &mut |r| r.range(0, 2))
        };
        let expected = brute_is_subtree(root.as_deref(), sub.as_deref());
        if expected {
            yes += 1;
        } else {
            no += 1;
        }
        assert_eq!(
            is_subtree(root.as_deref(), sub.as_deref()),
            expected,
            "root = {}, sub = {}",
            show(root.as_deref()),
            show(sub.as_deref())
        );
    }
    assert!(yes > 50 && no > 50);
}

#[test]
fn deep_chain_contains_its_tail() {
    big_stack(|| {
        let ones = vec![1; 2000];
        let long = chain(&ones, |_| true);
        let tail = chain(&ones[..1000], |_| true);
        assert!(is_subtree(long.as_deref(), tail.as_deref()));
        let mut vals = vec![1; 1000];
        vals[999] = 2;
        let wrong_end = chain(&vals, |_| true);
        assert!(!is_subtree(long.as_deref(), wrong_end.as_deref()));
        let wrong_side = chain(&ones[..1000], |_| false);
        assert!(!is_subtree(long.as_deref(), wrong_side.as_deref()));
    });
}

#[test]
fn large_tree_of_equal_values() {
    // Every node has the same value, so value comparison alone filters nothing.
    let root = complete(vec![7; 2000]);
    for m in [1, 3, 7, 15, 31, 1000] {
        let sub = complete(vec![7; m]);
        let expected = brute_is_subtree(root.as_deref(), sub.as_deref());
        assert_eq!(
            is_subtree(root.as_deref(), sub.as_deref()),
            expected,
            "complete sub of size {m}"
        );
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
