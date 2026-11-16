## Description

Nineteen small functions that build strings, and a few that avoid building
them. The first half is the `String` toolkit — `with_capacity`, `push`,
`insert_str`, `retain`, `write!` into a buffer, `concat`, `join`, `collect`.
The second half is [`Cow<'_, str>`][`Cow`]: return the input borrowed when
nothing needs changing, and allocate only when something does.

Every function says in its doc comment which method or idea it's after. The
`Cow` tests check *which* variant you return, not just the text:
`matches!(result, Cow::Borrowed(_))`.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Pushing, inserting and removing | `greeting`, `bracket`, `quote_in_place`, `prefix_in_place`, `append_words`, `remove_whitespace` | [`with_capacity`], [`push_str`], [`push`], [`insert`], [`insert_str`], [`extend`], [`retain`] |
| Formatting | `hex_string`, `numbered_list`, `format_point`, `right_align` | [`write!`], [`writeln!`], [`fmt::Write`], [`format!`] and its [width and precision][fmt] |
| Joining and collecting | `concat_parts`, `join_path`, `from_chars` | [`concat`], [`join`], [`FromIterator for String`][from_iter] |
| Cow | `escape_html`, `ensure_trailing_slash`, `lowercase_if_needed`, `normalize_newlines`, `escape_all`, `capitalize` | [`Cow`], [`into_owned`], [`to_mut`], [`char::to_lowercase`] |

[`with_capacity`]: https://doc.rust-lang.org/std/string/struct.String.html#method.with_capacity
[`push_str`]: https://doc.rust-lang.org/std/string/struct.String.html#method.push_str
[`push`]: https://doc.rust-lang.org/std/string/struct.String.html#method.push
[`insert`]: https://doc.rust-lang.org/std/string/struct.String.html#method.insert
[`insert_str`]: https://doc.rust-lang.org/std/string/struct.String.html#method.insert_str
[`extend`]: https://doc.rust-lang.org/std/string/struct.String.html#impl-Extend%3C%26str%3E-for-String
[`retain`]: https://doc.rust-lang.org/std/string/struct.String.html#method.retain
[`write!`]: https://doc.rust-lang.org/std/macro.write.html
[`writeln!`]: https://doc.rust-lang.org/std/macro.writeln.html
[`fmt::Write`]: https://doc.rust-lang.org/std/fmt/trait.Write.html
[`format!`]: https://doc.rust-lang.org/std/macro.format.html
[fmt]: https://doc.rust-lang.org/std/fmt/index.html#width
[`concat`]: https://doc.rust-lang.org/std/primitive.slice.html#method.concat
[`join`]: https://doc.rust-lang.org/std/primitive.slice.html#method.join
[from_iter]: https://doc.rust-lang.org/std/string/struct.String.html#impl-FromIterator%3Cchar%3E-for-String
[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[`into_owned`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#method.into_owned
[`to_mut`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#method.to_mut
[`char::to_lowercase`]: https://doc.rust-lang.org/std/primitive.char.html#method.to_lowercase

## Examples

```rust
assert_eq!(bracket("x", '«', '»'), "«x»");
assert_eq!(hex_string(&[0, 255, 16]), "00ff10");
assert_eq!(numbered_list(&["eggs", "milk"]), "1. eggs\n2. milk\n");
assert_eq!(format_point(1.0, -2.5), "(1.00, -2.50)");
assert_eq!(right_align("42", 5), "   42");

let clean = escape_html("just text");
assert!(matches!(clean, Cow::Borrowed(_)));          // nothing to escape
assert_eq!(escape_html("Tom & Jerry"), "Tom &amp; Jerry");

let mut text = Cow::Borrowed("Hello");
capitalize(&mut text);
assert!(matches!(text, Cow::Borrowed(_)));           // already capitalized
```

## Rust notes

- `insert` and `insert_str` take a **byte** index (it must be a char boundary)
  and shift everything after it: O(n).
- `write!(s, ...)` on a `String` needs `use std::fmt::Write`; without it the
  compiler looks for `io::Write` and complains. Its `Result` is always `Ok`
  for a `String` — `.expect(...)` it or `let _ =` it, but don't ignore it
  silently (it's `#[must_use]`).
- `format!("{x:.2}")` sets the precision, `format!("{s:>width$}")` right-aligns
  to a width taken from a variable; widths count `char`s.
- A `Cow<'a, str>` derefs to `&str`, so callers use it like a string either
  way, and it compares equal to `&str` and `String`.
- `char::to_lowercase` returns an iterator, not a `char`: one character can
  lower-case to several. Iterators compare with `Iterator::eq` / `ne`.
- `Cow::Borrowed("literal")` works in any `Cow<'a, str>`, because `'static`
  outlives `'a`.
