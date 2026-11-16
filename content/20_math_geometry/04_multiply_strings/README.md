## Description

Given two non-negative integers `a` and `b` written as decimal strings, return
their product, also as a decimal string.

The numbers can be hundreds of digits long, so you can't convert them to a
built-in integer type (not even `u128`), and you may not use a big-integer
library. The result has no leading zeros (zero is written as `"0"`); leading
zeros in the inputs are accepted and don't change their value.

If an input isn't a valid number, report why instead of guessing:

- an empty string is `NumberError::Empty`;
- any character other than the ASCII digits `0`–`9` (a sign, a space, a
  non-ASCII digit such as `'٣'`) is `NumberError::InvalidDigit(c)`, carrying
  the first offending character.

`a` is validated before `b`.

## Signature

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberError {
    Empty,
    InvalidDigit(char),
}

pub fn multiply(a: &str, b: &str) -> Result<String, NumberError>
```

The inputs are only read, so they are `&str`. The product is a newly built
string, so it is returned as an owned `String`. There are two distinct ways for
the input to be malformed, so failure is a small error enum rather than an
`Option` — much like `str::parse` returning `ParseIntError`.

## Examples

```rust
assert_eq!(multiply("2", "3"), Ok("6".to_string()));
assert_eq!(multiply("123", "456").as_deref(), Ok("56088"));
assert_eq!(multiply("0", "98765").as_deref(), Ok("0"));        // not "00000"
assert_eq!(multiply("007", "6").as_deref(), Ok("42"));         // leading zeros are fine
assert_eq!(multiply("12", ""), Err(NumberError::Empty));
assert_eq!(multiply("-5", "3"), Err(NumberError::InvalidDigit('-')));
```

## Constraints

- `0 <= a.len(), b.len() <= 200` (an empty input is an error)
- Inputs may contain any Unicode text; only ASCII digits are valid

## Rust notes

- `char::to_digit(10)` returns `Option<u32>`: `Some` only for `'0'..='9'`.
  `.ok_or(NumberError::InvalidDigit(c))` turns it into a `Result`, and
  `collect::<Result<Vec<_>, _>>()` stops at the first error.
- Validate by walking `chars()`, not `bytes()`: a multi-byte character must be
  reported as the `char` it is, not as a stray byte.
- `?` propagates the error out of `multiply` in one character.
- `skip_while(|&&d| d == 0)` drops leading zeros from the result; remember
  that dropping *all* of them leaves an empty string, which isn't `"0"`.
