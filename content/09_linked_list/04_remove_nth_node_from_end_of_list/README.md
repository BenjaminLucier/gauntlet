## Description

Given a singly linked list, remove the `n`-th node counting from the **end**
(`n = 1` is the last node) and return the resulting list.

If `n` is `0` or larger than the length of the list, there is no such node:
return the list unchanged.

## Signature

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn remove_nth_from_end(head: List, n: usize) -> List
```

The list is taken by value because the head itself may be removed — the
function has to be able to hand back a different first node. `n` is a position,
so it is a `usize`; an out-of-range position is not an error worth a panic, it
simply removes nothing.

## Examples

`list(&[...])` builds a `List` from a slice and `to_vec(list)` turns one back
into a `Vec<i32>` (both are helpers in `tests.rs`).

```rust
// The 2nd node from the end is 4.
assert_eq!(to_vec(remove_nth_from_end(list(&[1, 2, 3, 4, 5]), 2)), [1, 2, 3, 5]);
assert_eq!(remove_nth_from_end(list(&[1]), 1), None);
assert_eq!(to_vec(remove_nth_from_end(list(&[1, 2]), 1)), [1]);
assert_eq!(to_vec(remove_nth_from_end(list(&[1, 2]), 2)), [2]); // removes the head
assert_eq!(to_vec(remove_nth_from_end(list(&[1, 2]), 3)), [1, 2]); // out of range
```

## Constraints

- The list has between `0` and `5000` nodes.
- `0 <= val <= 100`
- `0 <= n <= 10^4`

## Rust notes

- The **cursor** pattern removes the "is it the head?" special case:
  walk `let mut slot: &mut List = &mut head;` forward with
  `slot = &mut slot.as_mut()?.next;` until it holds the doomed node, then
  `*slot = slot.take()?.next;` (or use `Option::and_then`).
- Two cursors into one `Box` list, one of them `&mut`, don't coexist. Counting
  the length with a shared walk (`node.next.as_deref()`, or
  `std::iter::successors`) and then making a second, mutable walk is still
  `O(len)` time and `O(1)` space.
- `len - n` on `usize` panics (overflow checks are on) when `n > len`: rule
  that case out first, or use `checked_sub`, which returns an `Option`.
