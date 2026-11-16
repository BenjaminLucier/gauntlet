## Description

Nineteen small drills on moving values in and out of places you only have a
`&mut` to. The borrow checker won't let you move out of a borrow and leave a
hole, so [`std::mem`] gives you ways to swap something valid in at the same
time: `take`, `replace` and `swap`. The same idea shows up as `Option::take`,
`Vec::drain` and `Vec::append`.

Every function says in its doc comment which function or idea it's after. The
odd one out is `Client::handle`, a small state machine whose transitions move
owned data from the old state into the new one.

[`std::mem`]: https://doc.rust-lang.org/std/mem/index.html

## Restrictions

- **No `clone()`, `to_owned()` or `to_string()` on values you already own.**
  Move them instead.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Taking and replacing | `take_string`, `replace_name`, `flush` | [`mem::take`], [`mem::replace`] |
| Swapping | `swap_buffers`, `rotate_three`, `normalize` | [`mem::swap`] |
| Option slots | `pop_front`, `take_if_long` | [`Option::take`], [`Option::take_if`] |
| Draining and appending | `drain_keep_capacity`, `move_all` | [`Vec::drain`], [`Vec::append`] |
| A state machine | `Client::new`, `Client::handle`, `Client::take_log` | [`mem::take`], [`Default`] |
| Slices | `pair_mut`, `swap_contents` | [`split_at_mut`], [`swap_with_slice`] |
| Sizes and discriminants | `box_sizes`, `option_sizes`, `same_variant`, `count_variants` | [`mem::size_of`], [`mem::discriminant`] |

[`mem::take`]: https://doc.rust-lang.org/std/mem/fn.take.html
[`mem::replace`]: https://doc.rust-lang.org/std/mem/fn.replace.html
[`mem::swap`]: https://doc.rust-lang.org/std/mem/fn.swap.html
[`Option::take`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.take
[`Option::take_if`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.take_if
[`Vec::drain`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain
[`Vec::append`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append
[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[`split_at_mut`]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut
[`swap_with_slice`]: https://doc.rust-lang.org/std/primitive.slice.html#method.swap_with_slice
[`mem::size_of`]: https://doc.rust-lang.org/std/mem/fn.size_of.html
[`mem::discriminant`]: https://doc.rust-lang.org/std/mem/fn.discriminant.html

### The `Client` state machine

| State | Event | New state |
|---|---|---|
| `Idle` | `Dial` | `Connecting { attempts: 1 }` |
| `Connecting { attempts }` | `Fail` | `Connecting { attempts + 1 }`, or `Closed { reason: "gave up" }` if `attempts` was already `MAX_ATTEMPTS` |
| `Connecting { .. }` | `Accept(session)` | `Connected { session }` |
| `Connected { session }` | `Hangup(reason)` | `Closed { reason }`, and `session` is pushed onto `log` |
| anything else | | unchanged |

## Examples

```rust
let mut s = String::from("hello");
assert_eq!(take_string(&mut s), "hello");
assert_eq!(s, "");                                  // Default::default() left behind

let mut slot = Some("hi".to_string());
assert_eq!(take_if_long(&mut slot, 3), None);       // too short: stays in the slot

let mut c = Client::new();
c.handle(Event::Dial);
c.handle(Event::Accept("s-42".into()));
assert_eq!(c.state, Conn::Connected { session: "s-42".into() });

let mut v = [10, 20, 30];
let (a, b) = pair_mut(&mut v, 0, 2);                // two &mut into one slice
std::mem::swap(a, b);
assert_eq!(v, [30, 20, 10]);
// pair_mut(&mut v, 2, 2) panics with "pair_mut needs i < j"

let mut span = Span { start: "m".into(), end: "c".into() };
normalize(&mut span);                               // swaps two fields of one struct
assert_eq!((span.start.as_str(), span.end.as_str()), ("c", "m"));

assert!(same_variant(&Conn::Connecting { attempts: 1 }, &Conn::Connecting { attempts: 5 }));
```

## Rust notes

- `mem::take` needs `T: Default`; that's why `Conn` derives `Default` with
  `#[default] Idle` — a cheap placeholder for the instant the state is out.
- `mem::take` on a `Vec` hands over the allocation and leaves a fresh,
  unallocated `Vec`; `drain(..)` moves the elements out but keeps the buffer.
- Two `&mut` into one slice need a proof that they don't overlap:
  `split_at_mut` gives you two disjoint halves. Two `&mut` to different
  *fields* of one struct need no proof: the borrow checker tracks fields
  separately (but not through a method call like `span.start_mut()`).
- `Option<Box<T>>`, `Option<&T>` and `Option<NonZeroU32>` cost nothing extra:
  the compiler stores `None` in a bit pattern the inner type can never have
  (a *niche*).
