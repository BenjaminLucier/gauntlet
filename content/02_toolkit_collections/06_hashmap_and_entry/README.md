## Description

Twenty-three small functions over [`HashMap`] and [`HashSet`]. The centre of
gravity is the [`Entry`] API: one hash lookup to find a slot, then insert,
update or both — the idiomatic replacement for "check if the key is there,
then insert or update".

Every function says in its doc comment which method it's after. Most are a
single expression or a two-line loop.

[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
[`Entry`]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

## Restrictions

- **No `contains_key` (or `get`) followed by `insert`** — insert-or-update
  goes through `entry`.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Counting with entry | `word_counts`, `group_by_len`, `first_index`, `bump`, `cached_len`, `merge_counts`, `take_one` | [`entry`], [`or_insert`], [`or_default`], [`or_insert_with`], [`and_modify`], [`Entry::Occupied`][occupied], [`OccupiedEntry::remove`][occupied-remove] |
| Looking up | `age_of`, `add_if_present`, `has_all` | [`get`], [`get_mut`], [`contains_key`] |
| Removing | `take_value`, `take_entry`, `drop_below` | [`remove`], [`remove_entry`], [`retain`] |
| Building | `roman_values`, `invert` | [`HashMap::from`][from], [`FromIterator`][collect] |
| Deterministic output | `sorted_keys`, `ranked` | [`keys`], [`sort_unstable`], [`sort_unstable_by`] |
| HashSet | `in_both`, `in_either`, `only_in_first`, `in_exactly_one`, `no_overlap`, `spellable` | [`intersection`], [`union`], [`difference`], [`symmetric_difference`], [`is_disjoint`], [`is_subset`] |

[`entry`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry
[`or_insert`]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert
[`or_default`]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_default
[`or_insert_with`]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert_with
[`and_modify`]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.and_modify
[occupied]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#variant.Occupied
[occupied-remove]: https://doc.rust-lang.org/std/collections/hash_map/struct.OccupiedEntry.html#method.remove
[`get`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get
[`get_mut`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_mut
[`contains_key`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.contains_key
[`remove`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.remove
[`remove_entry`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.remove_entry
[`retain`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.retain
[from]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#impl-From%3C%5B(K,+V);+N%5D%3E-for-HashMap%3CK,+V%3E
[collect]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#impl-FromIterator%3C(K,+V)%3E-for-HashMap%3CK,+V,+S%3E
[`keys`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.keys
[`sort_unstable`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable
[`sort_unstable_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by
[`intersection`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.intersection
[`union`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.union
[`difference`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.difference
[`symmetric_difference`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.symmetric_difference
[`is_disjoint`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.is_disjoint
[`is_subset`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.is_subset

## Examples

```rust
let counts = word_counts("the cat and the hat");
assert_eq!(counts["the"], 2);

let first = first_index(&['a', 'b', 'a']);
assert_eq!(first[&'a'], 0);                      // or_insert keeps the first value

let mut map = HashMap::new();
bump(&mut map, "x");
bump(&mut map, "x");
assert_eq!(map["x"], 2);

let ages = HashMap::from([("ferris".to_string(), 9)]);
assert_eq!(age_of(&ages, "ferris"), Some(9));    // a &str key, no allocation

let counts = HashMap::from([("b", 2), ("a", 2), ("c", 5)]);
assert_eq!(ranked(&counts), [("c", 5), ("a", 2), ("b", 2)]);

let a: HashSet<i32> = [1, 2, 3].into();
let b: HashSet<i32> = [3, 4].into();
assert_eq!(in_exactly_one(&a, &b), [1, 2, 4]);
```

## Rust notes

- `entry` takes the key by value, so a `HashMap<String, _>` needs
  `key.to_string()` there — but lookups (`get`, `get_mut`, `remove`,
  `contains_key`) take `&Q` where `K: Borrow<Q>`, so `&str` works directly.
- `or_insert(make())` evaluates `make()` even when the key exists;
  `or_insert_with(make)` only on a miss. `or_default()` needs `V: Default`.
- `HashMap` iteration order is unspecified and changes between runs. Anything
  returned as a `Vec` here is sorted.
- `entry` returns an enum: match on `Entry::Occupied` / `Entry::Vacant` when
  the two cases need different logic. An `OccupiedEntry` can `remove` itself,
  so "decrement, and delete at zero" is still one lookup.
- The set operations return lazy iterators of `&T`; `.copied()` or
  `.cloned()` before collecting.
