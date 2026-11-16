## Description

You are given a singly linked list. Determine whether it contains a **cycle**:
some node that is reached again by repeatedly following `next` links. Return
`true` if it does, `false` if following `next` eventually reaches the end.

The point of the problem: solve it with `O(1)` extra memory.

## Signature

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub next: Option<usize>, // index into the arena; None = end of list
}

pub fn has_cycle(nodes: &[Node], head: Option<usize>) -> bool
```

A cyclic list can't be built from `Option<Box<ListNode>>`: a `Box` has exactly
one owner, and in a cycle some node would need two (its predecessor in the
loop, and the node before the loop). `Rc` could express it, but an `Rc` cycle
is never freed. So here the nodes live in an **arena** — a slice — and `next`
is the *index* of the next node. Indices are `Copy`, comparing two of them is
comparing node identity, and the arena owns everything, so nothing leaks.

`head` is the index of the first node (`None` for an empty list). The arena may
be in any order and may contain nodes that are **not reachable** from `head` —
only the list starting at `head` matters. Every `next` index is in bounds.

## Examples

`build(&vals, pos)` lays the values out at indices `0..n` with node `i` linking
to `i + 1`, then links the tail back to index `pos` (`None` = no cycle); it is a
helper in `tests.rs`.

```rust
// 3 -> 2 -> 0 -> -4 -> (back to 2)
let (nodes, head) = build(&[3, 2, 0, -4], Some(1));
assert!(has_cycle(&nodes, head));

let (nodes, head) = build(&[1, 2], Some(0)); // tail links back to the head
assert!(has_cycle(&nodes, head));

let (nodes, head) = build(&[1], None);
assert!(!has_cycle(&nodes, head));

assert!(!has_cycle(&[], None)); // the empty list
```

## Constraints

- The arena has between `0` and `10^4` nodes.
- `-10^5 <= val <= 10^5`
- Every `next` is `None` or a valid index into `nodes`.

## Rust notes

- `Option<usize>` is the "nullable pointer" here; `Option::and_then` follows a
  link that may not exist: `cur.and_then(|i| nodes[i].next)`.
- A small closure such as `let step = |i: Option<usize>| …;` keeps the loop
  readable — it borrows `nodes` immutably, which is all you need.
- Compare indices (`Option<usize>` implements `PartialEq`), not values: values
  repeat.
- Arenas with index links are the standard safe-Rust answer for graphs,
  doubly linked lists and cycles; you'll meet them again in the LRU cache.
