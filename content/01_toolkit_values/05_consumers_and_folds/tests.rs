// Tests for Consumers and Folds — shared by template.rs, solution.rs and your attempt.
use super::*;

// ── Arithmetic ──

#[test]
fn total_widens_before_summing() {
    assert_eq!(total(&[1, 2, 3]), 6);
    assert_eq!(total(&[]), 0);
    assert_eq!(total(&[i32::MAX, i32::MAX, i32::MIN]), i64::from(i32::MAX) - 1, "would overflow i32");
}

#[test]
fn factorial_products() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(20), 2_432_902_008_176_640_000);
}

#[test]
fn mean_of_floats() {
    assert_eq!(mean(&[]), None);
    let m = mean(&[1.0, 2.0, 4.5]).unwrap_or(f64::NAN);
    assert!((m - 2.5).abs() < 1e-12, "got {m}");
}

// ── Counting and testing ──

#[test]
fn count_vowels_counts_both_cases() {
    assert_eq!(count_vowels("Programming in Rust"), 5);
    assert_eq!(count_vowels("AEIOUaeiou"), 10);
    assert_eq!(count_vowels("rhythm"), 0);
}

#[test]
fn has_negative_uses_any() {
    assert!(has_negative(&[3, 0, -1]));
    assert!(!has_negative(&[0, 1]));
    assert!(!has_negative(&[]), "any is false on empty");
}

#[test]
fn all_lowercase_uses_all() {
    assert!(all_lowercase("rust"));
    assert!(!all_lowercase("Rust"));
    assert!(!all_lowercase("ru st"));
    assert!(all_lowercase(""), "all is true on empty");
}

#[test]
fn same_words_ignores_spacing() {
    assert!(same_words("a  b\tc", " a b c\n"));
    assert!(!same_words("a b c", "a b"), "a prefix isn't equal");
    assert!(!same_words("a b", "b a"), "order matters");
    assert!(same_words("", "   "), "no words either side");
    assert!(!same_words("ab", "a b"));
}

// ── Extremes ──

#[test]
fn shortest_word_takes_first_tie() {
    assert_eq!(shortest_word("bb a c dd"), Some("a"), "first of the ties");
    assert_eq!(shortest_word(""), None);
}

#[test]
fn longest_word_takes_last_tie() {
    assert_eq!(longest_word("a bb cc d"), Some("cc"), "last of the ties");
    assert_eq!(longest_word("   "), None);
}

#[test]
fn min_max_in_one_pass() {
    assert_eq!(min_max(&[3, -1, 7]), Some((-1, 7)));
    assert_eq!(min_max(&[4]), Some((4, 4)));
    assert_eq!(min_max(&[]), None);
    assert_eq!(min_max(&[i32::MAX, i32::MIN]), Some((i32::MIN, i32::MAX)));
}

// ── Folding ──

#[test]
fn to_number_folds_digits() {
    assert_eq!(to_number(&[4, 0, 2]), 402);
    assert_eq!(to_number(&[0, 0, 7]), 7);
    assert_eq!(to_number(&[]), 0);
    assert_eq!(to_number(&[9; 19]), 9_999_999_999_999_999_999, "19 digits fit in a u64");
}

#[test]
fn hyphenate_reduces() {
    assert_eq!(hyphenate(&["a", "b", "c"]), Some("a-b-c".to_string()));
    assert_eq!(hyphenate(&["solo"]), Some("solo".to_string()));
    assert_eq!(hyphenate(&[]), None);
}

#[test]
fn nest_folds_from_the_right() {
    assert_eq!(nest(&["f", "g", "x"]), "f(g(x))");
    assert_eq!(nest(&["only"]), "only");
    assert_eq!(nest(&[]), "");
    assert_eq!(nest(&["", "x"]), "(x)", "an empty name still wraps");
}

// ── Searching ──

#[test]
fn index_of_finds_the_first() {
    assert_eq!(index_of(&[5, 1, 5, 2], 5), Some(0));
    assert_eq!(index_of(&[5, 1, 5, 2], 9), None);
}

#[test]
fn last_index_of_finds_the_last() {
    assert_eq!(last_index_of(&[5, 1, 5, 2], 5), Some(2));
    assert_eq!(last_index_of(&[], 1), None);
}

#[test]
fn first_long_word_finds() {
    assert_eq!(first_long_word(&["a", "abc", "abcd"], 3), Some("abc"));
    assert_eq!(first_long_word(&["a"], 2), None);
}

#[test]
fn last_before_zero_stops_at_zero() {
    assert_eq!(last_before_zero(&[1, 2, 0, 9]), Some(2));
    assert_eq!(last_before_zero(&[1, 2, 3]), Some(3));
    assert_eq!(last_before_zero(&[0, 1]), None);
    assert_eq!(last_before_zero(&[]), None);
}

#[test]
fn nth_word_counts_from_zero() {
    assert_eq!(nth_word("zero one  two", 2), Some("two"));
    assert_eq!(nth_word("zero", 0), Some("zero"));
    assert_eq!(nth_word("zero", 1), None);
}

// ── Splitting ──

#[test]
fn split_even_odd_partitions() {
    assert_eq!(split_even_odd(&[1, 2, 3, 4, -5, 0]), (vec![2, 4, 0], vec![1, 3, -5]));
    assert_eq!(split_even_odd(&[]), (vec![], vec![]));
}

#[test]
fn split_pairs_unzips_into_a_string() {
    assert_eq!(split_pairs(&[('h', 1), ('i', 2)]), ("hi".to_string(), vec![1, 2]));
    assert_eq!(split_pairs(&[]), (String::new(), vec![]));
}

// ── Collecting ──

#[test]
fn word_lengths_counts_chars() {
    let m = word_lengths(&["ab", "héé", "ab"]);
    assert_eq!(m, HashMap::from([("ab", 2), ("héé", 3)]));
}

#[test]
fn initials_skips_empty_names() {
    assert_eq!(initials(&["Ada", "", "Grace", "Émile"]), "AGÉ");
    assert_eq!(initials(&[]), "");
}

#[test]
fn distinct_sorted_collects_a_set() {
    let set = distinct_sorted(&[3, 1, 3, -2, 1]);
    assert_eq!(set.into_iter().collect::<Vec<_>>(), vec![-2, 1, 3]);
}
