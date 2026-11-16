## Description

So far you've *used* iterators. Here you build them: a handful of small types
that implement [`Iterator`] and its companion traits, two iterators made from
closures, and a collection that plugs into `for` loops and `collect()`.

| Item | What you write | Traits and APIs |
|---|---|---|
| `Countdown` | fields, `new`, `next`, `size_hint`, `next_back` | [`Iterator`], [`size_hint`], [`DoubleEndedIterator`], [`ExactSizeIterator`] |
| `Fibonacci` | fields, `new`, `next` | [`checked_add`], [`FusedIterator`] |
| `Pairs<'a, T>` | fields, `pairs`, `next` | an iterator that borrows a slice |
| `ChunksOf<'a, T>` | fields, `chunks_of`, `next`, `size_hint` | yielding `&'a [T]`, [`split_at`] |
| `Dedup<I>` | fields, `dedup`, `next` | a generic adapter, [`Peekable::next_if_eq`] |
| `digits_lsb`, `powers_of` | the function bodies | [`iter::from_fn`], [`iter::successors`] |
| `Stack<T>` | `iter`, both `IntoIterator`s, `FromIterator`, `Extend` | [`IntoIterator`], [`FromIterator`], [`Extend`] |

Each type's doc comment in the template spells out exactly what it yields.
Fields marked `// TODO` are yours to design.

[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[`size_hint`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint
[`DoubleEndedIterator`]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html
[`ExactSizeIterator`]: https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html
[`checked_add`]: https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add
[`FusedIterator`]: https://doc.rust-lang.org/std/iter/trait.FusedIterator.html
[`split_at`]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at
[`Peekable::next_if_eq`]: https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.next_if_eq
[`iter::from_fn`]: https://doc.rust-lang.org/std/iter/fn.from_fn.html
[`iter::successors`]: https://doc.rust-lang.org/std/iter/fn.successors.html
[`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[`FromIterator`]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html
[`Extend`]: https://doc.rust-lang.org/std/iter/trait.Extend.html

## Restrictions

- `Countdown` may not wrap a `Range` (`(1..=n).rev()` would do it all for you).
- `ChunksOf` may not use `slice::chunks`.
- `Dedup` may not collect its input: it has to work on an endless iterator.

## Signature

```rust
pub struct Countdown { /* yours */ }
impl Countdown { pub fn new(from: u32) -> Self }
impl Iterator for Countdown { type Item = u32; /* next, size_hint */ }
impl DoubleEndedIterator for Countdown { /* next_back */ }
impl ExactSizeIterator for Countdown {}

pub struct Fibonacci { /* yours */ }        // + Default, FusedIterator
pub fn pairs<T>(items: &[T]) -> Pairs<'_, T>;              // Item = (&'a T, &'a T)
pub fn chunks_of<T>(items: &[T], size: usize) -> ChunksOf<'_, T>; // Item = &'a [T]
pub fn dedup<I: Iterator>(iter: I) -> Dedup<I>;            // Item = I::Item
pub fn digits_lsb(n: u64) -> Box<dyn Iterator<Item = u64>>;
pub fn powers_of(base: u64) -> Box<dyn Iterator<Item = u64>>;

pub struct Stack<T> { items: Vec<T> }       // given, with new/push/len/is_empty
impl<T> Stack<T> { pub fn iter(&self) -> Rev<slice::Iter<'_, T>> }
impl<T> IntoIterator for Stack<T>      { type IntoIter = Rev<vec::IntoIter<T>>; .. }
impl<'a, T> IntoIterator for &'a Stack<T> { type IntoIter = Rev<slice::Iter<'a, T>>; .. }
impl<T> FromIterator<T> for Stack<T>;
impl<T> Extend<T> for Stack<T>;
```

`Pairs` and `ChunksOf` yield references with the slice's lifetime `'a`, not
the iterator's, so the items outlive the iterator. `digits_lsb` and
`powers_of` return a `Box<dyn Iterator>` because the closure inside has a type
you can't name.

## Examples

```rust
assert_eq!(Countdown::new(3).collect::<Vec<_>>(), vec![3, 2, 1]);
assert_eq!(Countdown::new(3).rev().collect::<Vec<_>>(), vec![1, 2, 3]);
assert_eq!(Countdown::new(7).len(), 7);

assert_eq!(Fibonacci::new().take(6).collect::<Vec<_>>(), vec![0, 1, 1, 2, 3, 5]);
assert_eq!(Fibonacci::new().count(), 94);          // then u64 overflows

assert_eq!(pairs(&[1, 2, 3]).count(), 3);           // (1,2) (1,3) (2,3)
assert_eq!(chunks_of(&[1, 2, 3, 4, 5], 2).collect::<Vec<_>>(), vec![&[1, 2][..], &[3, 4], &[5]]);
assert_eq!(dedup([1, 1, 2, 1].into_iter()).collect::<Vec<_>>(), vec![1, 2, 1]);
assert_eq!(digits_lsb(1234).collect::<Vec<_>>(), vec![4, 3, 2, 1]);
assert_eq!(powers_of(2).count(), 64);              // 2^0 ..= 2^63

let s: Stack<i32> = (1..=3).collect();             // 3 is on top
assert_eq!(s.iter().collect::<Vec<_>>(), vec![&3, &2, &1]);
```

## Rust notes

- `size_hint` defaults to `(0, None)`. Override it when you know better:
  `collect` uses it to pre-allocate, and `ExactSizeIterator::len` is built on
  it (and asserts that both bounds agree).
- `FusedIterator` and `ExactSizeIterator` have no methods to write; they are
  promises. Breaking them isn't undefined behaviour, but other code will
  rely on them.
- A struct that holds a `Peekable<I>` needs `I: Iterator` on the struct itself,
  because `Peekable` has that bound.
- `for x in &stack` calls `IntoIterator for &Stack<T>`; `for x in stack` moves
  the stack. Implement both and `iter()` for the full std feel.
