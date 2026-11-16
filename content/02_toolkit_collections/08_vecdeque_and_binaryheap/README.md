## Description

Seventeen small functions over [`VecDeque`], the growable ring buffer, and
[`BinaryHeap`], the priority queue. A deque is the right tool whenever you
push or pop at the front; a heap whenever you repeatedly want "the largest (or
smallest) so far".

Every function says in its doc comment which method it's after.

[`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
[`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html

## Restrictions

- **No `sort` / `sort_unstable`**, except on the slice `make_contiguous`
  returns in `sort_deque`. Ordering comes from the heap.
- **No `Vec::remove(0)` or `Vec::insert(0, _)`** — that's what a deque is for.

## The drills

| Group | Functions | Methods |
|---|---|---|
| VecDeque: both ends | `push_both`, `is_palindrome`, `rotate`, `elimination_order` | [`push_front`], [`push_back`], [`pop_front`], [`pop_back`], [`rotate_left`], [`rotate_right`] |
| VecDeque: sliding buffers | `last_k`, `recent_counts` | [`front`], [`pop_front`] |
| VecDeque: as slices | `sort_deque`, `total`, `middle` | [`make_contiguous`], [`as_slices`], [`range`] |
| BinaryHeap | `peek_max`, `largest_k`, `smallest_k`, `heap_sorted`, `merge_into`, `drain_total`, `halve_largest`, `drop_odds` | [`peek`], [`From<Vec<T>>`][from], [`pop`], [`Reverse`], [`into_sorted_vec`], [`append`], [`drain`], [`peek_mut`], [`retain`] |

[`push_front`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.push_front
[`push_back`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.push_back
[`pop_front`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front
[`pop_back`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_back
[`rotate_left`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.rotate_left
[`rotate_right`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.rotate_right
[`front`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.front
[`make_contiguous`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.make_contiguous
[`as_slices`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.as_slices
[`range`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.range
[`peek`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.peek
[from]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#impl-From%3CVec%3CT,+A%3E%3E-for-BinaryHeap%3CT,+A%3E
[`pop`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.pop
[`Reverse`]: https://doc.rust-lang.org/std/cmp/struct.Reverse.html
[`into_sorted_vec`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.into_sorted_vec
[`append`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.append
[`drain`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.drain
[`peek_mut`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.peek_mut
[`retain`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.retain

## Examples

```rust
assert_eq!(push_both(&[1, 2, 3], &[4, 5]), [3, 2, 1, 4, 5]);

let mut d: VecDeque<i32> = (1..=5).collect();
rotate(&mut d, 2);
assert_eq!(d, [4, 5, 1, 2, 3]);
rotate(&mut d, 7);                               // 7 % 5 == 2 more steps
assert_eq!(d, [2, 3, 4, 5, 1]);

assert_eq!(last_k(&[1, 2, 3, 4, 5], 3), [3, 4, 5]);
assert_eq!(elimination_order(7, 3), [3, 6, 2, 7, 5, 1, 4]);

assert_eq!(smallest_k(&[5, 1, 9, -3], 2), [-3, 1]);
assert_eq!(largest_k(&[5, 1, 9, -3], 2), [9, 5]);

let mut heap = BinaryHeap::from(vec![10, 7, 3]);
halve_largest(&mut heap);                        // 10 becomes 5 ...
assert_eq!(heap.peek(), Some(&7));               // ... and sinks below 7
```

## Rust notes

- `rotate_left(k)` / `rotate_right(k)` **panic** when `k > len()`. They cost
  O(min(k, len - k)).
- A `VecDeque` can be indexed (`d[i]`) and compared with arrays and `Vec`s, and
  converts into a `Vec` with `.into()` / `Vec::from` — free when the buffer
  doesn't wrap.
- `BinaryHeap::from(vec)` heapifies in O(n), cheaper than n pushes.
  `into_sorted_vec` returns *ascending* order, while popping gives descending.
- Iterating a `BinaryHeap` (`iter`, `into_vec`, `drain`) is in arbitrary
  order. Only `peek`, `pop` and `into_sorted_vec` respect the ordering — fine
  for a sum, not for a ranking.
- `VecDeque::range` panics on a range that runs past the end, just as slicing
  does.
- `peek_mut` returns a guard; change the top through it and the heap sifts it
  back into place when the guard is dropped.
