## Description

Twenty-two small functions that change a `Vec` or a slice in place, each a
one-liner (or close) with the right method. The goal is to stop writing
`*v = v.iter().filter(..).cloned().collect()` and index-juggling loops, and to
know which edits `Vec` gives you for free.

The signatures follow the usual rule: `&mut Vec<T>` when the length changes,
`&mut [T]` when it doesn't. Every function says in its doc comment which method
it's after.

## Restrictions

- **Edit the buffer you're given.** No building a new `Vec` and assigning it
  back (`*v = …collect()`), no cloning the input to work on a copy.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Removing | `drop_negatives`, `tick_down`, `collapse_runs`, `one_per_decade`, `cut_range`, `extract_evens`, `split_tail`, `keep_first`, `remove_unordered` | [`retain`], [`retain_mut`], [`dedup`], [`dedup_by_key`], [`drain`], [`extract_if`], [`split_off`], [`truncate`], [`swap_remove`] |
| Growing | `fit_to`, `append_all`, `double_up`, `insert_sorted` | [`resize`], [`extend_from_slice`], [`extend_from_within`], [`partition_point`], [`insert`] |
| Rearranging | `rotate`, `reverse_each_chunk`, `swap_ends`, `reset`, `number_from`, `clamp_all` | [`rotate_right`], [`rotate_left`], [`chunks_mut`], [`reverse`], [`swap`], [`fill`], [`fill_with`], [`wrapping_add`], [`iter_mut`] |
| Sorting | `sort_by_distance`, `sort_people`, `sort_case_insensitive` | [`sort_unstable_by_key`], [`sort_by`], [`then_with`], [`sort_by_cached_key`] |

[`retain`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain
[`retain_mut`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain_mut
[`dedup`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup
[`dedup_by_key`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by_key
[`drain`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain
[`extract_if`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extract_if
[`split_off`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off
[`truncate`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate
[`swap_remove`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap_remove
[`resize`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.resize
[`extend_from_slice`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend_from_slice
[`extend_from_within`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend_from_within
[`partition_point`]: https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point
[`insert`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert
[`rotate_right`]: https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right
[`rotate_left`]: https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_left
[`chunks_mut`]: https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_mut
[`reverse`]: https://doc.rust-lang.org/std/primitive.slice.html#method.reverse
[`swap`]: https://doc.rust-lang.org/std/primitive.slice.html#method.swap
[`fill`]: https://doc.rust-lang.org/std/primitive.slice.html#method.fill
[`fill_with`]: https://doc.rust-lang.org/std/primitive.slice.html#method.fill_with
[`wrapping_add`]: https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_add
[`iter_mut`]: https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut
[`sort_unstable_by_key`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by_key
[`sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
[`then_with`]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then_with
[`sort_by_cached_key`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_cached_key

## Examples

```rust
let mut v = vec![3, 1, 2, 1, 5];
tick_down(&mut v);                         // decrement, drop the zeros
assert_eq!(v, [2, 1, 4]);

let mut v = vec![1, 2, 3, 4, 6, 7];
assert_eq!(extract_evens(&mut v), [2, 4, 6]);
assert_eq!(v, [1, 3, 7]);

let mut v = vec![10, 20, 30, 40];
assert_eq!(remove_unordered(&mut v, 1), 20);
assert_eq!(v, [10, 40, 30]);               // the last element filled the hole

let mut xs = [1, 2, 3, 4, 5];
rotate(&mut xs, 7);                        // 7 steps on 5 elements is 2
assert_eq!(xs, [4, 5, 1, 2, 3]);

let mut xs = [1, 2, 3, 4, 5, 6, 7, 8];
reverse_each_chunk(&mut xs, 3);
assert_eq!(xs, [3, 2, 1, 6, 5, 4, 8, 7]);

let mut xs = [1, 9, 5, 4, 6, 10];
sort_by_distance(&mut xs, 5);
assert_eq!(xs, [5, 4, 6, 1, 9, 10]);       // ties: smaller number first
```

## Rust notes

- `remove(i)` and `insert(i, x)` shift everything after `i`: O(n).
  `swap_remove` is O(1) but doesn't keep the order.
- `drain`, `extract_if` and `split_off` hand back the removed elements, so
  nothing is lost or copied. `drain(..)` empties the vector but keeps its
  capacity.
- Unstable sorts may reorder equal elements; make the key total (add a
  tie-breaker) when the order of ties matters. `sort_by_cached_key` is stable
  and calls the key function once per element instead of once per comparison.
- `x.abs_diff(y)` returns the unsigned distance and can't overflow, unlike
  `(x - y).abs()`.
