// Tests for Editing in Place — shared by template.rs, solution.rs and your attempt.
use super::*;

// ── Removing ──

#[test]
fn drop_negatives_keeps_order() {
    let mut v = vec![3, -1, 0, -7, 5];
    drop_negatives(&mut v);
    assert_eq!(v, [3, 0, 5]);
    let mut all_negative = vec![-1, -2];
    drop_negatives(&mut all_negative);
    assert!(all_negative.is_empty());
}

#[test]
fn tick_down_decrements_and_removes() {
    let mut timers = vec![3, 1, 2, 1, 5];
    tick_down(&mut timers);
    assert_eq!(timers, [2, 1, 4]);
    tick_down(&mut timers);
    assert_eq!(timers, [1, 3]);
    let mut stale = vec![0, 4];
    tick_down(&mut stale);
    assert_eq!(stale, [3], "a timer already at 0 must not underflow");
}

#[test]
fn collapse_runs_only_merges_neighbours() {
    let mut v = vec![1, 1, 2, 2, 2, 1, 3, 3];
    collapse_runs(&mut v);
    assert_eq!(v, [1, 2, 1, 3]);
    let mut empty: Vec<i32> = vec![];
    collapse_runs(&mut empty);
    assert!(empty.is_empty());
}

#[test]
fn one_per_decade_keeps_the_first_of_each_run() {
    let mut years = vec![1969, 1971, 1979, 1980, 1989, 1975, 2001];
    one_per_decade(&mut years);
    assert_eq!(years, [1969, 1971, 1980, 1975, 2001]);
}

#[test]
fn cut_range_removes_and_returns() {
    let mut v = vec![10, 20, 30, 40, 50];
    assert_eq!(cut_range(&mut v, 1..3), [20, 30]);
    assert_eq!(v, [10, 40, 50]);
    assert_eq!(cut_range(&mut v, 2..2), Vec::<i32>::new());
    assert_eq!(v, [10, 40, 50]);
    assert_eq!(cut_range(&mut v, 0..3), [10, 40, 50]);
    assert!(v.is_empty());
}

#[test]
fn extract_evens_partitions_in_place() {
    let mut v = vec![1, 2, 3, 4, 6, 7, -8];
    assert_eq!(extract_evens(&mut v), [2, 4, 6, -8]);
    assert_eq!(v, [1, 3, 7]);
    assert_eq!(extract_evens(&mut v), Vec::<i32>::new());
}

#[test]
fn split_tail_returns_the_back() {
    let mut v = vec![1, 2, 3, 4, 5];
    assert_eq!(split_tail(&mut v, 3), [4, 5]);
    assert_eq!(v, [1, 2, 3]);
    assert_eq!(split_tail(&mut v, 3), Vec::<i32>::new(), "at == len is allowed");
}

#[test]
fn keep_first_truncates() {
    let mut v = vec![1, 2, 3];
    keep_first(&mut v, 2);
    assert_eq!(v, [1, 2]);
    keep_first(&mut v, 10);
    assert_eq!(v, [1, 2], "truncating beyond the length does nothing");
}

#[test]
fn remove_unordered_moves_the_last_in() {
    let mut v = vec![10, 20, 30, 40];
    assert_eq!(remove_unordered(&mut v, 1), 20);
    assert_eq!(v, [10, 40, 30]);
    assert_eq!(remove_unordered(&mut v, 2), 30);
    assert_eq!(v, [10, 40]);
}

// ── Growing ──

#[test]
fn fit_to_pads_or_cuts() {
    let mut v = vec![1, 2];
    fit_to(&mut v, 4, 0);
    assert_eq!(v, [1, 2, 0, 0]);
    fit_to(&mut v, 1, 9);
    assert_eq!(v, [1]);
}

#[test]
fn append_all_copies_onto_the_end() {
    let mut v = vec![1];
    append_all(&mut v, &[2, 3]);
    assert_eq!(v, [1, 2, 3]);
    append_all(&mut v, &[]);
    assert_eq!(v, [1, 2, 3]);
}

#[test]
fn double_up_repeats_the_contents() {
    let mut v = vec![1, 2, 3];
    double_up(&mut v);
    assert_eq!(v, [1, 2, 3, 1, 2, 3]);
    let mut empty: Vec<i32> = vec![];
    double_up(&mut empty);
    assert!(empty.is_empty());
}

#[test]
fn insert_sorted_keeps_order() {
    let mut v = vec![];
    for x in [5, 1, 3, 9, 0] {
        insert_sorted(&mut v, (x, '-'));
    }
    let keys: Vec<i32> = v.iter().map(|&(k, _)| k).collect();
    assert_eq!(keys, [0, 1, 3, 5, 9]);
}

