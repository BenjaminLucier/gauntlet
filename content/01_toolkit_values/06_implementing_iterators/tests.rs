// Tests for Implementing Iterators — shared by template.rs, solution.rs and your attempt.
use super::*;

// ── Countdown ──

#[test]
fn countdown_counts_down() {
    assert_eq!(Countdown::new(3).collect::<Vec<_>>(), vec![3, 2, 1]);
    assert_eq!(Countdown::new(0).next(), None);
    assert_eq!(Countdown::new(1).collect::<Vec<_>>(), vec![1]);
}

#[test]
fn countdown_runs_backwards() {
    assert_eq!(Countdown::new(4).rev().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
}

#[test]
fn countdown_ends_meet_without_repeats() {
    let mut c = Countdown::new(4);
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next_back(), Some(1));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next_back(), Some(2));
    assert_eq!(c.next(), None);
    assert_eq!(c.next_back(), None);
}

#[test]
fn countdown_reports_exact_size() {
    let mut c = Countdown::new(5);
    assert_eq!(c.size_hint(), (5, Some(5)));
    c.next();
    c.next_back();
    assert_eq!(c.size_hint(), (3, Some(3)));
    assert_eq!(c.len(), 3);
    let big = Countdown::new(u32::MAX);
    assert_eq!(big.len(), u32::MAX as usize, "no counting: size_hint is O(1)");
}

// ── Fibonacci ──

#[test]
fn fibonacci_starts_right() {
    let first: Vec<u64> = Fibonacci::new().take(10).collect();
    assert_eq!(first, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
}

#[test]
fn fibonacci_stops_before_overflow() {
    assert_eq!(Fibonacci::new().count(), 94, "F(0) through F(93) fit in a u64");
    assert_eq!(Fibonacci::default().last(), Some(12_200_160_415_121_876_738));
}

#[test]
fn fibonacci_stays_finished() {
    let mut f = Fibonacci::new();
    f.by_ref().for_each(drop);
    assert_eq!(f.next(), None);
    assert_eq!(f.next(), None, "fused: still None");
}

// ── Pairs ──

#[test]
fn pairs_in_order() {
    let got: Vec<(char, char)> = pairs(&['a', 'b', 'c']).map(|(x, y)| (*x, *y)).collect();
    assert_eq!(got, vec![('a', 'b'), ('a', 'c'), ('b', 'c')]);
}

#[test]
fn pairs_of_tiny_slices() {
    assert_eq!(pairs::<i32>(&[]).next(), None);
    assert_eq!(pairs(&[1]).next(), None);
    assert_eq!(pairs(&[1, 2]).collect::<Vec<_>>(), vec![(&1, &2)]);
}

#[test]
fn pairs_counts_and_borrows() {
    let n = 60;
    let words: Vec<String> = (0..n).map(|i| i.to_string()).collect();
    let all: Vec<(&String, &String)> = pairs(&words).collect();
    assert_eq!(all.len(), n * (n - 1) / 2);
    assert_eq!(all.last(), Some(&(&words[n - 2], &words[n - 1])));
    // The pairs borrow from `words` (no Clone needed), and outlive the iterator.
    let first = {
        let mut it = pairs(&words);
        it.next()
    };
    assert_eq!(first, Some((&words[0], &words[1])));
}

// ── ChunksOf ──

#[test]
fn chunks_of_with_remainder() {
    let xs = [1, 2, 3, 4, 5];
    let got: Vec<&[i32]> = chunks_of(&xs, 2).collect();
    assert_eq!(got, vec![&[1, 2][..], &[3, 4], &[5]]);
    let exact: Vec<&[i32]> = chunks_of(&xs[..4], 2).collect();
    assert_eq!(exact, vec![&[1, 2][..], &[3, 4]]);
}

#[test]
fn chunks_of_edge_cases() {
    assert_eq!(chunks_of::<u8>(&[], 3).next(), None);
    assert_eq!(chunks_of(&[1, 2], 10).collect::<Vec<_>>(), vec![&[1, 2][..]]);
}

#[test]
fn chunks_of_exact_size() {
    let xs: Vec<u32> = (0..10).collect();
    let mut it = chunks_of(&xs, 3);
    assert_eq!(it.len(), 4);
    it.next();
    assert_eq!(it.size_hint(), (3, Some(3)));
    it.next();
    it.next();
    it.next();
    assert_eq!(it.len(), 0);
}

#[test]
#[should_panic(expected = "chunk size must be non-zero")]
fn chunks_of_zero_panics() {
    let _ = chunks_of(&[1, 2, 3], 0);
}

// ── Dedup ──

#[test]
fn dedup_skips_adjacent_repeats() {
    assert_eq!(dedup([1, 1, 2, 1, 1, 1, 3].into_iter()).collect::<Vec<_>>(), vec![1, 2, 1, 3]);
    assert_eq!(dedup(std::iter::empty::<i32>()).next(), None);
    let words = ["a", "a", "b", "b", "a"].map(String::from);
    assert_eq!(dedup(words.into_iter()).collect::<Vec<_>>(), ["a", "b", "a"]);
}

#[test]
fn dedup_is_lazy() {
    let endless = [1, 1, 2].into_iter().cycle();
    assert_eq!(dedup(endless).take(5).collect::<Vec<_>>(), vec![1, 2, 1, 2, 1]);
}

// ── Closures as iterators ──

#[test]
fn digits_lsb_from_fn() {
    assert_eq!(digits_lsb(1234).collect::<Vec<_>>(), vec![4, 3, 2, 1]);
    assert_eq!(digits_lsb(0).collect::<Vec<_>>(), vec![0]);
    assert_eq!(digits_lsb(100).collect::<Vec<_>>(), vec![0, 0, 1]);
    assert_eq!(digits_lsb(u64::MAX).count(), 20);
}

#[test]
fn powers_of_stops_at_overflow() {
    assert_eq!(powers_of(3).take(5).collect::<Vec<_>>(), vec![1, 3, 9, 27, 81]);
    assert_eq!(powers_of(2).count(), 64);
    assert_eq!(powers_of(2).last(), Some(1 << 63));
    assert_eq!(powers_of(10).last(), Some(10_000_000_000_000_000_000));
}

// ── Stack ──

#[test]
fn stack_iterates_top_first() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.iter().collect::<Vec<_>>(), vec![&3, &2, &1]);
    let mut seen = Vec::new();
    for x in &s {
        seen.push(*x);
    }
    assert_eq!(seen, vec![3, 2, 1]);
    assert_eq!(s.len(), 3, "borrowing iteration leaves the stack alone");
    assert_eq!(s.into_iter().collect::<Vec<_>>(), vec![3, 2, 1]);
}

#[test]
fn stack_collects_and_extends() {
    let mut s: Stack<String> = ["a", "b"].into_iter().map(String::from).collect();
    s.extend(["c".to_string()]);
    s.extend(Vec::<String>::new());
    let mut moved = Vec::new();
    for item in s {
        moved.push(item); // owned Strings, moved out
    }
    assert_eq!(moved, ["c", "b", "a"]);
}

#[test]
fn stack_round_trip_reverses() {
    let s: Stack<i32> = (1..=5).collect();
    let flipped: Stack<i32> = s.into_iter().collect();
    assert_eq!(flipped.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
    let empty: Stack<i32> = std::iter::empty().collect();
    assert!(empty.iter().next().is_none());
}
