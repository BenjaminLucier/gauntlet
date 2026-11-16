## Description

Given a binary tree, return the length of its **diameter**: the longest path
between any two nodes, measured in **edges**. The path does not have to pass
through the root. A single node, and the empty tree, have diameter `0`.

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

pub fn diameter_of_binary_tree(root: Option<&TreeNode>) -> usize
```

The tree is only read, so it is borrowed as `Option<&TreeNode>` (pass
`tree.as_deref()`). A path length is a count of edges, so it is a `usize`.

## Examples

```rust
// 4 -> 2 -> 1 -> 3 (or 5 -> 2 -> 1 -> 3) has 3 edges.
assert_eq!(diameter_of_binary_tree(t("[1,2,3,4,5]").as_deref()), 3);
assert_eq!(diameter_of_binary_tree(t("[1,2]").as_deref()), 1);
assert_eq!(diameter_of_binary_tree(t("[1]").as_deref()), 0);
assert_eq!(diameter_of_binary_tree(None), 0);
```

## Constraints

- The number of nodes is in `[0, 10^4]`
- `-100 <= val <= 100`

## Rust notes

- A nested helper `fn height(node: Option<&TreeNode>, best: &mut usize) -> usize`
  can return one thing and update another through a `&mut` — no globals, no
  `RefCell`.
- Alternatively return a tuple `(height, best_inside)` from the helper and
  combine children's tuples; both are idiomatic.
- With `usize`, `left + right` can't go negative and won't overflow for any
  tree that fits in memory — but don't compute "height in edges" as
  `nodes - 1` on an empty subtree.
- `Box`, not `Rc<RefCell<…>>`: the tree has one owner, and a read-only walk
  only needs `&TreeNode`.
