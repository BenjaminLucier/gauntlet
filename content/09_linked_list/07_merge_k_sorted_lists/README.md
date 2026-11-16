## Description

You are given `k` singly linked lists, each sorted in non-decreasing order.
Merge all of them into a single sorted list and return it. The result is made
of the input nodes — relink them, don't allocate new ones.

## Signature

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn merge_k_lists(lists: Vec<List>) -> List
```

The function consumes the `Vec`: every node of every list is moved into the
result. Owning the `Vec` also gives you a place to keep each list's remaining
nodes while you work (`lists[i]`). Any of the lists may be empty (`None`), and
so may the `Vec` itself.

## Examples

`list(&[...])` builds a `List` from a slice and `to_vec(list)` turns one back
into a `Vec<i32>` (both are helpers in `tests.rs`).

```rust
let lists = vec![list(&[1, 4, 5]), list(&[1, 3, 4]), list(&[2, 6])];
assert_eq!(to_vec(merge_k_lists(lists)), [1, 1, 2, 3, 4, 4, 5, 6]);
assert_eq!(merge_k_lists(vec![]), None);     // no lists at all
assert_eq!(merge_k_lists(vec![None]), None); // one empty list
```

## Constraints

- `0 <= k <= 10^4`
- Each list has between `0` and `500` nodes; at most `10^4` nodes in total.
- `-10^4 <= val <= 10^4`
- Every list is sorted in non-decreasing order.

## Rust notes

- `std::collections::BinaryHeap` is a max-heap; wrap entries in
  `std::cmp::Reverse` for a min-heap.
- Push `(value, list_index)` rather than the nodes: tuples are `Ord` for free,
  `ListNode` isn't, and the nodes can stay in `lists[list_index]` until you
  pop them.
- Pop a node off list `i` with `Option::take`:
  `let mut node = lists[i].take()?; lists[i] = node.next.take();`
- Append through a `&mut List` cursor:
  `tail = &mut tail.insert(node).next;`.
