// Tests for Binary Tree Maximum Path Sum — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(input: &str, expected: i32) {
    assert_eq!(max_path_sum(t(input).as_deref()), Some(expected), "input {input}");
}

/// O(n^2): turn the tree into an undirected graph and sum every path from every start.
fn brute_max_path(tr: Option<&TreeNode>) -> Option<i32> {
    let (mut vals, mut adj): (Vec<i32>, Vec<Vec<usize>>) = (vec![], vec![]);
    let mut stack: Vec<(&TreeNode, Option<usize>)> = tr.map(|n| (n, None)).into_iter().collect();
    while let Some((n, parent)) = stack.pop() {
        let id = vals.len();
        vals.push(n.val);
        adj.push(vec![]);
        if let Some(p) = parent {
            adj[p].push(id);
            adj[id].push(p);
        }
        for c in [n.left.as_deref(), n.right.as_deref()].into_iter().flatten() {
            stack.push((c, Some(id)));
        }
    }
    let mut best = None;
    for s in 0..vals.len() {
        let mut st = vec![(s, usize::MAX, vals[s])];
        while let Some((u, from, sum)) = st.pop() {
            best = best.max(Some(sum));
            for &v in &adj[u] {
                if v != from {
                    st.push((v, u, sum + vals[v]));
                }
            }
        }
    }
    best
}

/// Kadane: on a path-shaped tree the answer is the max subarray sum.
fn kadane(vals: &[i32]) -> i32 {
    let (mut best, mut cur) = (i32::MIN, 0);
    for &v in vals {
        cur = v.max(cur + v);
        best = best.max(cur);
    }
    best
}

#[test]
fn example_1() {
    check("[1,2,3]", 6);
}

#[test]
fn example_2_path_avoids_root() {
    check("[-10,9,20,null,null,15,7]", 42);
}

#[test]
fn empty_tree_has_no_path() {
    assert_eq!(max_path_sum(None), None);
}

#[test]
fn single_negative_node() {
    // Starting the answer at 0 returns 0 here.
    check("[-3]", -3);
}

#[test]
fn all_negative_picks_largest_single_node() {
    check("[-2,-1,-3]", -1);
    check("[-5,-4,-6,-7,-1000]", -4);
}

#[test]
fn negative_branch_is_left_out() {
    check("[2,-1]", 2);
    check("[1,-2,3]", 4);
}

#[test]
fn path_cannot_fork() {
    // Both children of 2 are big, but a path through 1 may only use one of them.
    // Best is 4 + 2 + 5 = 11 (bend at 2), not 4 + 5 + 2 + 1 + 3.
    check("[1,2,-30,4,5]", 11);
    check("[10,2,10,20,1]", 42); // 20 -> 2 -> 10 -> 10
}

#[test]
fn bend_at_root_with_long_branches() {
    check("[5,4,8,11,null,13,4,7,2,null,null,null,1]", 48);
}

#[test]
fn zero_values() {
    check("[0]", 0);
    check("[0,-1,-1]", 0);
}

#[test]
fn extreme_values() {
    check("[1000,1000,1000,1000,1000,1000,1000]", 5000);
    check("[-1000,1000,1000]", 1000);
}

#[test]
fn works_on_a_borrowed_subtree() {
    // The function borrows: it can be pointed at any node, and the tree is
    // still usable afterwards.
    let tr = t("[-10,9,20,null,null,15,7]");
    let right = tr.as_deref().and_then(|n| n.right.as_deref());
    assert_eq!(max_path_sum(right), Some(42));
    let left = tr.as_deref().and_then(|n| n.left.as_deref());
    assert_eq!(max_path_sum(left), Some(9));
    assert_eq!(max_path_sum(tr.as_deref()), Some(42));
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(59);
    for _ in 0..300 {
        let n = rng.below(36);
        let (lo, hi) = [(-1000, 1000), (-10, 3), (-1000, -1)][rng.below(3)];
        let tr = rand_tree(&mut rng, n, &mut |r| r.range(lo, hi));
        let input = show(tr.as_deref());
        assert_eq!(max_path_sum(tr.as_deref()), brute_max_path(tr.as_deref()), "input {input}");
    }
}

#[test]
fn deep_chain_is_max_subarray() {
    big_stack(|| {
        let mut rng = Rng(5959);
        for _ in 0..5 {
            let vals: Vec<i32> = (0..2000).map(|_| rng.range(-1000, 1000)).collect();
            let shape = rng.next();
            let tr = chain(&vals, move |i| (shape >> (i % 64)) & 1 == 1);
            assert_eq!(max_path_sum(tr.as_deref()), Some(kadane(&vals)));
        }
        let all: Vec<i32> = vec![1000; 2000];
        assert_eq!(max_path_sum(chain(&all, |_| true).as_deref()), Some(2_000_000));
    });
}

#[test]
fn long_random_path() {
    // A random path of 1.5 * 10^4 nodes. Recomputing the best downward path
    // below every node costs ~n^2 / 2 = 10^8 steps; one DFS is 1.5 * 10^4.
    big_stack(|| {
        let n = 15_000;
        let mut rng = Rng(0x5A5A_1010);
        let vals: Vec<i32> = (0..n).map(|_| rng.range(-1000, 1000)).collect();
        let shape = rng.next();
        let tr = chain(&vals, move |i| (shape >> (i % 64)) & 1 == 1);
        assert_eq!(max_path_sum(tr.as_deref()), Some(kadane(&vals)), "n = {n}");
    });
}

#[test]
fn large_complete_tree() {
    // Array-form DP on a complete tree with 3 * 10^4 nodes.
    let mut rng = Rng(595959);
    let n = 30_000;
    let vals: Vec<i32> = (0..n).map(|_| rng.range(-1000, 1000)).collect();
    let mut down = vec![0; n];
    let mut expected = i32::MIN;
    for i in (0..n).rev() {
        let d = |c: usize| if c < n { down[c].max(0) } else { 0 };
        let (l, r) = (d(2 * i + 1), d(2 * i + 2));
        expected = expected.max(vals[i] + l + r);
        down[i] = vals[i] + l.max(r);
    }
    assert_eq!(max_path_sum(complete(vals).as_deref()), Some(expected));
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
