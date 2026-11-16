## Description

Twenty-three small functions over `&[i32]`, each a one-liner (or close) with the
right slice method or slice pattern. The goal is to reach for `windows`,
`split_first`, `partition_point` or `[first, .., last]` instead of an index
loop with off-by-one bugs waiting in it.

Every function says in its doc comment which method or idea it's after. Most
return borrowed sub-slices of the input — nothing needs copying.

## Restrictions

- **No `for`, `while` or `loop`** — slice methods, iterators and slice
  patterns only.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Windows and chunks | `has_adjacent_duplicate`, `max_window_sum`, `chunk_sums`, `full_chunks`, `chunks_from_end`, `ascending_runs` | [`windows`], [`chunks`], [`chunks_exact`], [`rchunks`], [`chunk_by`] |
| Splitting | `split_on_zero`, `split_at_first`, `head_and_tail`, `all_but_last`, `take_prefix`, `swap_halves` | [`split`], [`splitn`], [`split_first`], [`split_last`], [`split_at_checked`], [`split_at_mut`], [`swap_with_slice`] |
| Ends and membership | `first_and_last`, `framed_by`, `contains_all` | [`first`], [`last`], [`starts_with`], [`ends_with`], [`contains`] |
| Searching | `insertion_point`, `name_for_id`, `count_in_range` | [`binary_search`], [`binary_search_by_key`], [`partition_point`] |
| Joining | `flatten_rows`, `join_with` | [`concat`], [`join`] |
| Slice patterns | `inner`, `first_two_and_rest`, `is_palindrome` | [slice patterns] |

[`windows`]: https://doc.rust-lang.org/std/primitive.slice.html#method.windows
[`chunks`]: https://doc.rust-lang.org/std/primitive.slice.html#method.chunks
[`chunks_exact`]: https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact
[`rchunks`]: https://doc.rust-lang.org/std/primitive.slice.html#method.rchunks
[`chunk_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.chunk_by
[`split`]: https://doc.rust-lang.org/std/primitive.slice.html#method.split
[`splitn`]: https://doc.rust-lang.org/std/primitive.slice.html#method.splitn
[`split_first`]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_first
[`split_last`]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_last
[`split_at_checked`]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_checked
[`split_at_mut`]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut
[`swap_with_slice`]: https://doc.rust-lang.org/std/primitive.slice.html#method.swap_with_slice
[`first`]: https://doc.rust-lang.org/std/primitive.slice.html#method.first
[`last`]: https://doc.rust-lang.org/std/primitive.slice.html#method.last
[`starts_with`]: https://doc.rust-lang.org/std/primitive.slice.html#method.starts_with
[`ends_with`]: https://doc.rust-lang.org/std/primitive.slice.html#method.ends_with
[`contains`]: https://doc.rust-lang.org/std/primitive.slice.html#method.contains
[`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
[`binary_search_by_key`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search_by_key
[`partition_point`]: https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point
[`concat`]: https://doc.rust-lang.org/std/primitive.slice.html#method.concat
[`join`]: https://doc.rust-lang.org/std/primitive.slice.html#method.join
[slice patterns]: https://doc.rust-lang.org/reference/patterns.html#slice-patterns

## Examples

```rust
assert_eq!(max_window_sum(&[1, -2, 3, 4, -1], 2), Some(7));   // 3 + 4
assert_eq!(chunk_sums(&[1, 2, 3, 4, 5], 2), vec![3, 7, 5]);    // last chunk is short
assert_eq!(split_on_zero(&[1, 2, 0, 3]), vec![&[1, 2][..], &[3]]);
assert_eq!(take_prefix(&[1, 2, 3], 4), None);                  // no panic

let mut xs = [1, 2, 3, 4, 5];
swap_halves(&mut xs);
assert_eq!(xs, [4, 5, 3, 1, 2]);                               // middle stays put

assert_eq!(insertion_point(&[1, 3, 5, 7], 4), 2);
assert_eq!(count_in_range(&[1, 2, 2, 2, 5, 8, 8], 2, 8), 4);   // 2, 2, 2, 5
assert_eq!(join_with(&[vec![1, 2], vec![3]], 0), vec![1, 2, 0, 3]);
assert_eq!(inner(&[1, 2, 3, 4]), &[2, 3]);
assert!(is_palindrome(&['a', 'b', 'b', 'a']));
```

## Rust notes

- `windows(k)` and `chunks(k)` panic if `k == 0`; the tests never pass 0.
- `split_at(n)` panics when `n > len`; `split_at_checked(n)` returns `None`.
  Same story for `xs[a..b]` versus `xs.get(a..b)`.
- `binary_search` on a slice with duplicates may return *any* matching index;
  `partition_point` always gives the first index where the predicate is false,
  which makes it the tool for lower and upper bounds.
- In a `match` on a `&[T]`, the bindings are references (`&T` for elements,
  `&[T]` for `rest @ ..`). Copy elements out with `*a` or a `&[a, ..]` pattern.
