## Description

Build `DList<T>`, a doubly linked list whose nodes live in an **arena**: a
`Vec` of slots, linked by index instead of by pointer. Every insertion returns
a `Handle` to the new element; later, that handle reaches the element in
`O(1)` — to read it, change it, insert after it, or remove it from the middle
of the list.

Implement:

- `new`, `len`, `is_empty`, and `Default`
- `push_front`, `push_back` → `Handle`; `pop_front`, `pop_back` → `Option<T>`
- `insert_after(handle, value)` → the new element's `Handle`
- `remove(handle)` → `Option<T>`, unlinking the element from anywhere
- `get(handle)`, `get_mut(handle)`
- `iter`, front to back, returning your own `Iter<'_, T>` — which is also a
  `DoubleEndedIterator`, so `iter().rev()` walks back to front along the
  `prev` links (that's what they're for)
- `IntoIterator for &DList<T>`, so `for x in &list` works

Removed slots go on a **free list** and are reused by later insertions, so a
list that stays short never grows its arena, however many elements pass
through it.

### Why an arena?

Each node in a doubly linked list is pointed to by *two* others, its
predecessor and its successor. Rust's ownership model wants a single owner,
so the usual tools fit badly:

- **`Box`** can't express it at all: a node can only be owned once.
- **`Rc<RefCell<Node>>` forward, `Weak` backward** works, but every step
  through the list is a reference-count update plus a runtime borrow check,
  every `get` returns a `Ref` guard instead of a plain `&T`, and a mistake
  shows up as a panic (`BorrowMutError`) or a leaked cycle, not a compile
  error.
- **Raw pointers** are what `std::collections::LinkedList` uses — and they
  need `unsafe`, with all the aliasing rules on you.

An arena sidesteps the problem: the `Vec` owns every node, and links are plain
`usize`s that the borrow checker doesn't care about. `get` hands out an
ordinary `&T` tied to `&self`. Handles are `Copy`, cheap, and can't dangle in
the memory-safety sense — the worst a stale handle can do is miss, or (once
its slot is reused) refer to a newer element, exactly like a stale index into
a `Vec`. Production arenas fix that last bit with a generation counter per
slot; here, a handle is just the slot index.

## Restrictions

- No `std::collections::LinkedList` or `VecDeque`; no `Rc`, `RefCell`,
  `Box`-linked nodes or raw pointers. One `Vec` of slots is the arena.
- Safe Rust only: no `unsafe`.

## Signature

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(usize);

pub struct DList<T> { /* your fields */ }

impl<T> DList<T> {
    pub fn new() -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn push_front(&mut self, value: T) -> Handle;
    pub fn push_back(&mut self, value: T) -> Handle;
    pub fn pop_front(&mut self) -> Option<T>;
    pub fn pop_back(&mut self) -> Option<T>;
    pub fn insert_after(&mut self, at: Handle, value: T) -> Result<Handle, T>;
    pub fn remove(&mut self, at: Handle) -> Option<T>;
    pub fn get(&self, at: Handle) -> Option<&T>;
    pub fn get_mut(&mut self, at: Handle) -> Option<&mut T>;
    pub fn iter(&self) -> Iter<'_, T>;
}

pub struct Iter<'a, T> { /* your fields */ }
impl<'a, T> Iterator for Iter<'a, T> { type Item = &'a T; /* … */ }
impl<T> DoubleEndedIterator for Iter<'_, T> { /* … */ }
impl<'a, T> IntoIterator for &'a DList<T> { type Item = &'a T; type IntoIter = Iter<'a, T>; /* … */ }
```

A handle whose element has been removed (and whose slot hasn't been reused)
is *invalid*: `get`, `get_mut` and `remove` return `None`, and
`insert_after` hands the value back as `Err(value)`. The same goes for a
handle that was never issued (an index past the arena). `T` needs no bounds.

## Examples

```rust
let mut list = DList::new();
let b = list.push_back("b");
let d = list.push_back("d");
let a = list.push_front("a");                 // a b d
let c = list.insert_after(b, "c").unwrap();   // a b c d
assert_eq!(list.iter().copied().collect::<Vec<_>>(), ["a", "b", "c", "d"]);
assert_eq!(list.iter().rev().copied().collect::<Vec<_>>(), ["d", "c", "b", "a"]);

assert_eq!(list.remove(b), Some("b"));        // a c d
assert_eq!(list.get(b), None);                // b's handle is now invalid
assert_eq!(list.insert_after(b, "x"), Err("x"));
assert_eq!(list.get(c), Some(&"c"));          // other handles are unaffected

*list.get_mut(a).unwrap() = "A";
assert_eq!(list.pop_back(), Some("d"));
assert_eq!(list.get(d), None);
assert_eq!(list.pop_front(), Some("A"));
assert_eq!(list.len(), 1);
```

## Constraints

- Up to `2 * 10^5` elements and `10^6` operations; every operation is `O(1)`
  (amortized for the ones that may grow the arena).
- After any sequence of operations, the arena holds at most as many slots as
  the list's largest length so far.
- Every element is dropped exactly once.

## Rust notes

- An `enum` for the slot (`Used { .. }` / `Free { .. }`) makes "is this handle
  valid?" a `match`, and `let … else` keeps the early returns short.
- `mem::replace(&mut self.slots[i], Slot::Free { .. })` takes ownership of a
  node's value while freeing its slot in one move.
- Patching a neighbour needs `&mut self.slots[j]` while you hold no other
  borrow of `self.slots`: copy the `usize` links out first, then mutate.
- `Iter<'a, T>` can hold `&'a [Slot<T>]`, the next index to visit from each
  end, and how many elements are left — the count is what tells the two ends
  they have met, since the links alone would carry each end straight past the
  other.
