## Description

Encode a list of strings into a **single string**, and decode that string back
into the original list.

`decode(&encode(strs))` must give back exactly `strs` — same strings, same
order, same count — for **any** input. The strings may contain any characters
at all: digits, `#`, commas, spaces, newlines, NUL, non-ASCII Unicode, and they
may be empty. An empty list must round-trip too (and must not come back as
`[""]`).

Use this **length-prefixed** format: for each string, write its length in
**bytes** as a decimal number, then `#`, then the string itself. So
`["ab", "", "#1"]` encodes to `"2#ab0#2##1"`. `encode` writes lengths without
leading zeros.

`decode` must also cope with input that was *not* produced by `encode`. It
reads entries from the start of the input until the input is used up, and
reports the first problem it finds:

| Error | When |
|---|---|
| `MissingSeparator` | no `#` follows the current position |
| `InvalidLength` | the text before that `#` is empty, contains anything but ASCII digits `0`–`9`, or doesn't fit in a `usize` (leading zeros are fine) |
| `Truncated` | fewer bytes remain after the `#` than the length says |
| `NotCharBoundary` | the length ends in the middle of a multi-byte character |

## Signature

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    MissingSeparator,
    InvalidLength,
    Truncated,
    NotCharBoundary,
}

pub fn encode<S: AsRef<str>>(strs: &[S]) -> String
pub fn decode(encoded: &str) -> Result<Vec<&str>, DecodeError>
```

`encode` accepts any slice of string-like things — `&[&str]`, `&[String]`,
`&Vec<String>` — via `AsRef<str>`. `decode` doesn't copy anything: every piece
of text it returns is a slice of `encoded`, so it returns `Vec<&str>` borrowing
from the input. Decoding can fail in several distinct ways, so it returns a
`Result` with an error enum rather than panicking.

## Examples

```rust
assert_eq!(encode(&["ab", "", "#1"]), "2#ab0#2##1");
assert_eq!(decode("2#ab0#2##1"), Ok(vec!["ab", "", "#1"]));

let words = ["we", "say", ":", "yes"];
assert_eq!(decode(&encode(&words)), Ok(words.to_vec()));

let owned = vec![String::from("日本"), String::from("🦀")];
assert_eq!(encode(&owned), "6#日本4#🦀");      // lengths are in bytes

assert_eq!(encode::<&str>(&[]), "");
assert_eq!(decode(""), Ok(vec![]));             // empty list, not [""]

assert_eq!(decode("3#ab"), Err(DecodeError::Truncated));
assert_eq!(decode("12"), Err(DecodeError::MissingSeparator));
assert_eq!(decode("x#"), Err(DecodeError::InvalidLength));
assert_eq!(decode("1#é"), Err(DecodeError::NotCharBoundary));
```

## Constraints

- `0 <= strs.len() <= 2 * 10^4`
- `0 <= strs[i].len() < 1000`
- `strs[i]` may contain any Unicode characters.

## Rust notes

- `str::len` counts **bytes**, and `&s[a..b]` slices by byte offset — but
  panics if an offset isn't on a char boundary. `str::get(a..b)` returns
  `None` instead, and `str::is_char_boundary` lets you check first.
- `str::find('#')` / `split_once('#')` locate the separator.
- `"+5".parse::<usize>()` succeeds! Check the digits yourself
  (`bytes().all(|b| b.is_ascii_digit())`) before parsing.
- `write!(out, "{}#", s.len())` (with `std::fmt::Write`) appends a number to a
  `String` without a temporary allocation; `push_str` appends the rest.
- `?` and `ok_or(DecodeError::…)` turn `Option`s and parse failures into
  early returns.
