## Description

Find every occurrence of a pattern in a text in `O(n + m)` time with the
**Knuth–Morris–Pratt** algorithm.

The naive search tries every start position and compares up to `m`
characters at each: `O(n·m)` on inputs like `"aaa…ab"` in `"aaaa…a"`. KMP never
moves backwards in the text. It precomputes the **prefix function** (or
*failure table*) of the pattern:

> `pi[i]` = the length of the longest *proper* prefix of `pattern[..=i]` that
> is also a suffix of `pattern[..=i]`.

so that after a mismatch it knows how much of what it has already matched can
be reused.

Implement:

- `prefix_function(pattern)` — the table above, one entry per element.
- `find_all_in(text, pattern)` — the start index of every match, overlapping
  ones included, in increasing order, for slices of any `T: Eq`.
- `find_all(text, pattern)` — the same for `&str`, as **byte offsets**.
- `find_first_in(text, pattern)` — the first match only, without scanning
  the rest of the text, for slices of any `T: Eq`.
- `find_first(text, pattern)` — the same for `&str`, as a byte offset.

**The empty pattern** matches at every position: `find_all_in(text, &[])` is
`0..=text.len()` and `find_first_in(text, &[])` / `find_first(text, "")` are
`Some(0)` — the same convention
as `str::find("")`. A pattern longer than the text has no matches.

`find_all` and `find_first` work on the strings' **bytes**, and their results
are byte offsets into `text`. Because UTF-8 is self-synchronising, a match of a
valid `&str` pattern in a valid `&str` text always starts and ends on char
boundaries, so `&text[i..i + pattern.len()]` is always a valid slice.

## Restrictions

- No `str::find`, `contains`, `matches`, `match_indices`, `split`, no
  `slice::windows` / `starts_with` / `ends_with`-based search: the matching
  must be KMP's.
- In particular, `find_all` and `find_first` on `&str` must use the prefix
  function as well. This part is on the honor system: comparing
  `text[i..i + m] == pattern` at every start is `O(n·m)` but passes the
  `&str` tests, as nothing counts comparisons on bytes. The comparison-count
  tests (`comparison_count_is_linear`,
  `prefix_function_comparison_count_is_linear`,
  `find_first_in_stops_at_the_first_match`) check the generic versions,
  whose element type counts every `==`.

## Signature

```rust
pub fn prefix_function<T: Eq>(pattern: &[T]) -> Vec<usize>;
pub fn find_all_in<T: Eq>(text: &[T], pattern: &[T]) -> Vec<usize>;
pub fn find_all(text: &str, pattern: &str) -> Vec<usize>;
pub fn find_first_in<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize>;
pub fn find_first(text: &str, pattern: &str) -> Option<usize>;
```

The algorithm only ever compares elements for equality, so the slice version
is generic over `T: Eq`: it works on bytes, `char`s, integers or tokens alike.
Positions are `usize` indices; "no match" is an empty `Vec` or `None`.

## Examples

```rust
assert_eq!(prefix_function(b"abacaba"), [0, 0, 1, 0, 1, 2, 3]);
assert_eq!(prefix_function(b"aabaaab"), [0, 1, 0, 1, 2, 2, 3]);

assert_eq!(find_all("abababa", "aba"), [0, 2, 4]);      // overlapping
assert_eq!(find_first("abababa", "bab"), Some(1));
assert_eq!(find_first("abc", "abcd"), None);            // longer than text
assert_eq!(find_all("abc", ""), [0, 1, 2, 3]);           // empty pattern

assert_eq!(find_all("héllo, héllo", "llo"), [3, 11]);    // byte offsets: 'é' is 2 bytes

assert_eq!(find_all_in(&[1, 2, 1, 2, 1], &[1, 2, 1]), [0, 2]);
assert_eq!(find_first_in(&[3, 1, 2, 1, 2], &[1, 2]), Some(1));
```

## Constraints

- `text` and `pattern` up to `2 * 10^5` elements each.
- Must run in `O(n + m)`. The generic functions are checked by counting
  element comparisons; see Restrictions for the `&str` ones.

## Rust notes

- `prefix_function` and `find_all_in` share one inner loop shape:
  `while k > 0 && a != b { k = pi[k - 1]; }`. Watch the `k - 1` — with
  overflow checks on, `0 - 1` on a `usize` panics.
- `str::as_bytes` gives the `&[u8]` view of a string for free, so `find_all`
  can be a one-liner over `find_all_in`.
- A private iterator (a struct holding `text`, `pattern`, `pi`, the text
  position and `k`) lets the `find_all` variants be `.collect()` and the
  `find_first` variants be `.next()` over the same code.
- `b"abacaba"` is a `&[u8; 7]`, which coerces to `&[u8]` at the call.
