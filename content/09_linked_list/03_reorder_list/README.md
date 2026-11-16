## Description

You are given a singly linked list

```text
L0 → L1 → … → Ln-1 → Ln
```

Rearrange its nodes into

```text
L0 → Ln → L1 → Ln-1 → L2 → Ln-2 → …
```

and return the result. Move the nodes themselves — don't rewrite the values
stored in them, and don't allocate new nodes.

## Signature

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn reorder_list(head: List) -> List
```

Reordering restructures every link, so the function takes ownership of the list
and returns the reordered one — the same nodes, in a new order. (Callers who
hold a `&mut List` can write `*slot = reorder_list(slot.take());`.)

## Examples

`list(&[...])` builds a `List` from a slice and `to_vec(list)` turns one back
into a `Vec<i32>` (both are helpers in `tests.rs`).

```rust
assert_eq!(to_vec(reorder_list(list(&[1, 2, 3, 4]))), [1, 4, 2, 3]);
assert_eq!(to_vec(reorder_list(list(&[1, 2, 3, 4, 5]))), [1, 5, 2, 4, 3]);
assert_eq!(to_vec(reorder_list(list(&[1, 2]))), [1, 2]); // nothing to move
assert_eq!(reorder_list(None), None);
```

## Constraints

- The list has between `0` and `5000` nodes.
- `1 <= val <= 1000`

## Rust notes

- With `Box` links you can't hold a slow and a fast cursor into the same list
  at once if either is `&mut`. Counting the length first with a shared-borrow
  walk (`node.next.as_deref()`) and then walking a single `&mut` cursor is the
  straightforward way to find the middle.
- The **cursor** pattern: `let mut cur: &mut List = &mut head;` then
  `cur = &mut cur.as_mut()?.next;` advances one node. At the split point,
  `cur.take()` detaches everything after it.
- `Option::take` and `std::mem::replace` / `std::mem::swap` let you move a
  node's `next` out, splice something in, and put the rest back, without ever
  having two owners.
- `as_mut()` turns `&mut Option<Box<T>>` into `Option<&mut Box<T>>`;
  `as_deref()` / `as_deref_mut()` go one step further to `Option<&T>` /
  `Option<&mut T>`.
