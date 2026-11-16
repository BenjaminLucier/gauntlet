## Description

Twenty-three small functions, each ending in a *consumer*: the iterator methods
that run the iteration and produce a value — a number, a `bool`, an `Option`,
or a collection. Every function says in its doc comment which method it's
after.

## Restrictions

- **No `for`, `while` or `loop`** — and no indexing loops in disguise.
- No `unwrap()` / `expect()`.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Arithmetic | `total`, `factorial`, `mean` | [`sum`], [`product`] |
| Counting and testing | `count_vowels`, `has_negative`, `all_lowercase`, `same_words` | [`count`], [`any`], [`all`], [`eq`] |
| Extremes | `shortest_word`, `longest_word`, `min_max` | [`min_by_key`], [`max_by_key`], [`fold`] |
| Folding | `to_number`, `hyphenate`, `nest` | [`fold`], [`reduce`], [`rfold`] |
| Searching | `index_of`, `last_index_of`, `first_long_word`, `last_before_zero`, `nth_word` | [`position`], [`rposition`], [`find`], [`last`], [`nth`] |
| Splitting | `split_even_odd`, `split_pairs` | [`partition`], [`unzip`] |
| Collecting | `word_lengths`, `initials`, `distinct_sorted` | [`collect`] into `HashMap`, `String`, `BTreeSet` |

[`sum`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
[`product`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product
[`count`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count
[`any`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
[`all`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all
[`min_by_key`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min_by_key
[`max_by_key`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key
[`eq`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.eq
[`fold`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
[`reduce`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce
[`rfold`]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#method.rfold
[`position`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position
[`rposition`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rposition
[`find`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
[`last`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.last
[`nth`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth
[`partition`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition
[`unzip`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip
[`collect`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect

## Examples

```rust
assert_eq!(factorial(5), 120);
assert_eq!(longest_word("a bb cc d"), Some("cc"));        // max_by_key: last of the ties
assert_eq!(shortest_word("bb a c dd"), Some("a"));        // min_by_key: first of the ties
assert!(same_words("a  b", "a b"));                        // spacing ignored
assert_eq!(min_max(&[3, -1, 7]), Some((-1, 7)));
assert_eq!(to_number(&[4, 0, 2]), 402);
assert_eq!(hyphenate(&["a", "b", "c"]), Some("a-b-c".to_string()));
assert_eq!(nest(&["f", "g", "x"]), "f(g(x))");
assert_eq!(split_pairs(&[('h', 1), ('i', 2)]), ("hi".to_string(), vec![1, 2]));
```

## Rust notes

- `sum` and `product` need the result type: `iter.sum::<i64>()`, or let the
  return type decide. Widen *before* summing (`map(i64::from)`) or the sum can
  overflow its element type.
- `Iterator::eq` (and `cmp`, `lt`, …) compare two iterators item by item and
  stop at the first difference: no need to collect either side.
- `rfold` is `fold` from the back; it needs a `DoubleEndedIterator`. Reach for
  it when the innermost part of the result comes from the *last* item.
- `last()` walks the whole iterator. On a double-ended iterator `next_back()`
  is O(1) and clippy will say so; `last` is for iterators that can only go
  forwards, like a `take_while`.
- `rposition` needs an iterator that knows its length and can run backwards
  (`ExactSizeIterator + DoubleEndedIterator`) — a slice iterator does.