#[test]
fn insert_sorted_puts_equal_keys_last() {
    let mut v = vec![(1, 'x'), (3, 'm'), (3, 'n'), (3, 'p'), (5, 'y')];
    insert_sorted(&mut v, (3, 'a'));
    assert_eq!(
        v,
        [(1, 'x'), (3, 'm'), (3, 'n'), (3, 'p'), (3, 'a'), (5, 'y')],
        "after every existing 3, even though 'a' < 'm'"
    );
    insert_sorted(&mut v, (3, 'z'));
    assert_eq!(v[5], (3, 'z'), "after the 3s, before the 5");
}

// ── Rearranging ──

#[test]
fn rotate_both_ways_and_wraps() {
    let mut xs = [1, 2, 3, 4, 5];
    rotate(&mut xs, 2);
    assert_eq!(xs, [4, 5, 1, 2, 3]);
    rotate(&mut xs, -2);
    assert_eq!(xs, [1, 2, 3, 4, 5]);
    rotate(&mut xs, 7);
    assert_eq!(xs, [4, 5, 1, 2, 3], "7 steps on 5 elements is 2");
    rotate(&mut xs, -10);
    assert_eq!(xs, [4, 5, 1, 2, 3]);
    let mut empty: [i32; 0] = [];
    rotate(&mut empty, 3);
}

#[test]
fn reverse_each_chunk_handles_a_short_tail() {
    let mut xs = [1, 2, 3, 4, 5, 6, 7, 8];
    reverse_each_chunk(&mut xs, 3);
    assert_eq!(xs, [3, 2, 1, 6, 5, 4, 8, 7]);
    let mut ys = [1, 2, 3];
    reverse_each_chunk(&mut ys, 1);
    assert_eq!(ys, [1, 2, 3]);
}

#[test]
fn swap_ends_tolerates_short_slices() {
    let mut xs = [1, 2, 3];
    swap_ends(&mut xs);
    assert_eq!(xs, [3, 2, 1]);
    let mut one = [7];
    swap_ends(&mut one);
    assert_eq!(one, [7]);
    let mut empty: [i32; 0] = [];
    swap_ends(&mut empty);
}

#[test]
fn reset_overwrites_everything() {
    let mut xs = [4, 5, 6];
    reset(&mut xs, -1);
    assert_eq!(xs, [-1, -1, -1]);
}

#[test]
fn number_from_counts_up() {
    let mut ids = [0; 4];
    number_from(&mut ids, 10);
    assert_eq!(ids, [10, 11, 12, 13]);
}

#[test]
fn number_from_wraps_at_the_top() {
    let mut one = [0];
    number_from(&mut one, u32::MAX);
    assert_eq!(one, [u32::MAX], "the last number is fine to hand out");
    let mut ids = [0; 3];
    number_from(&mut ids, u32::MAX - 1);
    assert_eq!(ids, [u32::MAX - 1, u32::MAX, 0]);
}

#[test]
fn clamp_all_bounds_every_element() {
    let mut xs = [-5, 0, 3, 12, 7];
    clamp_all(&mut xs, 0, 10);
    assert_eq!(xs, [0, 0, 3, 10, 7]);
}

// ── Sorting ──

#[test]
fn sort_by_distance_breaks_ties_by_value() {
    let mut xs = [1, 9, 5, 4, 6, 10];
    sort_by_distance(&mut xs, 5);
    assert_eq!(xs, [5, 4, 6, 1, 9, 10]);
    let mut negatives = [i32::MIN, 0, i32::MAX];
    sort_by_distance(&mut negatives, -1);
    assert_eq!(negatives, [0, i32::MIN, i32::MAX], "distances must not overflow");
}

#[test]
fn sort_people_oldest_first_then_by_name() {
    let mut people = vec![
        ("cy".to_string(), 30),
        ("ann".to_string(), 41),
        ("bo".to_string(), 30),
        ("al".to_string(), 25),
    ];
    sort_people(&mut people);
    let order: Vec<&str> = people.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(order, ["ann", "bo", "cy", "al"]);
}

#[test]
fn sort_case_insensitive_is_stable() {
    let mut words: Vec<String> = ["banana", "Apple", "cherry", "apple", "BANANA"]
        .map(String::from)
        .into();
    sort_case_insensitive(&mut words);
    assert_eq!(words, ["Apple", "apple", "banana", "BANANA", "cherry"]);
}
