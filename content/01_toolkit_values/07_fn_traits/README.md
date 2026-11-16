## Description

Twenty drills on passing behaviour around. Every closure implements one or
more of three traits, depending on what it does with what it captured:

| Trait | The closure… | Callable |
|---|---|---|
| [`Fn`] | only reads its captures | any number of times, even through `&` |
| [`FnMut`] | mutates its captures | any number of times, through `&mut` |
| [`FnOnce`] | moves a capture out (consumes it) | once |

Every `Fn` is also `FnMut`, and every `FnMut` is also `FnOnce`, so a function
that asks for the *loosest* bound its body allows accepts the most callers:

- it calls the closure once → `FnOnce`;
- it calls it several times, one call after another → `FnMut`;
- it can only reach the closure through a shared reference (`&dyn Fn`, `&F`,
  a closure shared between threads) → `Fn`.

The drills are grouped that way, and the tests use closures that need each
group's bound: consuming closures for the `FnOnce` functions, mutating ones for
the `FnMut` functions. Plain functions and methods are values too, and
implement all three.

Every function says in its doc comment which trait or idea it's after.

[`Fn`]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[`FnMut`]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[`FnOnce`]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html

## Restrictions

- In the *Functions as values* group, pass a path (`str::len`, `i32::abs`,
  `Result::ok`), not a closure.
- `running_totals` must be built on `visit`.

## The drills

| Group | Functions | Traits and methods |
|---|---|---|
| FnOnce | `apply`, `run_once`, `call_or_default`, `fire_once` | [`FnOnce`], [`Option::take`] |
| FnMut | `apply_twice`, `apply_n`, `count_where`, `best_by`, `call_n`, `visit`, `running_totals`, `retry` | [`FnMut`], [`Iterator::max_by_key`] |
| Fn | `map_all`, `total_by` | [`Fn`], [`&dyn Fn`][dyn] |
| move | `greeters` | [`move` closures][move] |
| Functions as values | `lengths`, `abs_all`, `parse_all` | [`str::len`], [`i32::abs`], [`str::parse`], [`Result::ok`] |
| Collections of closures | `apply_all`, `standard_steps` | [`Box<dyn Fn>`][box] |

[`Iterator::max_by_key`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key
[move]: https://doc.rust-lang.org/std/keyword.move.html
[`str::len`]: https://doc.rust-lang.org/std/primitive.str.html#method.len
[`i32::abs`]: https://doc.rust-lang.org/std/primitive.i32.html#method.abs
[`str::parse`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[`Result::ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.ok
[box]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[dyn]: https://doc.rust-lang.org/reference/types/trait-object.html
[`Option::take`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.take

## Examples

```rust
assert_eq!(apply(i32::abs, -7), 7);                 // a method path is a value
assert_eq!(apply_n(|x| x * 2, 10, 1), 1024);

let mut count = 0;
call_n(5, || count += 1);                           // FnMut: mutates `count`
assert_eq!(count, 5);

assert_eq!(running_totals(&[1, 2, 3]), [1, 3, 6]);

let s = String::from("hey");
let mut slot: Option<Box<dyn FnOnce() -> String>> = Some(Box::new(move || s + "!"));
assert_eq!(fire_once(&mut slot), Some("hey!".to_string()));
assert_eq!(fire_once(&mut slot), None);             // FnOnce: it's gone now

assert_eq!(best_by(&["aa", "bbb", "ccc"], str::len), Some("bbb"));  // earliest on a tie

assert_eq!(apply_all(&standard_steps(), 4), [5, 8, 1]);  // 4 + 1, 4 * 2, 4 - 3
```

## Rust notes

- `impl Fn(i32) -> i32` in argument position is shorthand for a generic
  parameter: each call site gets its own monomorphized copy. `&dyn Fn` is one
  compiled function calling through a vtable.
- Closures capture as little as they can: by `&`, then by `&mut`, then by
  value. `move` forces by-value capture — needed when the closure outlives the
  current scope (returned, boxed, sent to a thread).
- Every closure has its own anonymous type, so a `Vec` of different closures
  needs a common type: `Box<dyn Fn(i32) -> i32>`.
