// Tests for Searching and Splitting — shared by template.rs, solution.rs and your attempt.
use super::*;

const EMPTY: &[i32] = &[];

// ── Windows and chunks ──

#[test]
fn has_adjacent_duplicate_checks_neighbours_only() {
    assert!(has_adjacent_duplicate(&[1, 2, 2, 3]));
    assert!(!has_adjacent_duplicate(&[1, 2, 1, 2]), "equal but not adjacent");
    assert!(!has_adjacent_duplicate(&[7]));
    assert!(!has_adjacent_duplicate(&[]));
}

#[test]
fn max_window_sum_slides() {
    assert_eq!(max_window_sum(&[1, -2, 3, 4, -1], 2), Some(7));
    assert_eq!(max_window_sum(&[1, 2, 3], 3), Some(6));
    assert_eq!(max_window_sum(&[-5, -1, -3], 1), Some(-1));
    assert_eq!(max_window_sum(&[1, 2], 3), None, "shorter than the window");
}

#[test]
fn chunk_sums_keeps_the_short_tail() {
    assert_eq!(chunk_sums(&[1, 2, 3, 4, 5], 2), vec![3, 7, 5]);
    assert_eq!(chunk_sums(&[1, 2, 3], 3), vec![6]);
    assert_eq!(chunk_sums(&[], 4), Vec::<i32>::new());
}

#[test]
fn full_chunks_separates_the_remainder() {
    let (chunks, rest) = full_chunks(&[1, 2, 3, 4, 5, 6, 7], 3);
    assert_eq!(chunks, vec![&[1, 2, 3][..], &[4, 5, 6]]);
    assert_eq!(rest, &[7]);
    let (chunks, rest) = full_chunks(&[1, 2], 2);
    assert_eq!(chunks, vec![&[1, 2][..]]);
    assert_eq!(rest, EMPTY);
    let (chunks, rest) = full_chunks(&[1, 2], 5);
    assert!(chunks.is_empty());
    assert_eq!(rest, &[1, 2]);
}

#[test]
fn chunks_from_end_puts_the_short_group_first_in_the_slice() {
    assert_eq!(
        chunks_from_end(&[1, 2, 3, 4, 5], 2),
        vec![&[4, 5][..], &[2, 3], &[1]]
    );
    assert_eq!(chunks_from_end(&[1, 2, 3], 3), vec![&[1, 2, 3][..]]);
}

#[test]
fn ascending_runs_break_on_equal_or_smaller() {
    assert_eq!(
        ascending_runs(&[1, 2, 2, 5, 3]),
        vec![&[1, 2][..], &[2, 5], &[3]]
    );
    assert_eq!(ascending_runs(&[3, 2, 1]), vec![&[3][..], &[2], &[1]]);
    assert_eq!(ascending_runs(&[1, 4, 9]), vec![&[1, 4, 9][..]]);
    assert!(ascending_runs(&[]).is_empty(), "no runs at all, not one empty run");
}

// ── Splitting ──

#[test]
fn split_on_zero_keeps_empty_runs() {
    assert_eq!(split_on_zero(&[1, 2, 0, 3]), vec![&[1, 2][..], &[3]]);
    assert_eq!(split_on_zero(&[0, 1, 0, 0]), vec![EMPTY, &[1], EMPTY, EMPTY]);
    assert_eq!(split_on_zero(&[]), vec![EMPTY], "one empty piece");
}

#[test]
fn split_at_first_stops_after_one_split() {
    assert_eq!(split_at_first(&[1, 9, 2, 9, 3], 9), vec![&[1][..], &[2, 9, 3]]);
    assert_eq!(split_at_first(&[1, 2], 9), vec![&[1, 2][..]]);
}

#[test]
fn head_and_tail_splits_off_the_first() {
    assert_eq!(head_and_tail(&[1, 2, 3]), Some((1, &[2, 3][..])));
    assert_eq!(head_and_tail(&[5]), Some((5, EMPTY)));
    assert_eq!(head_and_tail(&[]), None);
}

#[test]
fn all_but_last_drops_one() {
    assert_eq!(all_but_last(&[1, 2, 3]), &[1, 2]);
    assert_eq!(all_but_last(&[1]), EMPTY);
    assert_eq!(all_but_last(&[]), EMPTY);
}

#[test]
fn take_prefix_checks_the_length() {
    assert_eq!(take_prefix(&[1, 2, 3], 2), Some((&[1, 2][..], &[3][..])));
    assert_eq!(take_prefix(&[1, 2, 3], 3), Some((&[1, 2, 3][..], EMPTY)));
    assert_eq!(take_prefix(&[1, 2, 3], 0), Some((EMPTY, &[1, 2, 3][..])));
    assert_eq!(take_prefix(&[1, 2, 3], 4), None, "must not panic");
}

