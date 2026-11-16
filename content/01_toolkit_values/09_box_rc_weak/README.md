## Description

Eighteen drills on the three pointer types you reach for when a plain
reference isn't enough:

- **`Box<T>`** — one owner, value on the heap. Needed for trait objects
  (`Box<dyn Shape>`) and recursive types (an `Expr` that contains `Expr`s).
- **`Rc<T>`** — several owners of one value; it's freed when the last one goes.
- **`Weak<T>`** — a non-owning pointer into an `Rc`, for links that point
  *back* (child → parent) and so must not keep their target alive.

Every function says in its doc comment which function or idea it's after. The
`Shape` impls, the `Drop` impl on `TreeNode` and `dropped_nodes` are given.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Trait objects | `make_shape`, `total_area`, `largest` | [`Box<dyn Trait>`][box], [`f64::total_cmp`] |
| A recursive enum | `add`, `eval`, `depth`, `cancel_double_neg` | [`Box::new`], [`Deref`] (`&Box<T>` → `&T`), moving out with `*` |
| Rc | `share`, `owners`, `same_allocation`, `into_inner_if_unique`, `take_or_clone`, `push_cow` | [`Rc::clone`], [`Rc::strong_count`], [`Rc::ptr_eq`], [`Rc::try_unwrap`], [`Rc::unwrap_or_clone`], [`Rc::make_mut`] |
| Weak parent pointers | `TreeNode::new`, `add_child`, `parent_name`, `path`, `watchers` | [`Weak::new`], [`Rc::downgrade`], [`Weak::upgrade`], [`Rc::weak_count`] |

[box]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`f64::total_cmp`]: https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp
[`Box::new`]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.new
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`Rc::clone`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#impl-Clone-for-Rc%3CT,+A%3E
[`Rc::strong_count`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.strong_count
[`Rc::ptr_eq`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.ptr_eq
[`Rc::try_unwrap`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.try_unwrap
[`Rc::unwrap_or_clone`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.unwrap_or_clone
[`Rc::make_mut`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.make_mut
[`Weak::new`]: https://doc.rust-lang.org/std/rc/struct.Weak.html#method.new
[`Rc::downgrade`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.downgrade
[`Weak::upgrade`]: https://doc.rust-lang.org/std/rc/struct.Weak.html#method.upgrade
[`Rc::weak_count`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.weak_count

## Examples

```rust
let shapes = vec![make_shape("square", 2.0).unwrap(), make_shape("circle", 1.0).unwrap()];
assert_eq!(largest(&shapes).map(|s| s.name()), Some("square"));   // 4 > pi

let e = Expr::Neg(Box::new(Expr::Neg(Box::new(Expr::Num(3)))));
assert_eq!(cancel_double_neg(e), Expr::Num(3));

let mut a = Rc::new(vec![1]);
let b = Rc::clone(&a);
push_cow(&mut a, 2);                       // shared, so `a` gets its own copy
assert_eq!((&*a, &*b), (&vec![1, 2], &vec![1]));

let root = TreeNode::new("usr");
let bin = TreeNode::new("bin");
add_child(&root, Rc::clone(&bin));
assert_eq!(path(&bin), ["usr", "bin"]);
assert_eq!(watchers(&root), 1);            // bin's parent link
```

## Rust notes

- `Rc::clone(&rc)` is the idiomatic spelling: it reads as "another owner",
  not "a deep copy".
- `Rc` is single-threaded; its counts are plain integers. `Arc` is the
  thread-safe twin with the same API.
- If parent and child held `Rc`s to each other, neither count could reach
  zero and both would leak. With a `Weak` parent link, dropping the root frees
  the whole tree — the last test checks that with a drop counter.
- `TreeNode`'s links sit in `RefCell`s because nodes are shared through `Rc`
  (so only `&TreeNode` is available) yet `add_child` must change them;
  `RefCell` checks those borrows at run time instead of compile time.
