## Description

Decide whether a binary tree is a valid **binary search tree**:

- every value in a node's left subtree is **strictly less** than the node's value,
- every value in a node's right subtree is **strictly greater** than the node's value,
- and both subtrees are themselves valid BSTs.

The empty tree is a valid BST.

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

pub fn is_valid_bst(root: Option<&TreeNode>) -> bool
```

Validation only reads the tree, so it borrows it as `Option<&TreeNode>`
(from a `Tree`, pass `tree.as_deref()`).

## Examples

```rust
assert!(is_valid_bst(t("[2,1,3]").as_deref()));
// The root is 5 but its right child is 4.
assert!(!is_valid_bst(t("[5,1,4,null,null,3,6]").as_deref()));
// Every parent/child pair is in order, but 3 sits in the root's right
// subtree while being smaller than 5.
assert!(!is_valid_bst(t("[5,4,6,null,null,3,7]").as_deref()));
assert!(is_valid_bst(None));
```

## Constraints

- The number of nodes is in `[0, 10^4]`
- `-2^31 <= val <= 2^31 - 1` (the full `i32` range)

## Rust notes

- Values span all of `i32`, so `i32::MIN` / `i32::MAX` can't mean "no bound".
  `Option<i32>` says exactly that, and `Option::is_none_or` checks a bound
  that may be absent in one call. (Widening to `i64` also works.)
- `&&` short-circuits, so a recursive check can stop at the first violation.
- An inorder walk with an explicit `Vec<&TreeNode>` stack and a
  `prev: Option<i32>` is the recursion-free alternative.
