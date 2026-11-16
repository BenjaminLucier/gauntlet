// Tests for Meeting Rooms II — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Peak overlap: the busiest moment is always some meeting's start time.
fn brute_min_rooms(meetings: &[(i32, i32)]) -> usize {
    meetings
        .iter()
        .map(|&(t, _)| meetings.iter().filter(|&&(s, e)| s <= t && t < e).count())
        .max()
        .unwrap_or(0)
}

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() % (hi - lo + 1) as u64) as i32
    }
}

#[test]
fn example_1() {
    assert_eq!(min_meeting_rooms(vec![(0, 30), (5, 10), (15, 20)]), 2);
}

#[test]
fn example_2() {
    assert_eq!(min_meeting_rooms(vec![(7, 10), (2, 4)]), 1);
}

#[test]
fn example_3() {
    assert_eq!(min_meeting_rooms(vec![(0, 5), (5, 10), (1, 6)]), 2);
}

#[test]
fn no_meetings() {
    assert_eq!(min_meeting_rooms(vec![]), 0);
}

#[test]
fn single_meeting() {
    assert_eq!(min_meeting_rooms(vec![(1, 2)]), 1);
}

#[test]
fn back_to_back_share_room() {
    // Trap: treating end == start as a conflict.
    assert_eq!(min_meeting_rooms(vec![(10, 15), (0, 5), (5, 10)]), 1);
}

#[test]
fn all_identical() {
    assert_eq!(min_meeting_rooms(vec![(1, 5), (1, 5), (1, 5), (1, 5)]), 4);
}

#[test]
fn nested() {
    assert_eq!(min_meeting_rooms(vec![(0, 100), (10, 90), (20, 80), (30, 70)]), 4);
}

#[test]
fn overlapping_chain_needs_two() {
    // Trap: counting all meetings that overlap *some* other one (would be 4).
    assert_eq!(min_meeting_rooms(vec![(0, 2), (1, 3), (2, 4), (3, 5)]), 2);
}

#[test]
fn room_reused_after_gap() {
    assert_eq!(min_meeting_rooms(vec![(0, 10), (1, 2), (3, 4), (5, 6), (7, 8)]), 2);
}

#[test]
fn peak_after_first_meeting_ends() {
    // Trap: only counting meetings that overlap the first one (would be 2).
    assert_eq!(min_meeting_rooms(vec![(1, 3), (2, 6), (4, 8), (5, 7)]), 3);
}

#[test]
fn reuse_earliest_free_room_not_last_used() {
    // Trap: only checking the most recently assigned room.
    assert_eq!(min_meeting_rooms(vec![(0, 2), (1, 10), (2, 3), (3, 4)]), 2);
}

#[test]
fn bounds() {
    assert_eq!(min_meeting_rooms(vec![(0, 1_000_000), (999_999, 1_000_000)]), 2);
}

#[test]
fn random_against_peak_overlap() {
    let mut rng = Rng(0x2718_2818_2845_9045);
    for _ in 0..500 {
        let n = rng.range(0, 10) as usize;
        let meetings: Vec<(i32, i32)> = (0..n)
            .map(|_| {
                let s = rng.range(0, 20);
                (s, s + rng.range(1, 6))
            })
            .collect();
        assert_eq!(
            min_meeting_rooms(meetings.clone()),
            brute_min_rooms(&meetings),
            "meetings: {meetings:?}"
        );
    }
}

#[test]
fn large_all_overlap() {
    let meetings: Vec<(i32, i32)> = (0..10_000).map(|i| (i, 1_000_000 - i)).collect();
    assert_eq!(min_meeting_rooms(meetings), 10_000);
}

#[test]
fn large_staircase() {
    // Each meeting lasts 50 time units, one starts every 10: 5 rooms.
    let meetings: Vec<(i32, i32)> = (0..10_000).rev().map(|i| (i * 10, i * 10 + 50)).collect();
    assert_eq!(min_meeting_rooms(meetings), 5);
}
