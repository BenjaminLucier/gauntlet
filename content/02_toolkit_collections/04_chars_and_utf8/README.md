## Description

Twenty-one small functions about what a Rust string really is: UTF-8 bytes,
where a `char` is a Unicode scalar value taking 1 to 4 of them. The goal is to
know which length you're looking at, how to slice by character without
panicking, and how to move between `char`, `u32`, `u8` and bytes.

Every function says in its doc comment which method it's after. The tests use
`é` (2 bytes), `日本` (3 bytes each) and `🦀` (4 bytes): code that assumes one
byte per character fails them.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Lengths | `byte_and_char_len`, `utf8_width`, `is_plain_ascii` | [`str::len`], [`chars`], [`len_utf8`], [`is_ascii`] |
| Indexing by character | `char_at`, `char_prefix`, `char_slice`, `byte_slice`, `truncate_bytes` | [`char_indices`], [`str::get`], [`floor_char_boundary`] |
| Classifying and converting chars | `digit_value`, `digit_char`, `sum_digits`, `alnum_only`, `ascii_counts`, `rot13` | [`to_digit`], [`from_digit`], [`is_alphanumeric`], [`is_ascii_alphabetic`], [`is_ascii_digit`], [`is_ascii_whitespace`] |
| Bytes and code points | `utf8_bytes`, `decode`, `decode_lossy`, `code_points`, `from_code_point`, `latin1_to_string`, `reverse_chars` | [`encode_utf8`], [`str::from_utf8`], [`from_utf8_lossy`], [`From<char> for u32`][u32_from], [`from_u32`], [`u8 as char`][as], [`rev`] |

[`str::len`]: https://doc.rust-lang.org/std/primitive.str.html#method.len
[`chars`]: https://doc.rust-lang.org/std/primitive.str.html#method.chars
[`len_utf8`]: https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8
[`is_ascii`]: https://doc.rust-lang.org/std/primitive.str.html#method.is_ascii
[`char_indices`]: https://doc.rust-lang.org/std/primitive.str.html#method.char_indices
[`str::get`]: https://doc.rust-lang.org/std/primitive.str.html#method.get
[`floor_char_boundary`]: https://doc.rust-lang.org/std/primitive.str.html#method.floor_char_boundary
[`to_digit`]: https://doc.rust-lang.org/std/primitive.char.html#method.to_digit
[`from_digit`]: https://doc.rust-lang.org/std/primitive.char.html#method.from_digit
[`is_alphanumeric`]: https://doc.rust-lang.org/std/primitive.char.html#method.is_alphanumeric
[`is_ascii_alphabetic`]: https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_alphabetic
[`is_ascii_digit`]: https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_digit
[`is_ascii_whitespace`]: https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_whitespace
[`encode_utf8`]: https://doc.rust-lang.org/std/primitive.char.html#method.encode_utf8
[`str::from_utf8`]: https://doc.rust-lang.org/std/str/fn.from_utf8.html
[`from_utf8_lossy`]: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy
[`from_u32`]: https://doc.rust-lang.org/std/primitive.char.html#method.from_u32
[u32_from]: https://doc.rust-lang.org/std/primitive.u32.html#impl-From%3Cchar%3E-for-u32
[as]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
[`rev`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev

## Examples

```rust
assert_eq!(byte_and_char_len("日本"), (6, 2));
assert_eq!(utf8_width('🦀'), 4);

assert_eq!(char_slice("héllo", 1, 3), Some("él"));    // characters 1 and 2
assert_eq!(byte_slice("héllo", 0, 2), None);          // byte 2 is inside é
assert_eq!(truncate_bytes("日本語", 4), "日");         // 4 bytes would split 本

assert_eq!(digit_value('f', 16), Some(15));
assert_eq!(alnum_only("日本 é-2!"), "日本é2");
assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");

assert_eq!(utf8_bytes('é'), [0xC3, 0xA9]);
assert_eq!(decode_lossy(&[b'a', 0xFF, b'b']), "a\u{FFFD}b");
assert_eq!(from_code_point(0xD800), None);            // a surrogate
assert_eq!(latin1_to_string(&[0x63, 0x61, 0x66, 0xE9]), "café");
```

## Rust notes

- `&s[a..b]` **panics** if `a` or `b` isn't on a character boundary;
  `s.get(a..b)` returns `None`. Offsets from `find`, `char_indices` and
  `floor_char_boundary` are always safe; `is_char_boundary(i)` tells you
  whether a given offset is.
- `chars().nth(n)` is O(n): UTF-8 has no random access by character. If you
  need many character positions, collect `char_indices()` once.
- `b as char` treats a byte as a code point (Latin-1), which is *not* the same
  as decoding UTF-8: `0xC3 as char` is `'Ã'`, not half of `'é'`.
- `from_utf8` borrows and can fail; `from_utf8_lossy` never fails and returns a
  `Cow<str>` — borrowed when the input was already valid.
- A `char` is a code point, not a user-perceived character: `"e\u{301}"` (e +
  combining accent) is two `char`s, and reversing by `char` moves the accent.
