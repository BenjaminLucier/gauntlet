## Description

Twenty-two small functions, each an iterator chain built from the adaptors on
[`std::iter::Iterator`]: the lazy methods that take an iterator and return
another one. Every function says in its doc comment which adaptor it's after.

[`std::iter::Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html

## Restrictions

- **No `for`, `while` or `loop`** — and no indexing in a counting loop dressed
  up as `(0..n).map(|i| xs[i])`. Adaptors only.

## The drills

| Group | Functions | Adaptors |
|---|---|---|
| Transforming | `squares`, `evens`, `leading_digits` | [`map`], [`filter`], [`map_while`] |
| Positions | `indexed_labels`, `positions_of` | [`enumerate`], [`char_indices`] |
| Combining | `pairwise_sums`, `concat`, `framed`, `interleave`, `adjacent_differences` | [`zip`], [`chain`], [`iter::once`] |
| Slicing | `page`, `strip_leading_zeros`, `take_while_positive`, `every_nth`, `last_n`, `split_at_blank` | [`skip`], [`take`], [`skip_while`], [`take_while`], [`step_by`], [`by_ref`] |
| Reversing | `reversed_words` | [`rev`] |
| Nesting | `expand` | [`flat_map`], [`repeat_n`] |
| State | `running_total`, `run_lengths` | [`scan`], [`peekable`] / [`next_if_eq`] |
| Watching and repeating | `doubled_with_log`, `repeat_pattern` | [`inspect`], [`cycle`] |

[`map`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
[`filter`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[`map_while`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map_while
[`enumerate`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
[`char_indices`]: https://doc.rust-lang.org/std/primitive.str.html#method.char_indices
[`zip`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip
[`chain`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain
[`iter::once`]: https://doc.rust-lang.org/std/iter/fn.once.html
[`skip`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip
[`take`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take
[`skip_while`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip_while
[`take_while`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take_while
[`step_by`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
[`by_ref`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.by_ref
[`rev`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev
[`flat_map`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
[`repeat_n`]: https://doc.rust-lang.org/std/iter/fn.repeat_n.html
[`scan`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan
[`peekable`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable
[`next_if_eq`]: https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.next_if_eq
[`inspect`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect
[`cycle`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cycle

## Examples

```rust
assert_eq!(evens(&[1, 2, 3, 4]), vec![2, 4]);
assert_eq!(pairwise_sums(&[1, 2, 3], &[10, 20]), vec![11, 22]);   // zip stops at the shorter
assert_eq!(interleave(&[1, 2, 3], &[10]), vec![1, 10, 2, 3]);
assert_eq!(page(&[1, 2, 3, 4, 5], 1, 2), vec![3, 4]);              // page 1 (0-based), 2 per page
assert_eq!(split_at_blank(&["a", "", "b"]), (vec!["a"], vec!["b"]));
assert_eq!(adjacent_differences(&[1, 4, 9, 16]), vec![3, 5, 7]);
assert_eq!(running_total(&[1, 2, 3]), vec![1, 3, 6]);
assert_eq!(run_lengths("aaabcc"), vec![('a', 3), ('b', 1), ('c', 2)]);
assert_eq!(repeat_pattern(b"ab", 5), b"ababa".to_vec());
```

## Rust notes

- Adaptors are lazy: nothing happens until a consumer (`collect`, `sum`,
  `count`, …) pulls items through.
- `filter` and `skip_while` / `take_while` pass a *reference* to the item, so
  over `xs.iter()` the closure sees `&&i32`.
- `zip`, `take` and `take_while` stop early; `chain` and `cycle` keep going.
  `cycle` on an empty iterator is empty, not infinite.
- `rev` needs a `DoubleEndedIterator`; `step_by(0)` panics.
- Most adaptors take the iterator by value. `it.by_ref()` lends it instead, so
  after `it.by_ref().take_while(..)` is used up you still have `it`, positioned
  just past the item that ended the `take_while`.
