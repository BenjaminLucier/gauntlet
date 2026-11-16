## Description

A **path** in a binary tree is a sequence of nodes where each adjacent pair
is joined by an edge, and no node appears more than once. A path needs at
least one node and does not have to pass through the root. Its **path sum**
is the sum of its nodes' values.

Given a binary tree, return the maximum path sum over all non-empty paths —
or `None` if the tree is empty, since then there is no path at all.

Trees in the examples are built with `t("[…]")`, a helper from `tests.rs` that
reads a tree in level order, where `null` marks a missing child.

## Signature

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn max_path_sum(root: Option<&TreeNode>) -> Option<i32>
```

The function only reads the tree, so it borrows it (`tree.as_deref()` turns a
`&Tree` into an `Option<&TreeNode>`). The empty tree has no non-empty path,
so the result is an `Option` rather than a made-up sentinel like `i32::MIN`
or `0`. With at most `3 * 10^4` nodes of magnitude `<= 1000`, every path sum
fits in an `i32`.

## Examples

```rust
// The path 2 -> 1 -> 3 sums to 6.
assert_eq!(max_path_sum(t("[1,2,3]").as_deref()), Some(6));
// The path 15 -> 20 -> 7 sums to 42; the root is not on it.
assert_eq!(max_path_sum(t("[-10,9,20,null,null,15,7]").as_deref()), Some(42));
// A path may be a single node, even a negative one.
assert_eq!(max_path_sum(t("[-3]").as_deref()), Some(-3));
assert_eq!(max_path_sum(None), None);
```

## Constraints

- The number of nodes is in `[0, 3 * 10^4]`
- `-1000 <= val <= 1000`

## Rust notes

- Instead of threading a `&mut i32` "global best" through the recursion, a
  helper can return a tuple — one value for the parent, one for the answer.
- `node.left.as_deref().map(f)` applies `f` to a child only when it exists;
  `map_or(0, …)` folds the missing case in.
- `i32::max` is a plain function, so it can be passed to `fold` or `reduce`
  directly.
- Recursion depth equals the tree height; on a path-shaped tree of `3 * 10^4`
  nodes that is deep. The tests run deep cases on a thread with a big stack,
  but an explicit post-order stack avoids the question entirely.
