// Tests for Serialize and Deserialize Binary Tree — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Independent encoder: level order with every missing child spelled out.
fn brute_serialize(root: Option<&TreeNode>) -> String {
    let mut tokens = vec![];
    let mut frontier = vec![root];
    while !frontier.is_empty() {
        let mut next = vec![];
        for node in frontier {
            match node {
                Some(n) => {
                    tokens.push(n.val.to_string());
                    next.push(n.left.as_deref());
                    next.push(n.right.as_deref());
                }
                None => tokens.push("#".to_string()),
            }
        }
        frontier = next;
    }
    tokens.join(",")
}

/// Serialize, check the exact format, deserialize, compare.
fn round_trip(tr: Tree) {
    let data = serialize(tr.as_deref());
    assert_eq!(data, brute_serialize(tr.as_deref()), "encoding of {}", show(tr.as_deref()));
    let back = deserialize(&data);
    assert!(back.as_ref() == Ok(&tr), "round trip of {}", show(tr.as_deref()));
}

fn rt(s: &str) {
    round_trip(t(s));
}

#[test]
fn example_1() {
    assert_eq!(serialize(t("[1,2,3,null,null,4,5]").as_deref()), "1,2,3,#,#,4,5,#,#,#,#");
    assert_eq!(deserialize("1,2,3,#,#,4,5,#,#,#,#"), Ok(t("[1,2,3,null,null,4,5]")));
    rt("[1,2,3,null,null,4,5]");
}

#[test]
fn example_2_empty() {
    assert_eq!(serialize(None), "#");
    assert_eq!(deserialize("#"), Ok(None));
}

#[test]
fn example_errors() {
    assert_eq!(deserialize("1,x,#"), Err(ParseError::BadToken { index: 1 }));
    assert_eq!(deserialize("1,2,#"), Err(ParseError::UnexpectedEnd));
    assert_eq!(deserialize("1,#,#,7"), Err(ParseError::TrailingData { index: 3 }));
    assert_eq!(deserialize(""), Err(ParseError::BadToken { index: 0 }));
}

#[test]
fn single_node() {
    assert_eq!(serialize(t("[0]").as_deref()), "0,#,#");
    assert_eq!(serialize(t("[-7]").as_deref()), "-7,#,#");
    rt("[0]");
    rt("[-7]");
}

#[test]
fn shape_is_preserved() {
    // Same values, mirrored shapes: a values-only format can't tell these apart.
    assert_eq!(serialize(t("[1,2]").as_deref()), "1,2,#,#,#");
    assert_eq!(serialize(t("[1,null,2]").as_deref()), "1,#,2,#,#");
    rt("[1,2]");
    rt("[1,null,2]");
    rt("[1,2,null,null,3]");
    rt("[1,2,null,3]");
}

#[test]
fn negative_and_multi_digit_values() {
    rt("[-1000,1000,-1,10,-10,100,-100]");
    rt("[-1,-12,-123]");
}

#[test]
fn full_i32_range() {
    let s = "-2147483648,2147483647,0,#,#,#,#";
    assert_eq!(serialize(t("[-2147483648,2147483647,0]").as_deref()), s);
    assert_eq!(deserialize(s), Ok(t("[-2147483648,2147483647,0]")));
}

#[test]
fn repeated_values() {
    rt("[5,5,5,5,null,5,null,null,5]");
}

#[test]
fn sparse_tree() {
    rt("[1,null,2,3,null,null,4,5,6,null,7]");
}

#[test]
fn bad_tokens_report_their_index() {
    assert_eq!(deserialize("x"), Err(ParseError::BadToken { index: 0 }));
    assert_eq!(deserialize("1,#,abc"), Err(ParseError::BadToken { index: 2 }));
    assert_eq!(deserialize("1,,#"), Err(ParseError::BadToken { index: 1 }));
    // Spaces are not part of the format.
    assert_eq!(deserialize("1, 2,#,#,#"), Err(ParseError::BadToken { index: 1 }));
    assert_eq!(deserialize(" #"), Err(ParseError::BadToken { index: 0 }));
    // Brackets or `null` from other notations are not accepted either.
    assert_eq!(deserialize("[1,#,#]"), Err(ParseError::BadToken { index: 0 }));
    assert_eq!(deserialize("1,null,#"), Err(ParseError::BadToken { index: 1 }));
    assert_eq!(deserialize("1,##,#"), Err(ParseError::BadToken { index: 1 }));
}

#[test]
fn out_of_range_value_is_a_bad_token() {
    assert_eq!(deserialize("2147483648,#,#"), Err(ParseError::BadToken { index: 0 }));
    assert_eq!(deserialize("0,-2147483649,#,#,#"), Err(ParseError::BadToken { index: 1 }));
}

#[test]
fn missing_children_is_unexpected_end() {
    assert_eq!(deserialize("1"), Err(ParseError::UnexpectedEnd));
    assert_eq!(deserialize("1,#"), Err(ParseError::UnexpectedEnd));
    assert_eq!(deserialize("1,2,3,#,#,4,5,#,#,#"), Err(ParseError::UnexpectedEnd));
}

