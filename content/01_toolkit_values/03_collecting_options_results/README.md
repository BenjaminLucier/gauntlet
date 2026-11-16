## Description

Twenty small functions about turning *many* `Option`s and `Result`s into one:
collecting until the first failure, summing, folding with an early exit,
keeping only the good values, and splitting good from bad.

Every function says in its doc comment which method or idea it's after. Each
is a single iterator chain.

## Restrictions

- **No `for`, `while` or `loop`** — iterator chains only.
- No `unwrap()` / `expect()`.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Collecting | `parse_all`, `all_present`, `lookup_all`, `parse_unique`, `collect_string`, `try_map`, `parse_lines` | [`FromIterator for Result`][fi-result], [`FromIterator for Option`][fi-option] |
| Summing | `sum_parsed`, `product_present` | [`Sum for Result`][sum-result], [`Product for Option`][product-option] |
| Folding with an exit | `sum_until_negative`, `running_balance`, `all_non_negative` | [`try_fold`], [`ControlFlow`], [`try_for_each`] |
| Keeping the good ones | `valid_numbers`, `present_values`, `ok_values` | [`filter_map`], [`flatten`] |
| Finding | `first_number`, `first_error` | [`find_map`] |
| Splitting | `split_parsed` | [`partition`], [`Result::err`][err] |
| Nesting | `parse_optional_fields`, `max_parsed` | [`Option::transpose`][transpose], [`Result::map`][map] |

[fi-result]: https://doc.rust-lang.org/std/result/enum.Result.html#impl-FromIterator%3CResult%3CA,+E%3E%3E-for-Result%3CV,+E%3E
[fi-option]: https://doc.rust-lang.org/std/option/enum.Option.html#impl-FromIterator%3COption%3CA%3E%3E-for-Option%3CV%3E
[sum-result]: https://doc.rust-lang.org/std/result/enum.Result.html#impl-Sum%3CResult%3CU,+E%3E%3E-for-Result%3CT,+E%3E
[product-option]: https://doc.rust-lang.org/std/option/enum.Option.html#impl-Product%3COption%3CU%3E%3E-for-Option%3CT%3E
[`try_fold`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold
[`ControlFlow`]: https://doc.rust-lang.org/std/ops/enum.ControlFlow.html
[`try_for_each`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_for_each
[`filter_map`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map
[`flatten`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten
[`find_map`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find_map
[`partition`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition
[err]: https://doc.rust-lang.org/std/result/enum.Result.html#method.err
[transpose]: https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose
[map]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map

## Examples

```rust
assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
assert!(parse_all(&["1", "two", "3"]).is_err());          // stops at "two"

assert_eq!(all_present(&[Some(1), Some(2)]), Some(vec![1, 2]));
assert_eq!(all_present(&[Some(1), None]), None);

assert_eq!(sum_until_negative(&[4, 5, -1, 100]), 9);      // Break(9) at the -1
assert_eq!(running_balance(10, &[-5, 3, -9, 100]), Err(2)); // 10-5+3-9 < 0 at index 2

assert_eq!(split_parsed(&["1", "x", "2", "y"]), (vec![1, 2], vec!["x", "y"]));
assert_eq!(parse_optional_fields(&[Some("4"), None]), Ok(vec![Some(4), None]));
```

## Rust notes

- `collect` needs to know the target type: annotate the binding, the return
  type, or use a turbofish (`collect::<Result<Vec<_>, _>>()`).
- Collecting into `Result` / `Option` short-circuits: the iterator isn't
  polled past the first `Err` / `None`, so side effects after it never run.
- `filter_map(|w| w.parse().ok())` throws the errors away silently; that's
  the point, but make sure it's what you want.
- `try_fold` works with any `Try` type, not just `Option` and `Result`.
  `ControlFlow<B, C>` says what it means: `Continue(acc)` keeps going,
  `Break(value)` stops with a result that isn't an error.
- `partition` needs the collection type for both halves:
  `let (a, b): (Vec<_>, Vec<_>) = ...`.
