## Description

`#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]` compares fields in
declaration order. Often that's right; sometimes it's subtly wrong — a name that
should ignore (ASCII) case, a version where `1.10` is newer than `1.9`, a leaderboard
sorted by score *descending*. Here you write those impls by hand, plus the
`Ordering` toolkit: `then_with`, `reverse`, `cmp::Reverse`, `total_cmp`.

The types are given; only the bodies are `todo!()`. Where you implement `Ord`,
the matching `PartialOrd` is already written as `Some(self.cmp(other))` — the
canonical form clippy expects.

The one rule that ties it together: **if `a == b` then `hash(a) == hash(b)`**,
and `cmp` must return `Equal` exactly when `eq` returns `true`. Break it, and
`HashSet`, `HashMap` and `BTreeSet` silently misbehave.

## Restrictions

- **No `match` on an `Ordering`** — use `then`, `then_with`, `reverse`,
  `is_lt`, `is_eq` and friends.

## The drills

| Group | Items | Traits and methods |
|---|---|---|
| Case-insensitive string | `CaseInsensitive`: `PartialEq`, `Hash`, `Ord`; `distinct_names` | [`eq_ignore_ascii_case`], [`Hash`], [`Iterator::cmp`] |
| Versions | `Version::trimmed`; `Version`: `PartialEq`, `Hash`, `Ord`; `newest`, `strictly_ascending` | [`rposition`], [`Ord`], [`Ord::max`], [`windows`], [`Ordering::is_lt`] |
| Several keys | `Player: Ord`, `by_len_then_alpha`, `most_urgent`, `ranked_by_score` | [`then`], [`then_with`], [`reverse`], [`is_sorted_by_key`] |
| `Reverse` and `*_by_key` | `sort_longest_first`, `longest`, `closer_to_zero` | [`cmp::Reverse`], [`sort_by_key`], [`min_by_key`], [`cmp::min_by_key`] |
| Floats | `max_f64`, `sort_floats`, `Score`: `PartialEq`, `Ord` | [`f64::total_cmp`], [`max_by`], [`sort_by`], [`Ordering::is_eq`] |

[`eq_ignore_ascii_case`]: https://doc.rust-lang.org/std/primitive.str.html#method.eq_ignore_ascii_case
[`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
[`Iterator::cmp`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cmp
[`Ord`]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[`Ord::max`]: https://doc.rust-lang.org/std/cmp/trait.Ord.html#method.max
[`rposition`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rposition
[`windows`]: https://doc.rust-lang.org/std/primitive.slice.html#method.windows
[`Ordering::is_lt`]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.is_lt
[`Ordering::is_eq`]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.is_eq
[`then`]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then
[`then_with`]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then_with
[`reverse`]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.reverse
[`sort_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
[`is_sorted_by_key`]: https://doc.rust-lang.org/std/primitive.slice.html#method.is_sorted_by_key
[`cmp::Reverse`]: https://doc.rust-lang.org/std/cmp/struct.Reverse.html
[`sort_by_key`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key
[`min_by_key`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min_by_key
[`cmp::min_by_key`]: https://doc.rust-lang.org/std/cmp/fn.min_by_key.html
[`f64::total_cmp`]: https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp
[`max_by`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by

## Examples

```rust
let ci = |s: &str| CaseInsensitive(s.to_string());
assert_eq!(ci("Ann"), ci("ANN"));
assert_eq!(distinct_names(&["Ann", "ann", "Bob"]), 2);    // HashSet agrees with ==

let v = |s: &str| s.parse::<Version>().unwrap();
assert!(v("1.10") > v("1.9"));                            // not string order
assert_eq!(v("1.2"), v("1.2.0"));                         // trailing zeros ignored

let p = |name: &str, score, time| Player { name: name.into(), score, time };
assert!(p("b", 90, 10) < p("a", 90, 50));                 // same score: faster first

assert_eq!(longest(&["ab", "xyz", "uvw"]), Some("xyz"));  // first of the longest
assert_eq!(closer_to_zero(-3, 3), -3);                    // tie: the first argument
assert!(max_f64(&[1.0, f64::NAN]).unwrap().is_nan());     // NaN tops the total order
```

## Rust notes

- `HashSet`/`HashMap` find an entry by hash first, then `==`. A `Hash` that
  looks at more than `Eq` does (e.g. derived `Hash` with a case-insensitive
  `PartialEq`) puts "equal" keys in different buckets. Clippy's
  `derived_hash_with_manual_eq` exists for this reason.
- `cmp` must agree with `eq`: `BTreeSet` only uses `cmp`, `HashSet` only `eq` +
  `hash`. If `1.2` and `1.2.0` are equal to one, they must be to the other.
- `f64` is only `PartialOrd` (`NaN` isn't comparable), so `sort()` and `max()`
  don't compile on it. `f64::total_cmp` is a real total order:
  `-NaN < -∞ < … < -0.0 < 0.0 < … < ∞ < NaN`.
- `Ord::max(a, b)` returns `b` on a tie, `max_by_key` the last maximum,
  `min_by_key` the first minimum. Stable sorts keep ties in input order.
