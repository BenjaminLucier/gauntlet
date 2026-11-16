## Description

Design a stack that, in addition to the usual operations, can report its
current **minimum element in constant time**.

Implement `MinStack<T>`:

- `MinStack::new()` — creates an empty stack (also available via `Default`).
- `push(value)` — pushes `value` on top.
- `pop()` — removes the top element and returns it, or `None` if the stack is
  empty.
- `peek()` — returns a reference to the top element, or `None` if empty.
- `min()` — returns a reference to the smallest element currently in the stack,
  or `None` if empty.
- `len()` / `is_empty()` — the number of elements, and whether it is zero.

**Every** operation must run in `O(1)` time (amortized for `push`).

## Signature

```rust
pub struct MinStack<T> { /* your fields */ }

impl<T: Ord> MinStack<T> {
    pub fn new() -> Self
    pub fn push(&mut self, value: T)
    pub fn pop(&mut self) -> Option<T>
    pub fn peek(&self) -> Option<&T>
    pub fn min(&self) -> Option<&T>
    pub fn len(&self) -> usize
    pub fn is_empty(&self) -> bool
}

impl<T: Ord> Default for MinStack<T>
```

The stack is generic: all it needs is `T: Ord` to compare elements. Queries
return `Option<&T>` — a borrow, so `T` doesn't have to be `Clone` or `Copy` —
and `None` on an empty stack instead of panicking. `pop` hands the element back
by value, like `Vec::pop`.

## Examples

```rust
let mut s = MinStack::new();
s.push(-2);
s.push(0);
s.push(-3);
assert_eq!(s.min(), Some(&-3));
assert_eq!(s.pop(), Some(-3));
assert_eq!(s.peek(), Some(&0));
assert_eq!(s.min(), Some(&-2));    // the old minimum is back
assert_eq!(s.len(), 2);

let mut words: MinStack<&str> = MinStack::default();
assert_eq!(words.min(), None);     // empty: nothing to report
words.push("pear");
words.push("apple");
assert_eq!(words.min(), Some(&"apple"));
```

## Constraints

- At most `3 * 10^4` calls in total.
- Any `T: Ord`; the tests use `i32` over its full range, strings, and a type
  that is not `Clone`.

## Rust notes

- A second `Vec` alongside the elements is the natural shape. What you store in
  it matters: copies of `T` would need `T: Clone`; there is a way to avoid that.
- `Vec::last` returns `Option<&T>`, and `Option::map` lets you turn it into
  whatever you need without `unwrap`.
- If `len` is public, clippy expects an `is_empty` too; and a `new()` without
  arguments wants a `Default` impl.
- Put the `T: Ord` bound on the `impl` block, not the struct definition.
