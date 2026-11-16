## Description

`heights[i]` is the height of bar `i` in a histogram where every bar has width
`1` and bars stand side by side. Return the area of the **largest rectangle**
that fits entirely inside the histogram (it spans a contiguous run of bars and
its height is at most the shortest bar in that run).

An empty histogram has no rectangle, so its largest area is `0`.

## Signature

```rust
pub fn largest_rectangle_area(heights: &[u32]) -> u64
```

Heights can't be negative, so they are `u32`. The area is a height times a
width of up to `10^5` bars, which can far exceed `u32::MAX` — so it is returned
as a `u64`, and you'll need to widen before multiplying.

## Examples

```rust
// Bars 2 and 3 (heights 5 and 6) form a 2-wide rectangle of height 5.
assert_eq!(largest_rectangle_area(&[2, 1, 5, 6, 2, 3]), 10);
assert_eq!(largest_rectangle_area(&[2, 4]), 4);
assert_eq!(largest_rectangle_area(&[]), 0);
// Two bars of height u32::MAX: the area doesn't fit in a u32.
assert_eq!(largest_rectangle_area(&[u32::MAX, u32::MAX]), 2 * u64::from(u32::MAX));
```

## Constraints

- `0 <= heights.len() <= 10^5`
- `0 <= heights[i] <= u32::MAX`

## Rust notes

- `u64::from(h) * width as u64` widens losslessly before multiplying; with
  overflow checks on, multiplying two `u32`s near the top of the range panics.
- Iterating `0..=n` and treating index `n` as a height-`0` bar flushes the stack
  at the end without duplicating the popping loop; `heights.get(i).copied()`
  returns `None` past the end, and `unwrap_or(0)` turns it into that bar.
- `stack.last().map_or(0, |&l| l + 1)` reads the left edge of a rectangle
  whether or not anything is left on the stack.
