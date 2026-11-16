## Description

Imagine standing to the right of a binary tree. Return the values of the
nodes you can see, ordered from top to bottom — that is, the **rightmost
node of every level**.

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

pub fn right_side_view(root: Option<&TreeNode>) -> Vec<i32>
```

The tree is owned `Box`es (one parent per node, so no `Rc`), and this
function only reads it, so it borrows: `Option<&TreeNode>`, which callers get
from a `Tree` with `.as_deref()`.

## Examples

```rust
assert_eq!(right_side_view(t("[1,2,3,null,5,null,4]").as_deref()), [1, 3, 4]);
// The left subtree is deeper, so 4 and 5 show below the right subtree.
assert_eq!(right_side_view(t("[1,2,3,4,null,null,null,5]").as_deref()), [1, 3, 4, 5]);
assert_eq!(right_side_view(t("[1,null,3]").as_deref()), [1, 3]);
assert!(right_side_view(None).is_empty());
```

## Constraints

- The number of nodes is in `[0, 100]` (tests also use bigger trees)
- `-100 <= val <= 100`

## Rust notes

- `VecDeque::back()` peeks at the last node queued — at the start of a round,
  that's the rightmost node of the level.
- `queue.extend(node.left.as_deref())` pushes a child only if it exists;
  `Option` is an iterator of zero or one items.
- A recursive DFS needs `&mut Vec<i32>` for the output plus the current depth;
  `depth == out.len()` tells you a level is seen for the first time.
