## Description

Given two binary trees `root` and `sub`, return `true` if some node of `root`
is the root of a subtree **identical** to `sub` (same shape and values), and
`false` otherwise.

A subtree of a node consists of that node and **all** of its descendants, so
`root` counts as a subtree of itself. The empty tree is a subtree of every
tree (every leaf has empty children), but a non-empty `sub` is never a subtree
of the empty tree.

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

pub fn is_subtree(root: Option<&TreeNode>, sub: Option<&TreeNode>) -> bool
```

Both trees are only read, so both are borrowed (pass `tree.as_deref()`).
Taking `Option` for `sub` makes the empty case explicit instead of forbidden.

## Examples

```rust
assert!(is_subtree(t("[3,4,5,1,2]").as_deref(), t("[4,1,2]").as_deref()));
// The node 4 in root has the extra descendant 0.
assert!(!is_subtree(
    t("[3,4,5,1,2,null,null,null,null,0]").as_deref(),
    t("[4,1,2]").as_deref(),
));
assert!(is_subtree(t("[1]").as_deref(), None)); // empty sub
assert!(!is_subtree(None, t("[1]").as_deref()));
```

## Constraints

- The number of nodes in `root` is in `[0, 2000]`
- The number of nodes in `sub` is in `[0, 1000]`
- `-10^4 <= val <= 10^4`

## Rust notes

- `TreeNode` derives `PartialEq`, so `node == sub` on two `&TreeNode`s is a
  full structural comparison (same shape, same values), for free. Know what the
  derive does before you rely on it.
- `let Some(sub) = sub else { return … };` dispatches the empty case up front
  and leaves you with a plain `&TreeNode`.
- `Vec::extend` accepts an `Option` (it's an iterator of zero or one items),
  so `stack.extend(node.left.as_deref())` pushes a child only if it exists.
- Recursing over `root` *and* inside the comparison can nest deeply on
  chain-shaped trees; an explicit stack for the outer walk keeps one of the
  two off the call stack.
