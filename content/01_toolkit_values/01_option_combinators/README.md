## Description

Twenty-one small functions, each a one-liner with the right `Option` method.
The goal is to know what's in [`std::option::Option`] well enough that you
never write `match opt { Some(x) => Some(f(x)), None => None }` again.

Every function says in its doc comment which method or idea it's after. Most
are a single method call; a few chain two.

[`std::option::Option`]: https://doc.rust-lang.org/std/option/enum.Option.html

## Restrictions

- **No `match`, `if let`, `let … else` or `?`** — combinators only.
- No `unwrap()` / `expect()`.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Transforming | `double`, `len_or_zero`, `first_char_upper` | [`map`], [`map_or`] |
| Defaults | `name_or_anonymous`, `or_else_build` | [`unwrap_or`], [`unwrap_or_else`] |
| Filtering and chaining | `even_only`, `parse_positive`, `lookup_twice`, `flatten_nested` | [`filter`], [`Result::ok`], [`and_then`], [`flatten`] |
| Combining | `first_some`, `exactly_one`, `add_both`, `split_pair` | [`or`], [`xor`], [`zip`], [`unzip`] |
| Inspecting | `is_positive` | [`is_some_and`] |
| Slots (`&mut Option`) | `take_out`, `swap_in`, `get_or_start` | [`take`], [`replace`], [`get_or_insert_with`] |
| Borrowing | `as_name` | [`as_deref`] |
| To and from `Result` | `require`, `parse_if_present` | [`ok_or`], [`transpose`] |
| Many at once | `sum_all` | [`Sum for Option`][sum] |

[`map`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.map
[`map_or`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or
[`unwrap_or`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or
[`unwrap_or_else`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else
[`filter`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.filter
[`Result::ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.ok
[`and_then`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then
[`flatten`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.flatten
[`or`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.or
[`xor`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.xor
[`zip`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.zip
[`unzip`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unzip
[`is_some_and`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some_and
[`take`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.take
[`replace`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.replace
[`get_or_insert_with`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert_with
[`as_deref`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref
[`ok_or`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
[`transpose`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose
[sum]: https://doc.rust-lang.org/std/option/enum.Option.html#impl-Sum%3COption%3CU%3E%3E-for-Option%3CT%3E

## Examples

```rust
assert_eq!(double(Some(4)), Some(8));
assert_eq!(name_or_anonymous(None), "anonymous");
assert_eq!(parse_positive("0"), None);              // parses, but isn't positive
assert_eq!(exactly_one(Some(1), Some(2)), None);    // both: not exactly one

let mut slot = Some(String::from("x"));
assert_eq!(take_out(&mut slot).as_deref(), Some("x"));
assert_eq!(slot, None);                             // take leaves None behind

assert_eq!(sum_all(&[Some(1), Some(2)]), Some(3));
assert_eq!(sum_all(&[Some(1), None]), None);        // any None makes it None
```

## Rust notes

- Most `Option` methods take `self` by value. On an `&Option<T>` you want
  `as_ref()` (→ `Option<&T>`) or `as_deref()` (→ `Option<&str>` for an
  `Option<String>`) first.
- `unwrap_or(expensive())` evaluates `expensive()` even when the option is
  `Some`; `unwrap_or_else(|| expensive())` doesn't.
- `Option<T>` implements `Sum`, `Product` and `FromIterator` whenever `T`
  does: summing or collecting stops at the first `None`.
- `as_name` takes `&Option<String>` because that's what you have when you
  borrow a struct field. As a *parameter* type in your own APIs, prefer
  `Option<&str>` — callers can always produce one with `as_deref()`.
