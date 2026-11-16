## Description

You are given an arithmetic expression in **Reverse Polish Notation** (postfix)
as a list of tokens. Each token is either an integer or one of the operators
`+`, `-`, `*`, `/`. An operator applies to the two values produced just before
it. Evaluate the expression and return the result.

- Division between two integers **truncates toward zero** (`-7 / 2 == -3`).
- All arithmetic is on `i32`.

The input is **not** guaranteed to be well formed. Report the *first* problem
met while reading the tokens left to right:

| Error | When |
|---|---|
| `RpnError::InvalidToken(token)` | a token is neither an operator nor an integer that fits in `i32` |
| `RpnError::MissingOperand` | an operator finds fewer than two values to work on |
| `RpnError::DivisionByZero` | a `/` has a right-hand operand of `0` |
| `RpnError::Overflow` | an operation's result doesn't fit in `i32` |
| `RpnError::Empty` | there are no tokens at all |
| `RpnError::TooManyOperands` | after the last token, more than one value is left |

An integer token is what `str::parse::<i32>` accepts: an optional sign followed
by decimal digits, with no spaces. `"-3"` is a number, `"-"` is an operator.

## Signature

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpnError {
    InvalidToken(String),
    MissingOperand,
    DivisionByZero,
    Overflow,
    Empty,
    TooManyOperands,
}

pub fn eval_rpn(tokens: &[&str]) -> Result<i32, RpnError>
```

Tokens are borrowed as `&[&str]`: evaluation only reads them. There are several
distinct ways to fail, so the result is a `Result` with a small error enum
rather than a panic or a magic number; `InvalidToken` carries the offending
token so the caller can show it.

## Examples

```rust
assert_eq!(eval_rpn(&["2", "1", "+", "3", "*"]), Ok(9));     // (2 + 1) * 3
assert_eq!(eval_rpn(&["4", "13", "5", "/", "+"]), Ok(6));    // 4 + 13 / 5
assert_eq!(
    eval_rpn(&["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]),
    Ok(22),                     // ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
);
assert_eq!(eval_rpn(&["-7", "2", "/"]), Ok(-3));             // truncates toward zero
assert_eq!(eval_rpn(&["1", "x", "+"]), Err(RpnError::InvalidToken("x".to_string())));
assert_eq!(eval_rpn(&["1", "+"]), Err(RpnError::MissingOperand));
assert_eq!(eval_rpn(&["1", "0", "/"]), Err(RpnError::DivisionByZero));
assert_eq!(eval_rpn(&["2147483647", "1", "+"]), Err(RpnError::Overflow));
assert_eq!(eval_rpn(&[]), Err(RpnError::Empty));
assert_eq!(eval_rpn(&["1", "2"]), Err(RpnError::TooManyOperands));
```

## Constraints

- `0 <= tokens.len() <= 10^4`

## Rust notes

- `i32::checked_add`, `checked_sub`, `checked_mul` and `checked_div` return
  `None` on overflow — including the sneaky `i32::MIN / -1`. `Option::ok_or`
  turns that into an error, and `?` returns it.
- `str::parse::<i32>()` returns a `Result`; `map_err` converts its error into
  yours.
- Rust's `/` on integers already truncates toward zero; `div_euclid` does not.
- `let … else` and slice patterns (`match stack[..] { [x] => …, [] => …, _ => … }`)
  keep the "exactly one value left" check tidy.
