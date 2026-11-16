// Tests for Iterator Adaptors — shared by template.rs, solution.rs and your attempt.
use super::*;

// ── Transforming ──

#[test]
fn squares_maps() {
    assert_eq!(squares(&[1, -2, 3]), vec![1, 4, 9]);
    assert_eq!(squares(&[]), Vec::<i32>::new());
}

#[test]
fn evens_filters() {
    assert_eq!(evens(&[1, 2, 3, 4]), vec![2, 4]);
    assert_eq!(evens(&[-4, -3, 0]), vec![-4, 0]);
    assert_eq!(evens(&[1, 3]), Vec::<i32>::new());
}

#[test]
fn leading_digits_stops_at_first_non_digit() {
    assert_eq!(leading_digits("123abc4"), vec![1, 2, 3]);
    assert_eq!(leading_digits("abc"), Vec::<u32>::new());
    assert_eq!(leading_digits("907"), vec![9, 0, 7]);
}

// ── Positions ──

#[test]
fn indexed_labels_count_from_zero() {
    assert_eq!(indexed_labels(&["a", "b"]), vec!["0: a", "1: b"]);
    assert!(indexed_labels(&[]).is_empty());
}

#[test]
fn positions_of_uses_byte_offsets() {
    assert_eq!(positions_of("banana", 'a'), vec![1, 3, 5]);
    assert_eq!(positions_of("héhé", 'h'), vec![0, 3], "é is two bytes");
    assert_eq!(positions_of("xyz", 'a'), Vec::<usize>::new());
}

// ── Combining ──

#[test]
fn pairwise_sums_stops_at_shorter() {
    assert_eq!(pairwise_sums(&[1, 2, 3], &[10, 20]), vec![11, 22]);
    assert_eq!(pairwise_sums(&[1], &[10, 20, 30]), vec![11]);
    assert_eq!(pairwise_sums(&[], &[1]), Vec::<i32>::new());
}

#[test]
fn concat_chains() {
    assert_eq!(concat(&[1, 2], &[3]), vec![1, 2, 3]);
    assert_eq!(concat(&[], &[]), Vec::<i32>::new());
}

#[test]
fn framed_wraps_with_once() {
    assert_eq!(framed(&[1, 2], 0, 9), vec![0, 1, 2, 9]);
    assert_eq!(framed(&[], -1, -1), vec![-1, -1], "the frame is there even with no values");
}

#[test]
fn interleave_appends_the_leftovers() {
    assert_eq!(interleave(&[1, 2, 3], &[10]), vec![1, 10, 2, 3]);
    assert_eq!(interleave(&[1], &[10, 20, 30]), vec![1, 10, 20, 30]);
    assert_eq!(interleave(&[1, 2], &[10, 20]), vec![1, 10, 2, 20]);
    assert_eq!(interleave(&[], &[5]), vec![5]);
}

#[test]
fn adjacent_differences_without_windows() {
    assert_eq!(adjacent_differences(&[1, 4, 9, 16]), vec![3, 5, 7]);
    assert_eq!(adjacent_differences(&[5, 2]), vec![-3]);
    assert_eq!(adjacent_differences(&[7]), Vec::<i32>::new());
    assert_eq!(adjacent_differences(&[]), Vec::<i32>::new());
}

// ── Slicing ──

#[test]
fn page_skips_and_takes() {
    let xs = [1, 2, 3, 4, 5];
    assert_eq!(page(&xs, 0, 2), vec![1, 2]);
    assert_eq!(page(&xs, 1, 2), vec![3, 4]);
    assert_eq!(page(&xs, 2, 2), vec![5], "the last page is short");
    assert_eq!(page(&xs, 3, 2), Vec::<i32>::new());
}

#[test]
fn page_offset_scales_with_page_size() {
    let xs: Vec<i32> = (1..=10).collect();
    assert_eq!(page(&xs, 2, 3), vec![7, 8, 9], "page 2 of size 3 starts at index 6");
    assert_eq!(page(&xs, 1, 5), vec![6, 7, 8, 9, 10]);
    assert_eq!(page(&xs, 1, 0), Vec::<i32>::new(), "empty pages");
}

