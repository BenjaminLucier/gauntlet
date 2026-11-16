## Description

Twenty small functions about [`std::result::Result`]: transforming one side,
chaining fallible steps, falling back, peeking at errors without consuming
them, and the `?` operator with a `From` conversion into a small error enum.

Every function says in its doc comment which method or idea it's after. The
error enum `ConfigError` (and its `From<ParseIntError>` impl) is given.

[`std::result::Result`]: https://doc.rust-lang.org/std/result/enum.Result.html

## Restrictions

- **No `match` or `if let` on a `Result`** — combinators and `?` only.
- No `unwrap()` / `expect()`.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Transforming | `parse_double`, `describe_error`, `parse_or_tag` | [`map`], [`map_err`] |
| Chaining | `parse_even`, `parse_with_fallback`, `first_ok`, `then_second` | [`and_then`], [`or_else`], [`or`], [`and`] |
| Defaults | `parse_or_zero`, `render` | [`unwrap_or_default`], [`map_or_else`] |
| Inspecting | `is_small_number`, `failed_because_empty`, `parse_logging` | [`is_ok_and`], [`is_err_and`], [`as_ref`], [`inspect`], [`inspect_err`] |
| To `Option` and borrowing | `ok_value`, `err_value`, `borrow_message` | [`ok`], [`err`], [`as_deref`] |
| The `?` operator | `parse_and_add`, `parse_port`, `parse_setting` | [`?` and `From`][question] |
| `Result<(), E>` | `check_all_below`, `apply_settings` | early `return Err(..)`, `Ok(())` |

[`map`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map
[`map_err`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err
[`and_then`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then
[`or_else`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.or_else
[`or`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.or
[`and`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.and
[`unwrap_or_default`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default
[`map_or_else`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or_else
[`is_ok_and`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok_and
[`is_err_and`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.is_err_and
[`as_ref`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.as_ref
[`inspect`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.inspect
[`inspect_err`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.inspect_err
[`ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.ok
[`err`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.err
[`as_deref`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.as_deref
[question]: https://doc.rust-lang.org/std/result/index.html#the-question-mark-operator-

## Examples

```rust
assert_eq!(parse_double("21"), Ok(42));
assert_eq!(parse_even("7"), Err("7 is odd".to_string()));
assert_eq!(parse_with_fallback("oops", || "5".parse()), Ok(5)); // first failed, fallback ran
assert_eq!(parse_or_zero("nope"), 0);                   // i32::default()

assert_eq!(parse_port("8080"), Ok(8080));
assert_eq!(parse_port("70000"), Err(ConfigError::OutOfRange(70000)));
assert!(matches!(parse_port("http"), Err(ConfigError::BadNumber(_)))); // via From
assert!(matches!(parse_port("99999999999999999999"), Err(ConfigError::BadNumber(_)))); // too big for i64

assert_eq!(parse_setting(" depth = 3 "), Ok(("depth".to_string(), 3)));
assert_eq!(parse_setting("depth"), Err(ConfigError::MissingEquals));
```

## Rust notes

- Like `Option`, most `Result` methods take `self` by value. On a
  `&Result<T, E>` call `as_ref()` first to get a `Result<&T, &E>`.
- `ParseIntError::kind()` returns an [`IntErrorKind`] you can compare against
  (`Empty`, `InvalidDigit`, `PosOverflow`, …).
- `?` on a `Result<T, E1>` inside a function returning `Result<U, E2>` needs
  `E2: From<E1>`. On an `Option` it only works inside a function returning
  `Option` — convert with `ok_or` first.
- `inspect` / `inspect_err` run a closure on a borrowed value and hand the
  `Result` back unchanged: handy for logging in the middle of a chain.

[`IntErrorKind`]: https://doc.rust-lang.org/std/num/enum.IntErrorKind.html
