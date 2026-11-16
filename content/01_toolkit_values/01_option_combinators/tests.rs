// Tests for Option Combinators — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;

// ── Transforming ──

#[test]
fn double_maps_the_value() {
    assert_eq!(double(Some(4)), Some(8));
    assert_eq!(double(Some(-3)), Some(-6));
    assert_eq!(double(None), None);
}

#[test]
fn len_or_zero_defaults() {
    assert_eq!(len_or_zero(Some("hello")), 5);
    assert_eq!(len_or_zero(Some("")), 0);
    assert_eq!(len_or_zero(None), 0);
}

#[test]
fn first_char_upper_handles_empty() {
    assert_eq!(first_char_upper("rust"), Some('R'));
    assert_eq!(first_char_upper("Zig"), Some('Z'));
    assert_eq!(first_char_upper(""), None);
    assert_eq!(first_char_upper("éa"), Some('é'), "non-ASCII is left alone");
}

// ── Defaults ──

#[test]
fn name_or_anonymous_defaults() {
    assert_eq!(name_or_anonymous(Some("Ferris")), "Ferris");
    assert_eq!(name_or_anonymous(None), "anonymous");
}

#[test]
fn or_else_build_is_lazy() {
    let calls = Cell::new(0);
    let build = || {
        calls.set(calls.get() + 1);
        "built".to_string()
    };
    assert_eq!(or_else_build(Some("have".into()), build), "have");
    assert_eq!(calls.get(), 0, "build must not run when there's a value");
    assert_eq!(or_else_build(None, build), "built");
    assert_eq!(calls.get(), 1);
}

// ── Filtering and chaining ──

#[test]
fn even_only_filters() {
    assert_eq!(even_only(Some(4)), Some(4));
    assert_eq!(even_only(Some(0)), Some(0));
    assert_eq!(even_only(Some(7)), None);
    assert_eq!(even_only(None), None);
}

#[test]
fn parse_positive_rejects_zero_and_junk() {
    assert_eq!(parse_positive("42"), Some(42));
    assert_eq!(parse_positive("0"), None);
    assert_eq!(parse_positive("-5"), None);
    assert_eq!(parse_positive("4x"), None);
    assert_eq!(parse_positive(""), None);
}

#[test]
fn lookup_twice_chains() {
    let map: HashMap<&str, &str> = [("a", "b"), ("b", "c"), ("x", "nowhere")].into();
    assert_eq!(lookup_twice(&map, "a"), Some("c"));
    assert_eq!(lookup_twice(&map, "b"), None, "c isn't a key");
    assert_eq!(lookup_twice(&map, "x"), None);
    assert_eq!(lookup_twice(&map, "zzz"), None);
}

#[test]
fn flatten_nested_unwraps_one_level() {
    assert_eq!(flatten_nested(Some(Some(3))), Some(3));
    assert_eq!(flatten_nested(Some(None)), None);
    assert_eq!(flatten_nested(None), None);
}

// ── Combining ──

#[test]
fn first_some_prefers_a() {
    assert_eq!(first_some(Some(1), Some(2)), Some(1));
    assert_eq!(first_some(None, Some(2)), Some(2));
    assert_eq!(first_some::<i32>(None, None), None);
}

#[test]
fn exactly_one_is_exclusive() {
    assert_eq!(exactly_one(Some('a'), None), Some('a'));
    assert_eq!(exactly_one(None, Some('b')), Some('b'));
    assert_eq!(exactly_one(Some('a'), Some('b')), None);
    assert_eq!(exactly_one::<char>(None, None), None);
}

#[test]
fn add_both_needs_both() {
    assert_eq!(add_both(Some(2), Some(3)), Some(5));
    assert_eq!(add_both(Some(2), None), None);
    assert_eq!(add_both(None, Some(3)), None);
}

#[test]
fn split_pair_unzips() {
    assert_eq!(split_pair(Some((1, 'x'))), (Some(1), Some('x')));
    assert_eq!(split_pair(None), (None, None));
}

// ── Inspecting ──

#[test]
fn is_positive_checks_presence_and_sign() {
    assert!(is_positive(Some(1)));
    assert!(!is_positive(Some(0)));
    assert!(!is_positive(Some(-1)));
    assert!(!is_positive(None));
}

// ── Slots ──

#[test]
fn take_out_leaves_none() {
    let mut slot = Some("x".to_string());
    assert_eq!(take_out(&mut slot).as_deref(), Some("x"));
    assert_eq!(slot, None);
    assert_eq!(take_out(&mut slot), None);
}

#[test]
fn swap_in_returns_the_old_value() {
    let mut slot = None;
    assert_eq!(swap_in(&mut slot, 1), None);
    assert_eq!(swap_in(&mut slot, 2), Some(1));
    assert_eq!(slot, Some(2));
}

#[test]
fn get_or_start_creates_once() {
    let mut slot = None;
    get_or_start(&mut slot).push(1);
    get_or_start(&mut slot).push(2);
    assert_eq!(slot, Some(vec![1, 2]));
    let mut full = Some(vec![9]);
    get_or_start(&mut full).push(10);
    assert_eq!(full, Some(vec![9, 10]), "an existing list is kept");
}

// ── Borrowing ──

#[test]
fn as_name_borrows() {
    let name = Some("Ferris".to_string());
    let borrowed: Option<&str> = as_name(&name);
    assert_eq!(borrowed, Some("Ferris"));
    assert_eq!(as_name(&None), None);
    assert_eq!(name.as_deref(), Some("Ferris"), "still ours");
}

// ── To and from Result ──

#[test]
fn require_converts_to_result() {
    assert_eq!(require(Some(7)), Ok(7));
    assert_eq!(require(None), Err("missing"));
}

#[test]
fn parse_if_present_transposes() {
    assert_eq!(parse_if_present(Some("12")), Ok(Some(12)));
    assert_eq!(parse_if_present(None), Ok(None));
    assert!(parse_if_present(Some("twelve")).is_err());
}

// ── Many at once ──

#[test]
fn sum_all_short_circuits_on_none() {
    assert_eq!(sum_all(&[Some(1), Some(2), Some(3)]), Some(6));
    assert_eq!(sum_all(&[Some(1), None, Some(3)]), None);
    assert_eq!(sum_all(&[]), Some(0), "the empty sum is 0");
}
