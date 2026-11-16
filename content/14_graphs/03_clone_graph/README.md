## Description

You are given a reference to one node of a **connected, undirected** graph.
Return a **deep copy** of the whole graph: a brand-new node for every original
node, wired up with the same neighbour lists (in the same order), and return
the copy of the node you were given.

An undirected edge `a — b` appears as `b` in `a`'s neighbour list **and** `a` in
`b`'s neighbour list. There are no self-loops and no repeated edges.

No node of the copy may be an original node, and each original node must be
copied **exactly once** — if two original nodes both point at node `3`, their
copies must both point at the *same* copy of `3`. The original graph must be
left as it was.

## Signature

```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    pub val: i32,
    pub neighbors: Vec<NodeRef>,
}

pub type NodeRef = Rc<RefCell<Node>>;

impl Node {
    pub fn new(val: i32) -> NodeRef { /* given */ }
}

pub fn clone_graph(node: &NodeRef) -> NodeRef
```

This problem is *about* shared, cyclic ownership, so it keeps the shape a
Rust developer would reach for when nodes genuinely have several owners:

- **`Rc`** because a node is owned by every neighbour that points at it — no
  single parent owns it, so `Box` can't express the graph.
- **`RefCell`** because a cycle can't be built from immutable pieces: you
  create a node first and push neighbours into it later, through
  `borrow_mut()`, while other `Rc`s to it already exist.
- **Node identity is the pointer**, not the value: `Rc::ptr_eq` tells two
  handles to the same node apart from two equal-looking nodes, and
  `Rc::as_ptr` gives a hashable key.

Every edge is a strong `Rc`, so the graph's cycles keep it alive forever —
dropping the last outside handle leaks it. `Weak` back-edges would fix that in
a tree, but an undirected graph has no "parent" direction to make strong; a
leak-free design needs a separate owner of all the nodes (an arena, a
`Vec<NodeRef>`). Here the caller owns that responsibility: the tests break the
cycles with `neighbors.clear()` when they are done. The function borrows the
start node (`&NodeRef`) since it only reads the original graph. An empty graph
has no node to pass; `Option<&NodeRef>::map(clone_graph)` handles it.

## Examples

```rust
// A 4-cycle 1 - 2 - 3 - 4 - 1.
let nodes: Vec<NodeRef> = (1..=4).map(Node::new).collect();
for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
    nodes[a].borrow_mut().neighbors.push(Rc::clone(&nodes[b]));
    nodes[b].borrow_mut().neighbors.push(Rc::clone(&nodes[a]));
}
let copy = clone_graph(&nodes[0]);
assert_eq!(copy.borrow().val, 1);
assert!(!Rc::ptr_eq(&copy, &nodes[0]));          // a new node...
let two = Rc::clone(&copy.borrow().neighbors[0]);
assert_eq!(two.borrow().val, 2);                  // ...with the same wiring
assert!(Rc::ptr_eq(&two.borrow().neighbors[0], &copy)); // and the cycle closes on the copy

let lonely = Node::new(7);
assert!(clone_graph(&lonely).borrow().neighbors.is_empty());
```

## Constraints

- `1 <= number of nodes <= 100` (an empty graph is `None` on the caller's side)
- values are unique, `1 <= val <= 100`
- no repeated edges, no self-loops; the graph is connected

## Rust notes

- A `HashMap<*const RefCell<Node>, NodeRef>` keyed by `Rc::as_ptr(&rc)` maps
  originals to copies by identity. The raw pointer is only used as a key,
  never dereferenced, so no `unsafe` is involved.
- `Rc::clone(&rc)` copies the handle, not the node — that's what you want when
  wiring a copy's neighbours to copies that already exist.
- Don't hold a `borrow()` / `borrow_mut()` across code that borrows the same
  `RefCell` mutably: that's a runtime panic ("already borrowed"), not a
  compile error. Scope borrows tightly, or collect what you need first.
- Use a `VecDeque` (BFS) or `Vec` (DFS) worklist rather than recursion.
