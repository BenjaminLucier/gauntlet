## Description

Twenty small functions over [`BTreeMap`] and [`BTreeSet`]. A B-tree keeps its
keys sorted, so beyond lookups it answers ordered questions cheaply: the
smallest and largest keys, everything between two keys, and the nearest key
below or above a value — all through [`range`].

Every function says in its doc comment which method it's after.

[`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
[`range`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.range

## Restrictions

- **No sorting** — the tree is already sorted.
- **No linear scans where a `range` will do**: `floor`, `ceiling`,
  `count_below` and friends start from the right place in the tree, not from
  the beginning with a `filter`.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Ordered iteration | `sorted_unique`, `values_by_key`, `descending_keys`, `nth_smallest` | [`FromIterator`][collect], [`into_values`], [`keys`], [`Iterator::rev`][rev], [`Iterator::nth`][nth] |
| Ranges | `values_between`, `range_sum`, `raise_between`, `count_below`, `events_from`, `with_prefix` | [`range`], [`range_mut`], [`Bound`] |
| Floor and ceiling | `floor`, `ceiling`, `strictly_after` | [`BTreeSet::range`][set-range], [`next_back`] |
| Ends | `first_and_last`, `pop_smallest`, `sell_cheapest`, `pop_latest` | [`first_key_value`], [`last_key_value`], [`pop_first`], [`first_entry`], [`OccupiedEntry::remove`][occupied-remove], [`pop_last`] |
| Splitting and editing | `split_at_key`, `keep_even_keys`, `absorb` | [`split_off`], [`retain`], [`append`] |

[collect]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#impl-FromIterator%3CT%3E-for-BTreeSet%3CT%3E
[`into_values`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.into_values
[`keys`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.keys
[rev]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev
[nth]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth
[`range_mut`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.range_mut
[`Bound`]: https://doc.rust-lang.org/std/ops/enum.Bound.html
[set-range]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.range
[`next_back`]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#tymethod.next_back
[`first_key_value`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.first_key_value
[`last_key_value`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.last_key_value
[`pop_first`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.pop_first
[`first_entry`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.first_entry
[occupied-remove]: https://doc.rust-lang.org/std/collections/btree_map/struct.OccupiedEntry.html#method.remove
[`pop_last`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.pop_last
[`split_off`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.split_off
[`retain`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.retain
[`append`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.append

## Examples

```rust
let s: BTreeSet<i32> = [10, 20, 30].into();
assert_eq!(floor(&s, 25), Some(20));            // largest <= 25
assert_eq!(ceiling(&s, 25), Some(30));          // smallest >= 25
assert_eq!(strictly_after(&s, 20), Some(30));
assert_eq!(count_below(&s, 30), 2);

let map = BTreeMap::from([(1, "a"), (3, "c"), (5, "e")]);
assert_eq!(values_between(&map, 2, 5), ["c", "e"]);
assert_eq!(values_between(&map, 5, 2), Vec::<&str>::new());  // backwards: empty, no panic

let mut stock = BTreeMap::from([(10, 1), (20, 4)]);   // price -> how many
assert_eq!(sell_cheapest(&mut stock), Some(10));
assert_eq!(stock, BTreeMap::from([(20, 4)]));    // sold out at 10: key removed

let mut m = BTreeMap::from([(1, 'a'), (3, 'c'), (5, 'e')]);
let upper = split_at_key(&mut m, 3);
assert_eq!(upper, BTreeMap::from([(3, 'c'), (5, 'e')]));
assert_eq!(m, BTreeMap::from([(1, 'a')]));
```

## Rust notes

- `range` and `range_mut` **panic** if the start is greater than the end (or
  if both are equal and both excluded). Check backwards inputs before calling
  them.
- The iterator `range` returns is double-ended and starts from a tree search,
  not from the first key: `range(..=x).next_back()` is an O(log n) floor.
- To range over a `BTreeSet<String>` with a `&str`, the compiler needs to know
  the borrowed type: `set.range::<str, _>((Bound::Included(p), Bound::Unbounded))`.
- `split_off(&k)` keeps keys `< k` and returns keys `>= k`. `append` moves every
  element out of the other collection, leaving it empty.
- `BTreeMap` iteration is always in key order, so it's the easy way to get
  deterministic output from counted or grouped data.
- `first_entry` / `last_entry` hand you an `OccupiedEntry` for the smallest /
  largest key: read or change its value, or `remove` it, without a second
  lookup.
