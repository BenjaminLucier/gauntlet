// Meeting Rooms II — reference solution.
// Sort by start; a min-heap holds the end time of every room. Reuse the room
// that frees up earliest if it's free in time, else open a new one; the heap
// size is the answer. Time O(n log n), space O(n).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn min_meeting_rooms(mut meetings: Vec<(i32, i32)>) -> usize {
    meetings.sort_unstable();
    let mut room_ends: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(meetings.len());
    for (start, end) in meetings {
        if room_ends.peek().is_some_and(|&Reverse(free_at)| free_at <= start) {
            room_ends.pop();
        }
        room_ends.push(Reverse(end));
    }
    room_ends.len()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
