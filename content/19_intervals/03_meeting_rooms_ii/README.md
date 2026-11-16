## Description

Given a list of meetings `(start, end)`, return the **minimum number of
conference rooms** needed to hold all of them.

A meeting `(s, e)` occupies the half-open time `s..e`: a room freed by a
meeting ending at `5` can host a meeting starting at `5`. No meetings need no
rooms.

## Signature

```rust
pub fn min_meeting_rooms(meetings: Vec<(i32, i32)>) -> usize
```

The answer is a count, so it is a `usize`. The function sorts the meetings, so
it takes the `Vec` by value and sorts it in place.

## Examples

```rust
// (0, 30) in one room; (5, 10) then (15, 20) in another.
assert_eq!(min_meeting_rooms(vec![(0, 30), (5, 10), (15, 20)]), 2);
assert_eq!(min_meeting_rooms(vec![(7, 10), (2, 4)]), 1);
// (0, 5) and (5, 10) can share a room.
assert_eq!(min_meeting_rooms(vec![(0, 5), (5, 10), (1, 6)]), 2);
assert_eq!(min_meeting_rooms(vec![]), 0);
```

## Constraints

- `0 <= meetings.len() <= 10^4`
- `0 <= start < end <= 10^6`

## Rust notes

- `BinaryHeap` is a max-heap; wrap items in `std::cmp::Reverse` to get a
  min-heap: `BinaryHeap<Reverse<i32>>` keeps the earliest end time on top.
- `heap.peek().is_some_and(|&Reverse(end)| end <= start)` checks the top
  without removing it; `BinaryHeap::peek_mut` lets you replace it in place.
- The heap's final `len()` can be the answer — think about why it never
  needs to shrink.
- Alternatively, sort the starts and the ends into two separate `Vec`s and
  sweep them with two indices.
