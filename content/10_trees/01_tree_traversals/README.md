## Description

Walk a binary tree in the four classic orders:

- **preorder** — node, then left subtree, then right subtree
- **inorder** — left subtree, node, right subtree
- **postorder** — left subtree, right subtree, node
- **level order** — breadth first: one `Vec` per depth, from the root
  downwards, each listing that level's values from left to right

Write each depth-first order twice: once recursively, and once iteratively
with an explicit stack (`preorder_iter`, `inorder_iter`, `postorder_iter`).
Recursion is the natural way to write a tree walk, but its depth is the
tree's height, and a degenerate tree (every node has one child) can be
100 000 levels deep — far more than the call stack holds. The `_iter`
versions must survive that: the deep tests run them on a thread with a 1 MiB
stack. (A stack overflow can't be caught, so a recursive attempt there
aborts the whole test run with "has overflowed its stack" rather than
failing one test.)

Trees in the examples are built with `t("[…]")`, a helper from `tests.rs` that
reads a tree in level order, where `null` marks a missing child.

## Restrictions

- The `_iter` functions and `level_order` must not recurse (directly, or by
  calling the recursive versions). Use a `Vec` as the stack and a `VecDeque`
  as the queue.

## Signature

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn preorder(root: Option<&TreeNode>) -> Vec<i32>
pub fn inorder(root: Option<&TreeNode>) -> Vec<i32>
pub fn postorder(root: Option<&TreeNode>) -> Vec<i32>

pub fn preorder_iter(root: Option<&TreeNode>) -> Vec<i32>
pub fn inorder_iter(root: Option<&TreeNode>) -> Vec<i32>
pub fn postorder_iter(root: Option<&TreeNode>) -> Vec<i32>

pub fn level_order(root: Option<&TreeNode>) -> Vec<Vec<i32>>
```

Trees are owned `Box`es: each node has exactly one parent, so no `Rc` is
needed. A traversal only reads the tree, so it borrows it as
`Option<&TreeNode>` — callers holding a `Tree` pass `tree.as_deref()`, and
any subtree can be passed the same way.

## Examples

```rust
//        1
//       / \
//      2   3
//     / \   \
//    4   5   6
let tree = t("[1,2,3,4,5,null,6]");
let root = tree.as_deref();
assert_eq!(preorder(root), [1, 2, 4, 5, 3, 6]);
assert_eq!(inorder(root), [4, 2, 5, 1, 3, 6]);
assert_eq!(postorder(root), [4, 5, 2, 6, 3, 1]);
assert_eq!(inorder_iter(root), inorder(root)); // same answers, no recursion
assert_eq!(level_order(root), [vec![1], vec![2, 3], vec![4, 5, 6]]);

assert!(preorder(None).is_empty()); // empty tree: nothing to visit
assert!(level_order(None).is_empty()); // ...and no levels at all
```

## Constraints

- Up to `10^5` nodes, and the tree may be a single path of that length.
- `-10^9 <= val <= 10^9`

## Rust notes

- The explicit stack holds borrowed nodes, `Vec<&TreeNode>` — no cloning, no
  moving out of the tree. `node.left.as_deref()` turns a `&Tree` into an
  `Option<&TreeNode>`.
- An `Option` is an iterator of zero or one items, so
  `stack.extend(node.right.as_deref())` pushes the child only if it exists;
  the same works for `queue.extend(...)`.
- `while let Some(node) = stack.pop()` is the shape of most iterative walks.
  For inorder, `let mut cur = root;` plus `while let Some(n) = cur` walks down
  a left spine.
- A recursive helper that appends to a shared `&mut Vec<i32>` avoids
  allocating and concatenating a `Vec` per subtree.
- `Vec::reverse` is `O(n)` and in place.
