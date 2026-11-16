// Tests for Collecting Options and Results — shared by template.rs, solution.rs and your attempt.
use super::*;

fn invalid_digit() -> ParseIntError {
    "x".parse::<i32>().unwrap_err()
}

fn empty() -> ParseIntError {
    "".parse::<i32>().unwrap_err()
}

// ── Collecting ──

#[test]
fn parse_all_collects_or_fails() {
    assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
    assert_eq!(parse_all(&[]), Ok(vec![]));
    assert_eq!(parse_all(&["1", "two", ""]), Err(invalid_digit()), "the first error wins");
}

#[test]
fn all_present_needs_every_entry() {
    assert_eq!(all_present(&[Some(1), Some(2)]), Some(vec![1, 2]));
    assert_eq!(all_present(&[Some(1), None, Some(3)]), None);
    assert_eq!(all_present(&[]), Some(vec![]));
}

#[test]
fn lookup_all_fails_on_a_missing_key() {
    let map: HashMap<&str, i32> = [("a", 1), ("b", 2)].into();
    assert_eq!(lookup_all(&map, &["b", "a", "b"]), Some(vec![2, 1, 2]));
    assert_eq!(lookup_all(&map, &["a", "z"]), None);
}

#[test]
fn parse_unique_collects_into_a_set() {
    assert_eq!(parse_unique(&["3", "1", "3", "2"]), Ok(BTreeSet::from([1, 2, 3])));
    assert_eq!(parse_unique(&["3", "-1"]), Err(invalid_digit()));
}

#[test]
fn collect_string_builds_or_fails() {
    assert_eq!(collect_string(&[Some('h'), Some('é')]).as_deref(), Some("hé"));
    assert_eq!(collect_string(&[Some('h'), None]), None);
    assert_eq!(collect_string(&[]).as_deref(), Some(""));
}

#[test]
fn try_map_stops_calling_after_the_first_error() {
    let mut calls = 0;
    let r = try_map(&[1, 2, -3, 4, -5], |&x| {
        calls += 1;
        if x < 0 { Err(x) } else { Ok(x * 10) }
    });
    assert_eq!(r, Err(-3));
    assert_eq!(calls, 3, "collect must short-circuit");
    assert_eq!(try_map(&["a", "bc"], |s| Ok::<_, ()>(s.len())), Ok(vec![1, 2]));
}

#[test]
fn parse_lines_skips_blank_lines() {
    assert_eq!(parse_lines("1\n  2 \n\n   \n3"), Ok(vec![1, 2, 3]));
    assert_eq!(parse_lines(""), Ok(vec![]));
    assert_eq!(parse_lines("1\nx\n"), Err(invalid_digit()));
}

// ── Summing ──

#[test]
fn sum_parsed_short_circuits() {
    assert_eq!(sum_parsed(&["1", "-2", "40"]), Ok(39));
    assert_eq!(sum_parsed(&["3000000000", "3000000000"]), Ok(6_000_000_000), "i64 sum");
    assert_eq!(sum_parsed(&[]), Ok(0));
    assert_eq!(sum_parsed(&["1", ""]), Err(empty()));
}

#[test]
fn product_present_short_circuits() {
    assert_eq!(product_present(&[Some(2), Some(3), Some(7)]), Some(42));
    assert_eq!(product_present(&[Some(2), None]), None);
    assert_eq!(product_present(&[]), Some(1), "the empty product is 1");
}

// ── Folding with an exit ──

#[test]
fn sum_until_negative_breaks_early() {
    assert_eq!(sum_until_negative(&[1, 2, 3]), 6, "no negative: sum everything");
    assert_eq!(sum_until_negative(&[4, 5, -1, 100]), 9);
    assert_eq!(sum_until_negative(&[-3, 7]), 0, "breaks before adding anything");
    assert_eq!(sum_until_negative(&[]), 0);
    assert_eq!(
        sum_until_negative(&[i32::MAX, -1, i32::MAX]),
        i32::MAX,
        "the values after the break must not be added"
    );
}

#[test]
fn running_balance_reports_the_overdraft() {
    assert_eq!(running_balance(10, &[-5, 3, -9, 100]), Err(2));
    assert_eq!(running_balance(10, &[-10, 5]), Ok(5), "exactly zero is fine");
    assert_eq!(running_balance(0, &[-1]), Err(0));
    assert_eq!(running_balance(7, &[]), Ok(7));
}

#[test]
fn all_non_negative_returns_first_negative() {
    assert_eq!(all_non_negative(&[0, 1, 2]), Ok(()));
    assert_eq!(all_non_negative(&[]), Ok(()));
    assert_eq!(all_non_negative(&[3, -1, -2]), Err(-1));
}

// ── Keeping the good ones ──

#[test]
fn valid_numbers_skips_junk() {
    assert_eq!(valid_numbers(&["1", "x", "-2", "", "3"]), vec![1, -2, 3]);
    assert_eq!(valid_numbers(&["x"]), Vec::<i32>::new());
}

#[test]
fn present_values_flattens() {
    assert_eq!(present_values(&[None, Some(1), None, Some(2)]), vec![1, 2]);
    assert_eq!(present_values(&[None]), Vec::<i32>::new());
}

#[test]
fn ok_values_flattens() {
    let rs = vec![Ok(1), Err("bad".to_string()), Ok(3)];
    assert_eq!(ok_values(rs), vec![1, 3]);
    assert_eq!(ok_values(vec![]), Vec::<i32>::new());
}

// ── Finding ──

#[test]
fn first_number_finds_first_parse() {
    assert_eq!(first_number(&["a", "b", "12", "13"]), Some(12));
    assert_eq!(first_number(&["a"]), None);
    assert_eq!(first_number(&[]), None);
}

#[test]
fn first_error_borrows_the_message() {
    let rs = vec![Ok(1), Err("first".to_string()), Err("second".to_string())];
    assert_eq!(first_error(&rs), Some("first"));
    assert_eq!(first_error(&[Ok(1), Ok(2)]), None);
}

// ── Splitting ──

#[test]
fn split_parsed_keeps_both_sides_in_order() {
    assert_eq!(split_parsed(&["1", "x", "2", "y"]), (vec![1, 2], vec!["x", "y"]));
    assert_eq!(split_parsed(&[]), (vec![], vec![]));
    let owned = String::from("oops 5");
    let words: Vec<&str> = owned.split(' ').collect();
    let (nums, bad) = split_parsed(&words);
    assert_eq!((nums, bad), (vec![5], vec!["oops"]), "bad words borrow from the input");
}

// ── Nesting ──

#[test]
fn parse_optional_fields_transposes() {
    assert_eq!(parse_optional_fields(&[Some("4"), None, Some("-1")]), Ok(vec![Some(4), None, Some(-1)]));
    assert_eq!(parse_optional_fields(&[None, None]), Ok(vec![None, None]));
    assert_eq!(parse_optional_fields(&[None, Some("x")]), Err(invalid_digit()));
}

#[test]
fn max_parsed_handles_empty_and_errors() {
    assert_eq!(max_parsed(&["3", "9", "-2"]), Ok(Some(9)));
    assert_eq!(max_parsed(&[]), Ok(None));
    assert_eq!(max_parsed(&["3", "nine"]), Err(invalid_digit()));
}