#[test]
fn strip_leading_zeros_keeps_inner_zeros() {
    assert_eq!(strip_leading_zeros(&[0, 0, 1, 0, 2]), vec![1, 0, 2]);
    assert_eq!(strip_leading_zeros(&[0, 0]), Vec::<u8>::new());
    assert_eq!(strip_leading_zeros(&[3, 0]), vec![3, 0]);
}

#[test]
fn take_while_positive_stops_for_good() {
    assert_eq!(take_while_positive(&[3, 1, 0, 5]), vec![3, 1], "5 comes after the stop");
    assert_eq!(take_while_positive(&[-1, 2]), Vec::<i32>::new());
    assert_eq!(take_while_positive(&[1, 2]), vec![1, 2]);
}

#[test]
fn every_nth_steps() {
    assert_eq!(every_nth(&[0, 1, 2, 3, 4, 5, 6], 3), vec![0, 3, 6]);
    assert_eq!(every_nth(&[1, 2], 1), vec![1, 2]);
    assert_eq!(every_nth(&[1, 2], 5), vec![1]);
}

#[test]
fn last_n_keeps_order() {
    assert_eq!(last_n(&[1, 2, 3, 4], 2), vec![3, 4]);
    assert_eq!(last_n(&[1, 2], 5), vec![1, 2]);
    assert_eq!(last_n(&[1, 2], 0), Vec::<i32>::new());
}

#[test]
fn split_at_blank_resumes_after_take_while() {
    assert_eq!(
        split_at_blank(&["Host: x", "Accept: *", "", "body", "", "more"]),
        (vec!["Host: x", "Accept: *"], vec!["body", "", "more"]),
        "only the first empty line splits"
    );
    assert_eq!(split_at_blank(&["a", "b"]), (vec!["a", "b"], vec![]));
    assert_eq!(split_at_blank(&["", "a"]), (vec![], vec!["a"]));
    assert_eq!(split_at_blank(&["a", " ", "b"]), (vec!["a", " ", "b"], vec![]), "a space isn't empty");
}

// ── Reversing ──

#[test]
fn reversed_words_normalizes_spaces() {
    assert_eq!(reversed_words("one two  three"), "three two one");
    assert_eq!(reversed_words("  solo "), "solo");
    assert_eq!(reversed_words(""), "");
}

// ── Nesting ──

#[test]
fn expand_flat_maps_runs() {
    assert_eq!(expand(&[('a', 2), ('b', 1), ('é', 3)]), "aabééé");
    assert_eq!(expand(&[('z', 0)]), "");
}

// ── State ──

#[test]
fn running_total_scans() {
    assert_eq!(running_total(&[1, 2, 3]), vec![1, 3, 6]);
    assert_eq!(running_total(&[5, -5, 2]), vec![5, 0, 2]);
    assert_eq!(running_total(&[]), Vec::<i32>::new());
}

#[test]
fn run_lengths_groups_neighbours() {
    assert_eq!(run_lengths("aaabcc"), vec![('a', 3), ('b', 1), ('c', 2)]);
    assert_eq!(run_lengths("abab"), vec![('a', 1), ('b', 1), ('a', 1), ('b', 1)], "only adjacent runs");
    assert_eq!(run_lengths("ééx"), vec![('é', 2), ('x', 1)]);
    assert_eq!(run_lengths(""), vec![]);
}

// ── Watching and repeating ──

#[test]
fn doubled_with_log_sees_originals() {
    let mut seen = Vec::new();
    assert_eq!(doubled_with_log(&[1, 2, 3], |x| seen.push(x)), vec![2, 4, 6]);
    assert_eq!(seen, vec![1, 2, 3]);
}

#[test]
fn repeat_pattern_cycles() {
    assert_eq!(repeat_pattern(b"ab", 5), b"ababa".to_vec());
    assert_eq!(repeat_pattern(b"xyz", 2), b"xy".to_vec());
    assert_eq!(repeat_pattern(b"", 4), Vec::<u8>::new(), "an empty cycle is empty");
    assert_eq!(repeat_pattern(b"q", 0), Vec::<u8>::new());
}
