## Description

You are given an `n x n` matrix representing an image. Rotate the image by
**90 degrees clockwise**.

The rotation must be done **in place**: modify the given matrix directly rather
than building a second `n x n` matrix and copying it back.

The function is generic over the element type: pixels can be numbers, bytes,
strings — anything. Elements are only moved around, never copied, so `T` needs
no trait bounds at all.

## Signature

```rust
pub fn rotate<T>(matrix: &mut [Vec<T>])
```

The matrix is a mutable slice of rows, not `&mut Vec<Vec<T>>`: the rotation
never adds or removes rows, so it doesn't need the power to resize the outer
`Vec`. An empty slice (a `0 x 0` matrix) is left unchanged.

## Examples

```rust
let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
rotate(&mut m);
assert_eq!(m, [[7, 4, 1], [8, 5, 2], [9, 6, 3]]);

let mut m = vec![
    vec![5, 1, 9, 11],
    vec![2, 4, 8, 10],
    vec![13, 3, 6, 7],
    vec![15, 14, 12, 16],
];
rotate(&mut m);
assert_eq!(m, [[15, 13, 2, 5], [14, 3, 4, 1], [12, 6, 8, 9], [16, 7, 10, 11]]);

let mut m = vec![vec!["a", "b"], vec!["c", "d"]]; // any element type works
rotate(&mut m);
assert_eq!(m, [["c", "a"], ["d", "b"]]);
```

## Constraints

- `matrix.len() == matrix[i].len()` for every row (the matrix is square)
- `0 <= n <= 20`

## Rust notes

- With no `T: Copy` or `T: Clone` bound you can't write `let tmp = m[r][c];`.
  Move elements with `std::mem::swap` or the slice method `row.swap(i, j)`.
- `std::mem::swap(&mut m[r][c], &mut m[c][r])` doesn't compile: both borrows go
  through `m`. `m.split_at_mut(k)` hands you two disjoint mutable halves of the
  outer slice, so you can hold one row from each at the same time.
- `<[T]>::reverse` reverses a slice in place; `iter_mut` over the rows lets you
  apply it to each one.
