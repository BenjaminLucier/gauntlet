## Description

You are given two slices, `preorder` and `inorder`, which should hold the
preorder (node, left, right) and inorder (left, node, right) traversals of
the same binary tree with **distinct** values. Rebuild the tree.

The input isn't trusted: if the slices have different lengths, repeat a
value, or don't describe any single tree, return the matching error instead.
Two empty slices describe the empty tree.

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
    /// The traversals have different lengths.
    LengthMismatch { preorder: usize, inorder: usize },
    /// This value occurs more than once in one of the traversals.
    DuplicateValue(i32),
    /// No binary tree has both traversals.
    Inconsistent,
}

pub fn build_tree(preorder: &[i32], inorder: &[i32]) -> Result<Tree, BuildError>
```

The traversals are only read, so they're borrowed slices. The function
creates a new tree, so it returns an owned `Tree`. There are distinct ways
for the input to be wrong, so failure is a `Result` with a small error enum.
Checks happen in the order listed: lengths, then duplicates, then whether a
tree exists.

## Examples

```rust
assert_eq!(build_tree(&[3, 9, 20, 15, 7], &[9, 3, 15, 20, 7]), Ok(t("[3,9,20,null,null,15,7]")));
assert_eq!(build_tree(&[-1], &[-1]), Ok(t("[-1]")));
assert_eq!(build_tree(&[], &[]), Ok(None));
assert_eq!(
    build_tree(&[1, 2], &[1]),
    Err(BuildError::LengthMismatch { preorder: 2, inorder: 1 }),
);
assert_eq!(build_tree(&[1, 1], &[1, 1]), Err(BuildError::DuplicateValue(1)));
// Root 1 puts 3 on the left and 2 on the right, so preorder would be 1, 3, 2.
assert_eq!(build_tree(&[1, 2, 3], &[3, 1, 2]), Err(BuildError::Inconsistent));
```

## Constraints

- `0 <= preorder.len(), inorder.len() <= 3000`
- `-3000 <= preorder[i], inorder[i] <= 3000`

## Rust notes

- A small struct holding `preorder`, the shared cursor and the
  value-to-index `HashMap` keeps the recursive helper's argument list short;
  give it a method taking an inorder `Range<usize>`.
- The `?` operator propagates an `Err` straight out of nested recursion, and
  `map.get(&v).ok_or(BuildError::Inconsistent)?` turns a missing value into
  an error.
- `HashMap::insert` returns the previous value, so `is_some()` spots a
  duplicate while you build the index.
- `Range::contains` and `Range::is_empty` read better than hand-written
  comparisons.
