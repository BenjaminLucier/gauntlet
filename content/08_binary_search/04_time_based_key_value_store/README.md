## Description

Design a key-value store that keeps **every version** of a key's value, each
stamped with the time it was set, and can answer "what was the value of this
key at time `t`?".

Implement `TimeMap<V>`:

- `TimeMap::new()` — creates an empty store (also available via `Default`).
- `set(key, value, timestamp)` — records `value` for `key` at `timestamp`.
- `get(key, timestamp)` — returns the value from the `set` call for `key` with
  the **largest timestamp `<= timestamp`**, or `None` if there is no such call
  (unknown key, or every stored timestamp for it is later).

Timestamps passed to `set` are **strictly increasing** across all calls; your
store may rely on that.

## Signature

```rust
pub struct TimeMap<V> { /* your fields */ }

impl<V> TimeMap<V> {
    pub fn new() -> Self
    pub fn set(&mut self, key: &str, value: V, timestamp: u32)
    pub fn get(&self, key: &str, timestamp: u32) -> Option<&V>
}

impl<V> Default for TimeMap<V>
```

The store is generic over the value type `V` and needs nothing from it: values
are moved in by `set` and lent out by `get` as `Option<&V>`, so no cloning, and
"nothing there" is `None` rather than an empty string. Keys are borrowed as
`&str`; the store makes its own owned copy only when it sees a key for the first
time. Timestamps are positive integers, so `u32`.

## Examples

```rust
let mut tm = TimeMap::new();
tm.set("foo", "bar", 1);
assert_eq!(tm.get("foo", 1), Some(&"bar"));
assert_eq!(tm.get("foo", 3), Some(&"bar"));   // latest at or before 3 is time 1
tm.set("foo", "bar2", 4);
assert_eq!(tm.get("foo", 4), Some(&"bar2"));
assert_eq!(tm.get("foo", 5), Some(&"bar2"));
assert_eq!(tm.get("foo", 0), None);           // before the first set
assert_eq!(tm.get("baz", 9), None);           // never set

let mut prices: TimeMap<u64> = TimeMap::default();
prices.set("btc", 30_000, 10);
assert_eq!(prices.get("btc", 11), Some(&30_000));
```

## Constraints

- `1 <= key.len() <= 100`
- `0 <= timestamp <= u32::MAX`; timestamps of `set` calls are strictly increasing.
- At most `2 * 10^5` calls to `set` and `get` in total.

## Rust notes

- `HashMap<String, _>::get` accepts a `&str` directly, thanks to
  `String: Borrow<str>` — no allocation needed to look a key up.
- The `entry` API needs an owned key every time. With a `&str` key, `get_mut`
  first and `insert(key.to_owned(), …)` only on a miss avoids allocating when
  the key already exists.
- `slice::partition_point` returns how many leading elements satisfy a
  predicate — ideal for "the last one that is `<= t`". `count.checked_sub(1)`
  turns "zero of them" into `None`, ready for `?`.
- `new()` without arguments wants a `Default` impl too; put no bounds on `V`
  that the code doesn't need.
