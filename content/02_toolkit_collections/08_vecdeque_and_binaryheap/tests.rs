// Tests for VecDeque and BinaryHeap — shared by template.rs, solution.rs and your attempt.
use super::*;

/// A deque whose storage wraps around the end of its buffer, so `as_slices`
/// returns two non-empty halves.
fn wrapped(xs: &[i64]) -> VecDeque<i64> {
    let mut d = VecDeque::with_capacity(xs.len());
    let mid = xs.len() / 2;
    for &x in &xs[mid..] {
        d.push_back(x);
    }
    for &x in xs[..mid].iter().rev() {
        d.push_front(x);
    }
    d
}

// ── VecDeque: both ends ──

#[test]
fn push_both_orders_front_reversed() {
    assert_eq!(push_both(&[1, 2, 3], &[4, 5]), [3, 2, 1, 4, 5]);
    assert_eq!(push_both(&[], &[7]), [7]);
    assert!(push_both(&[], &[]).is_empty());
}

#[test]
fn is_palindrome_pops_both_ends() {
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("abba"));
    assert!(is_palindrome(""));
    assert!(is_palindrome("x"));
    assert!(!is_palindrome("ab"));
    assert!(is_palindrome("été"), "by char, not by byte");
    assert!(!is_palindrome("éa"));
}

#[test]
fn rotate_both_directions() {
    let mut d: VecDeque<i32> = (1..=5).collect();
    rotate(&mut d, 2);
    assert_eq!(d, [4, 5, 1, 2, 3]);
    rotate(&mut d, -2);
    assert_eq!(d, [1, 2, 3, 4, 5]);
    rotate(&mut d, -1);
    assert_eq!(d, [2, 3, 4, 5, 1]);
}

#[test]
fn rotate_more_than_len_wraps() {
    let mut d: VecDeque<i32> = (1..=3).collect();
    rotate(&mut d, 7);
    assert_eq!(d, [3, 1, 2], "7 steps on 3 elements is 1 step");
    rotate(&mut d, isize::MIN);
    assert_eq!(d.len(), 3);
    let mut empty = VecDeque::new();
    rotate(&mut empty, 5);
    assert!(empty.is_empty());
}

#[test]
fn elimination_order_counts_out() {
    assert_eq!(elimination_order(7, 3), [3, 6, 2, 7, 5, 1, 4]);
    assert_eq!(elimination_order(4, 1), [1, 2, 3, 4]);
    assert_eq!(elimination_order(1, 10), [1]);
    assert_eq!(elimination_order(0, 2), Vec::<usize>::new());
}

// ── VecDeque: sliding buffers ──

#[test]
fn last_k_keeps_the_tail() {
    assert_eq!(last_k(&[1, 2, 3, 4, 5], 3), [3, 4, 5]);
    assert_eq!(last_k(&[1, 2], 5), [1, 2]);
    assert_eq!(last_k(&[1, 2, 3], 0), Vec::<i32>::new());
}

#[test]
fn recent_counts_slides() {
    assert_eq!(recent_counts(&[1, 100, 3001, 3002], 3000), [1, 2, 3, 3]);
    assert_eq!(recent_counts(&[5, 5, 6, 20], 0), [1, 2, 1, 1]);
    assert_eq!(recent_counts(&[], 10), Vec::<usize>::new());
}

#[test]
fn recent_counts_near_zero_does_not_underflow() {
    assert_eq!(recent_counts(&[0, 1, 2], u32::MAX), [1, 2, 3]);
}

// ── VecDeque: as slices ──

#[test]
fn sort_deque_handles_a_wrapped_buffer() {
    let mut d = VecDeque::new();
    for x in [5, 1, 4] {
        d.push_back(x);
    }
    for x in [9, -2, 3] {
        d.push_front(x);
    }
    sort_deque(&mut d);
    assert_eq!(d, [-2, 1, 3, 4, 5, 9]);
}

