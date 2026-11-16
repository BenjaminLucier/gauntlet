## Description

Given a binary search tree and two values `p` and `q`, return the node that
is their **lowest common ancestor**: the deepest node that has both `p` and
`q` in its subtree. A node counts as a descendant of itself, so if `p` is an
ancestor of `q` the answer is the node holding `p` (and if `p == q`, it is
that node).

If `p` or `q` does not occur in the tree, there is no such node: return `None`.

In a BST every value in a node's left subtree is smaller than the node's
value, and every value in its right subtree is larger.

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

pub fn lowest_common_ancestor(root: Option<&TreeNode>, p: i32, q: i32) -> Option<&TreeNode>
```

BST values are unique, so a value identifies a node: `p` and `q` are passed as
values. The answer is a *borrow of the node inside the tree* — its lifetime is
tied to `root` by elision — so the caller can inspect the whole ancestor
subtree without copying anything. `Option` covers values that aren't there.

## Examples

```rust
let tr = t("[6,2,8,0,4,7,9,null,null,3,5]");
let lca = |p, q| lowest_common_ancestor(tr.as_deref(), p, q).map(|n| n.val);
assert_eq!(lca(2, 8), Some(6));
assert_eq!(lca(2, 4), Some(2)); // 2 is an ancestor of 4
assert_eq!(lca(9, 7), Some(8)); // order of p and q doesn't matter
assert_eq!(lca(3, 1), None);    // 1 is not in the tree
assert_eq!(lowest_common_ancestor(t("[2,1]").as_deref(), 2, 1).map(|n| n.val), Some(2));
```

## Constraints

- The number of nodes is in `[0, 10^5]`
- `-10^9 <= val, p, q <= 10^9`, all node values unique
- `p` may be larger than, smaller than, or equal to `q`

## Rust notes

- Returning `Option<&TreeNode>` from a function taking `Option<&TreeNode>`
  needs no lifetime annotations: elision ties the output to the only input
  reference.
- `Option<&TreeNode>` is `Copy`, so `let mut cur = root;` plus
  `cur = node.left.as_deref();` walks the tree with no recursion and no clones.
- `i32::cmp` returning `Ordering` reads well in a `match` for the
  three-way BST step.
- `bool::then_some(node)` turns "found both?" into the `Option` result.
- Avoid tricks like `(node - p) * (node - q) > 0`: with values up to `10^9`
  the product overflows `i32` (and panics in debug builds).