#[test]
fn leftover_tokens_are_trailing_data() {
    assert_eq!(deserialize("#,#"), Err(ParseError::TrailingData { index: 1 }));
    assert_eq!(deserialize("#,5"), Err(ParseError::TrailingData { index: 1 }));
    // A trailing comma is an extra (empty) token.
    assert_eq!(deserialize("1,#,#,"), Err(ParseError::TrailingData { index: 3 }));
    assert_eq!(deserialize("1,#,#,junk"), Err(ParseError::TrailingData { index: 3 }));
}

#[test]
fn first_problem_wins() {
    // The bad token comes before the input runs out.
    assert_eq!(deserialize("1,x"), Err(ParseError::BadToken { index: 1 }));
    // The tree is complete before the bad token, so it's trailing data.
    assert_eq!(deserialize("#,x"), Err(ParseError::TrailingData { index: 1 }));
}

#[test]
fn error_works_with_question_mark_and_dyn_error() {
    fn sum_of_root_children(data: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let tr = deserialize(data)?;
        let root = tr.ok_or("empty tree")?;
        Ok([&root.left, &root.right].iter().filter_map(|c| c.as_ref()).map(|c| c.val).sum())
    }
    assert_eq!(sum_of_root_children("1,2,3,#,#,#,#").ok(), Some(5));
    let err = sum_of_root_children("1,2").expect_err("truncated input");
    assert_eq!(err.to_string(), ParseError::UnexpectedEnd.to_string());
    assert!(!err.to_string().is_empty());
}

#[test]
fn many_trees_decoded_out_of_order() {
    // Nothing is remembered between calls: each string stands on its own.
    let inputs = ["[1,2,3]", "[]", "[4,null,5,6]", "[-1000]"];
    let encoded: Vec<String> = inputs.iter().map(|s| serialize(t(s).as_deref())).collect();
    for (s, data) in inputs.iter().zip(&encoded).rev() {
        assert_eq!(deserialize(data), Ok(t(s)), "tree {s}");
    }
}

#[test]
fn decode_twice_gives_independent_trees() {
    let data = serialize(t("[1,2,3]").as_deref());
    let mut a = deserialize(&data).expect("valid encoding");
    let b = deserialize(&data).expect("valid encoding");
    if let Some(root) = a.as_deref_mut() {
        root.val = 99;
    }
    assert_eq!(b, t("[1,2,3]"));
}

#[test]
fn serialize_only_borrows() {
    let tr = t("[1,2,3]");
    let left = tr.as_deref().and_then(|n| n.left.as_deref());
    assert_eq!(serialize(left), "2,#,#");
    assert_eq!(serialize(tr.as_deref()), "1,2,3,#,#,#,#");
    assert_eq!(tr, t("[1,2,3]"));
}

#[test]
fn randomized_round_trip() {
    let mut rng = Rng(60);
    for _ in 0..400 {
        let n = rng.below(60);
        let range = [1, 1000][rng.below(2)];
        round_trip(rand_tree(&mut rng, n, &mut |r| r.range(-range, range)));
    }
}

#[test]
fn randomized_corruptions_are_rejected() {
    let mut rng = Rng(0x6060);
    for _ in 0..300 {
        let n = 1 + rng.below(40);
        let tr = rand_tree(&mut rng, n, &mut |r| r.range(-1000, 1000));
        let data = brute_serialize(tr.as_deref());
        let tokens: Vec<&str> = data.split(',').collect();
        // Dropping the final `#` always leaves a node short of a child.
        let cut = tokens[..tokens.len() - 1].join(",");
        assert_eq!(deserialize(&cut), Err(ParseError::UnexpectedEnd), "input {cut}");
        // An extra token after a complete tree.
        let extra = format!("{data},#");
        assert_eq!(
            deserialize(&extra),
            Err(ParseError::TrailingData { index: tokens.len() }),
            "input {extra}"
        );
        // Replace one token with garbage.
        let i = rng.below(tokens.len());
        let mut bad = tokens.clone();
        bad[i] = "?";
        let bad = bad.join(",");
        assert_eq!(deserialize(&bad), Err(ParseError::BadToken { index: i }), "input {bad}");
    }
}

#[test]
fn deep_chains() {
    big_stack(|| {
        let vals: Vec<i32> = (0..2000).map(|i| i % 2001 - 1000).collect();
        round_trip(chain(&vals, |_| true));
        round_trip(chain(&vals, |_| false));
        round_trip(chain(&vals, |i| i % 2 == 0));
    });
}

#[test]
fn large_complete_tree() {
    round_trip(complete((0..10_000).map(|i| i % 2001 - 1000)));
}

#[test]
fn large_random_tree() {
    big_stack(|| {
        let mut rng = Rng(6060);
        round_trip(rand_tree(&mut rng, 10_000, &mut |r| r.range(-1000, 1000)));
    });
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