#[test]
fn total_reads_both_slices() {
    let d = wrapped(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(total(&d), 21);
    assert_eq!(total(&VecDeque::new()), 0);
}

#[test]
fn middle_takes_a_range() {
    let d: VecDeque<char> = "abcdef".chars().collect();
    assert_eq!(middle(&d, 1, 4), "bcd");
    assert_eq!(middle(&d, 3, 3), "");
    assert_eq!(middle(&d, 0, 6), "abcdef");
}

#[test]
#[should_panic(expected = "out of range")]
fn middle_past_the_end_panics() {
    let d: VecDeque<char> = "abc".chars().collect();
    middle(&d, 1, 4);
}

// ── BinaryHeap ──

#[test]
fn peek_max_does_not_remove() {
    let heap = BinaryHeap::from(vec![3, 9, 1]);
    assert_eq!(peek_max(&heap), Some(9));
    assert_eq!(heap.len(), 3);
    assert_eq!(peek_max(&BinaryHeap::new()), None);
}

#[test]
fn largest_k_descending() {
    assert_eq!(largest_k(&[5, 1, 9, 3, 9, 7], 3), [9, 9, 7]);
    assert_eq!(largest_k(&[2, 1], 5), [2, 1]);
    assert_eq!(largest_k(&[], 2), Vec::<i32>::new());
}

#[test]
fn smallest_k_ascending() {
    assert_eq!(smallest_k(&[5, 1, 9, -3, 1, 7], 3), [-3, 1, 1]);
    assert_eq!(smallest_k(&[4], 0), Vec::<i32>::new());
}

#[test]
fn heap_sorted_ascends() {
    assert_eq!(heap_sorted(vec![3, -1, 2, 2, 0]), [-1, 0, 2, 2, 3]);
    assert_eq!(heap_sorted(vec![]), Vec::<i32>::new());
}

#[test]
fn merge_into_moves_everything() {
    let mut heap = BinaryHeap::from(vec![5, 1]);
    let mut other = BinaryHeap::from(vec![9, 1, -4]);
    merge_into(&mut heap, &mut other);
    assert!(other.is_empty(), "append leaves the other heap empty");
    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.into_sorted_vec(), [-4, 1, 1, 5, 9], "duplicates are kept");
}

#[test]
fn merge_into_with_empty_heaps() {
    let mut heap = BinaryHeap::new();
    let mut other = BinaryHeap::from(vec![3]);
    merge_into(&mut heap, &mut other);
    assert_eq!(heap.into_vec(), [3]);
    let mut heap = BinaryHeap::from(vec![2]);
    merge_into(&mut heap, &mut BinaryHeap::new());
    assert_eq!(heap.into_vec(), [2]);
}

#[test]
fn drain_total_empties_but_keeps_capacity() {
    let mut heap = BinaryHeap::with_capacity(64);
    heap.extend([10, 20, 30, u64::from(u32::MAX)]);
    let capacity = heap.capacity();
    assert_eq!(drain_total(&mut heap), 60 + u64::from(u32::MAX));
    assert!(heap.is_empty());
    assert_eq!(heap.capacity(), capacity, "drain keeps the allocation");
    assert_eq!(drain_total(&mut heap), 0);
}

#[test]
fn halve_largest_reorders() {
    let mut heap = BinaryHeap::from(vec![10, 7, 3]);
    halve_largest(&mut heap);
    assert_eq!(heap.peek(), Some(&7), "5 is no longer the max");
    halve_largest(&mut heap);
    assert_eq!(heap.into_sorted_vec(), [3, 3, 5]);
    let mut empty = BinaryHeap::new();
    halve_largest(&mut empty);
    assert!(empty.is_empty());
}

#[test]
fn drop_odds_retains_evens() {
    let mut heap = BinaryHeap::from(vec![1, 2, 3, 4, -6, -7]);
    drop_odds(&mut heap);
    assert_eq!(heap.into_sorted_vec(), [-6, 2, 4]);
}
