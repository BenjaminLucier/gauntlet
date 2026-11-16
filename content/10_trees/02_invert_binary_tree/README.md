## Description

Given a binary tree, mirror it: every node's left and right children swap
places, all the way down. Return the mirrored tree.

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

pub fn invert_tree(root: Tree) -> Tree
```

Every node has exactly one owner — its parent — so a tree is plain owned
`Box`es. The function takes the tree by value and hands it back: it can
rearrange the existing nodes instead of allocating new ones, and the caller
can't keep using the un-mirrored version by accident.

## Examples

```rust
assert_eq!(invert_tree(t("[4,2,7,1,3,6,9]")), t("[4,7,2,9,6,3,1]"));
assert_eq!(invert_tree(t("[2,1,3]")), t("[2,3,1]"));
assert_eq!(invert_tree(None), None); // the empty tree mirrors to itself
```

## Constraints

- The number of nodes is in `[0, 100]` (tests also use much bigger trees)
- `-100 <= val <= 100`

## Rust notes

- `root.as_deref_mut()` turns `&mut Option<Box<TreeNode>>` into
  `Option<&mut TreeNode>` — the usual way to get at a node without moving it
  out of its `Option`.
- `std::mem::swap(&mut node.left, &mut node.right)` swaps two fields of the
  same struct; the borrow checker allows it because the borrows are disjoint.
- `Option::take` moves a child out and leaves `None` behind, which is how you
  pass a subtree by value to a recursive call and put the result back.
- `Box`, not `Rc<RefCell<…>>`: nothing here shares nodes, so single ownership
  is both simpler and checked at compile time.
