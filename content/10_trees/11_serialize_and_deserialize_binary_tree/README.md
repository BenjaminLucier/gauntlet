## Description

Write a pair of functions that turn a binary tree into a `String`
(`serialize`) and turn that string back into an identical tree
(`deserialize`), so that `deserialize(&serialize(t))` rebuilds a tree with the
same shape and values.

So that both directions can be tested — including what happens with bad
input — the format is fixed:

- Walk the tree in **level order** (breadth-first, left to right). Emit each
  node's value in decimal, and emit `#` for every **missing** child of a node
  you've emitted (so every value is followed, eventually, by exactly two
  tokens for its children).
- Separate tokens with a single `,` — no spaces, no brackets.
- The empty tree is `#`.

For example, the tree with root `1`, children `2` and `3`, and `3` having
children `4` and `5` is `1,2,3,#,#,4,5,#,#,#,#`.

`deserialize` must reject anything that isn't a complete encoding, saying
why:

- a token that is neither `#` nor something `str::parse::<i32>` accepts →
  `ParseError::BadToken { index }` (tokens are counted from 0);
- input that ends while some node's children haven't been given →
  `ParseError::UnexpectedEnd`;
- tokens left over once the tree is complete → `ParseError::TrailingData { index }`
  pointing at the first extra token.

Tokens are read in order, so the first problem encountered is the one
reported. Trees in the examples are built with `t("[…]")`, a helper from
`tests.rs` that reads level order with `null` for missing children.

## Signature

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    BadToken { index: usize },
    UnexpectedEnd,
    TrailingData { index: usize },
}
// (template.rs also implements Display and std::error::Error for it)

pub fn serialize(root: Option<&TreeNode>) -> String
pub fn deserialize(data: &str) -> Result<Tree, ParseError>
```

The codec has no state, so it is two free functions rather than a struct
with methods. `serialize` only reads the tree, so it borrows it;
`deserialize` only reads the text, so it takes `&str`. Decoding can fail in
distinct ways, so it returns a `Result` with a small error enum instead of
panicking or returning an empty tree. `ParseError` implements
`std::error::Error`, so callers can use `?` with `Box<dyn Error>`.

## Examples

```rust
assert_eq!(serialize(t("[1,2,3,null,null,4,5]").as_deref()), "1,2,3,#,#,4,5,#,#,#,#");
assert_eq!(deserialize("1,2,3,#,#,4,5,#,#,#,#"), Ok(t("[1,2,3,null,null,4,5]")));
assert_eq!(serialize(None), "#");
assert_eq!(deserialize("#"), Ok(None));

assert_eq!(deserialize("1,x,#"), Err(ParseError::BadToken { index: 1 }));
assert_eq!(deserialize("1,2,#"), Err(ParseError::UnexpectedEnd)); // 2 has no children listed
assert_eq!(deserialize("1,#,#,7"), Err(ParseError::TrailingData { index: 3 }));
assert_eq!(deserialize(""), Err(ParseError::BadToken { index: 0 }));
```

## Constraints

- The number of nodes is in `[0, 10^4]`
- `val` is any `i32` (most tests use `-1000..=1000`)

## Rust notes

- `str::split(',')` yields tokens lazily; `.enumerate()` gives you the index
  for error reports, and a hand-pulled iterator (`next()`) lets the decoder
  consume exactly two tokens per node.
- Building a `Box` tree breadth-first needs a queue of `&mut TreeNode` that
  borrow from the root you're about to return. `as_deref_mut()` gets them, and
  the borrow checker is happy as long as the queue is no longer used when you
  return the root.
- `?` works inside closures that return `Result` too, and `Option::ok_or`
  turns "no more tokens" into an error.
- `std::fmt::Write` lets `write!` append to a `String` without building a
  temporary `String` per value.
- A recursive preorder decoder is shorter but recurses as deep as the tree;
  a BFS needs no recursion at all.
