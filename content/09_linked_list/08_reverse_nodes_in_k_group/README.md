## Description

Given a singly linked list and a group size `k`, reverse the nodes of the list
`k` at a time and return the result.

Split the list into consecutive groups of `k` nodes from the front and reverse
the order of the nodes within each full group. If the number of nodes isn't a
multiple of `k`, the final group of fewer than `k` nodes keeps its original
order. With `k = 0` or `k = 1` there is nothing to reverse: the list comes back
unchanged.

Only the links may change: don't rewrite the values stored in the nodes or
allocate new ones.

## Signature

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn reverse_k_group(head: List, k: usize) -> List
```

The list is restructured, so it is taken and returned by value. `k` is a group
size, so it is a `usize`; `0` is accepted (as a no-op, like `1`) rather than
being a panic waiting to happen.

## Examples

`list(&[...])` builds a `List` from a slice and `to_vec(list)` turns one back
into a `Vec<i32>` (both are helpers in `tests.rs`).

```rust
assert_eq!(to_vec(reverse_k_group(list(&[1, 2, 3, 4, 5]), 2)), [2, 1, 4, 3, 5]);
assert_eq!(to_vec(reverse_k_group(list(&[1, 2, 3, 4, 5]), 3)), [3, 2, 1, 4, 5]);
assert_eq!(to_vec(reverse_k_group(list(&[1, 2, 3]), 4)), [1, 2, 3]); // no full group
assert_eq!(reverse_k_group(None, 2), None);
```

## Constraints

- The list has between `0` and `5000` nodes.
- `0 <= k <= 10^4`
- `0 <= val <= 1000`

## Rust notes

- Keep a **cursor** `tail: &mut List` at the end of the finished part of the
  result. Write a reversed group into it, then walk it forward `k` nodes with
  `tail = &mut tail.as_mut()?.next;` (or an `expect` stating why the node
  exists).
- Reversing a group is the reverse-a-list loop from problem 35, stopped after
  `k` pops: `Option::take` / `std::mem::replace` move each node from the rest
  of the list onto the front of the group.
- Counting the length first with a shared walk (`node.next.as_deref()`, or
  `std::iter::successors(head.as_deref(), |n| n.next.as_deref()).count()`)
  tells you how many full groups there are, so you never have to undo a
  partial reversal.
- `len / k` panics when `k == 0`: handle that case before dividing.