#[test]
fn swap_halves_even_and_odd() {
    let mut even = [1, 2, 3, 4];
    swap_halves(&mut even);
    assert_eq!(even, [3, 4, 1, 2]);
    let mut odd = [1, 2, 3, 4, 5];
    swap_halves(&mut odd);
    assert_eq!(odd, [4, 5, 3, 1, 2], "the middle stays put");
    let mut one = [9];
    swap_halves(&mut one);
    assert_eq!(one, [9]);
    let mut none: [i32; 0] = [];
    swap_halves(&mut none);
}

// ── Ends and membership ──

#[test]
fn first_and_last_of_short_slices() {
    assert_eq!(first_and_last(&[1, 2, 3]), Some((1, 3)));
    assert_eq!(first_and_last(&[4]), Some((4, 4)));
    assert_eq!(first_and_last(&[]), None);
}

#[test]
fn framed_by_checks_both_ends() {
    assert!(framed_by(&[1, 2, 3, 4], &[1, 2], &[4]));
    assert!(framed_by(&[1, 2], &[], &[]), "empty prefix and suffix always match");
    assert!(!framed_by(&[1, 2, 3], &[1], &[2]));
    assert!(!framed_by(&[1], &[1, 2], &[]), "prefix longer than the slice");
}

#[test]
fn contains_all_needles() {
    assert!(contains_all(&[3, 1, 4, 1, 5], &[5, 1]));
    assert!(!contains_all(&[3, 1, 4], &[1, 2]));
    assert!(contains_all(&[], &[]));
}

// ── Searching ──

#[test]
fn insertion_point_found_or_not() {
    let sorted = [1, 3, 5, 7];
    assert_eq!(insertion_point(&sorted, 5), 2);
    assert_eq!(insertion_point(&sorted, 4), 2);
    assert_eq!(insertion_point(&sorted, 0), 0);
    assert_eq!(insertion_point(&sorted, 99), 4);
    assert_eq!(insertion_point(&[], 1), 0);
}

#[test]
fn name_for_id_searches_by_key() {
    let people = [(2, "ada"), (5, "grace"), (9, "linus")];
    assert_eq!(name_for_id(&people, 5), Some("grace"));
    assert_eq!(name_for_id(&people, 9), Some("linus"));
    assert_eq!(name_for_id(&people, 3), None);
    assert_eq!(name_for_id(&[], 1), None);
}

#[test]
fn count_in_range_is_half_open() {
    let sorted = [1, 2, 2, 2, 5, 8, 8];
    assert_eq!(count_in_range(&sorted, 2, 8), 4);
    assert_eq!(count_in_range(&sorted, 2, 9), 6);
    assert_eq!(count_in_range(&sorted, 3, 5), 0);
    assert_eq!(count_in_range(&sorted, 8, 2), 0, "hi below lo is empty, not a panic");
}

// ── Joining ──

#[test]
fn flatten_rows_concatenates() {
    assert_eq!(flatten_rows(&[vec![1, 2], vec![], vec![3]]), vec![1, 2, 3]);
    assert_eq!(flatten_rows(&[]), Vec::<i32>::new());
}

#[test]
fn join_with_puts_sep_between_rows() {
    assert_eq!(join_with(&[vec![1, 2], vec![3]], 0), vec![1, 2, 0, 3]);
    assert_eq!(join_with(&[vec![1], vec![], vec![2]], 0), vec![1, 0, 0, 2]);
    assert_eq!(join_with(&[vec![4, 5]], 0), vec![4, 5], "no separator for one row");
}

// ── Slice patterns ──

#[test]
fn inner_drops_both_ends() {
    assert_eq!(inner(&[1, 2, 3, 4]), &[2, 3]);
    assert_eq!(inner(&[1, 2]), EMPTY);
    assert_eq!(inner(&[1]), EMPTY);
    assert_eq!(inner(&[]), EMPTY);
}

#[test]
fn first_two_and_rest_needs_two() {
    assert_eq!(first_two_and_rest(&[1, 2, 3, 4]), Some((1, 2, &[3, 4][..])));
    assert_eq!(first_two_and_rest(&[1, 2]), Some((1, 2, EMPTY)));
    assert_eq!(first_two_and_rest(&[1]), None);
}

#[test]
fn is_palindrome_generic() {
    assert!(is_palindrome(&[1, 2, 3, 2, 1]));
    assert!(is_palindrome(&['a', 'b', 'b', 'a']));
    assert!(is_palindrome::<i32>(&[]));
    assert!(is_palindrome(&["solo"]));
    assert!(!is_palindrome(&[1, 2, 3, 1]));
}
