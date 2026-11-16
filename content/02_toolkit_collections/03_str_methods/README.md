## Description

Twenty-two small functions on `&str`, each a short chain of the right `str`
methods. The goal is to know the splitting, trimming and searching toolkit well
enough that you never hand-roll a loop over `chars()` to find a delimiter.

Every function says in its doc comment which method it's after. Where the
result is part of the input, it's returned as a borrowed `&str`.

## Restrictions

- **No `for`, `while` or `loop`**, and no scanning `chars()` or `bytes()` by
  hand where a `str` method does the job.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Splitting | `words`, `csv_fields`, `key_value`, `extension`, `first_fields`, `non_blank_lines`, `tokens` | [`split_whitespace`], [`split`], [`split_once`], [`rsplit_once`], [`splitn`], [`lines`], [`trim`] |
| Trimming and stripping | `unquote`, `trim_leading_zeros`, `trim_punctuation`, `strip_comment` | [`strip_prefix`], [`strip_suffix`], [`trim_start_matches`], [`trim_matches`], [`trim_end`] |
| Searching | `starts_with_digit`, `base_name`, `count_occurrences`, `vowel_offsets` | [`starts_with`], [`find`], [`rfind`], [`matches`], [`char_indices`] |
| Transforming | `censor`, `replace_first`, `shout`, `same_command`, `underline` | [`replace`], [`replacen`], [`to_uppercase`], [`eq_ignore_ascii_case`], [`repeat`] |
| Parsing | `parse_numbers`, `parse_point` | [`parse`], [patterns] |

[`split_whitespace`]: https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
[`split`]: https://doc.rust-lang.org/std/primitive.str.html#method.split
[`split_once`]: https://doc.rust-lang.org/std/primitive.str.html#method.split_once
[`rsplit_once`]: https://doc.rust-lang.org/std/primitive.str.html#method.rsplit_once
[`splitn`]: https://doc.rust-lang.org/std/primitive.str.html#method.splitn
[`lines`]: https://doc.rust-lang.org/std/primitive.str.html#method.lines
[`trim`]: https://doc.rust-lang.org/std/primitive.str.html#method.trim
[`strip_prefix`]: https://doc.rust-lang.org/std/primitive.str.html#method.strip_prefix
[`strip_suffix`]: https://doc.rust-lang.org/std/primitive.str.html#method.strip_suffix
[`trim_start_matches`]: https://doc.rust-lang.org/std/primitive.str.html#method.trim_start_matches
[`trim_matches`]: https://doc.rust-lang.org/std/primitive.str.html#method.trim_matches
[`trim_end`]: https://doc.rust-lang.org/std/primitive.str.html#method.trim_end
[`starts_with`]: https://doc.rust-lang.org/std/primitive.str.html#method.starts_with
[`find`]: https://doc.rust-lang.org/std/primitive.str.html#method.find
[`rfind`]: https://doc.rust-lang.org/std/primitive.str.html#method.rfind
[`matches`]: https://doc.rust-lang.org/std/primitive.str.html#method.matches
[`char_indices`]: https://doc.rust-lang.org/std/primitive.str.html#method.char_indices
[`replace`]: https://doc.rust-lang.org/std/primitive.str.html#method.replace
[`replacen`]: https://doc.rust-lang.org/std/primitive.str.html#method.replacen
[`to_uppercase`]: https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase
[`eq_ignore_ascii_case`]: https://doc.rust-lang.org/std/primitive.str.html#method.eq_ignore_ascii_case
[`repeat`]: https://doc.rust-lang.org/std/primitive.str.html#method.repeat
[`parse`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[patterns]: https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html

## Examples

```rust
assert_eq!(words("  the quick\tbrown\n fox "), ["the", "quick", "brown", "fox"]);
assert_eq!(key_value("expr=a=b"), Some(("expr", "a=b")));   // first `=` only
assert_eq!(extension("archive.tar.gz"), Some("gz"));         // last `.`
assert_eq!(extension(".bashrc"), None);
assert_eq!(tokens("a,b; c  d;"), ["a", "b", "c", "d"]);
assert_eq!(unquote("\"hi\""), "hi");
assert_eq!(trim_leading_zeros("0000"), "0");
assert_eq!(count_occurrences("aaaa", "aa"), 2);              // non-overlapping
assert_eq!(shout("straße"), "STRASSE!");
assert_eq!(underline("Café"), "====");                       // 4 chars, 5 bytes
assert_eq!(parse_point(" 3 , -4 "), Some((3, -4)));
```

## Rust notes

- `str::len` is the length in **bytes**. Count characters with
  `chars().count()`.
- Closures used as patterns need their parameter type spelled out:
  `|c: char| c.is_ascii_digit()`.
- `split(',')` on `""` yields one empty piece; `split_whitespace()` on `""`
  yields none.
- `to_uppercase` / `to_lowercase` follow Unicode and may change the length;
  the `*_ascii_*` variants only touch ASCII letters and never do.
- `s.parse::<i32>()` (or `parse()` with the type inferred from the return
  type) gives a `Result`; overflow is an error, not a wraparound.
