// Tests for Result Combinators — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;

fn invalid_digit() -> ParseIntError {
    "x".parse::<i32>().unwrap_err()
}

fn empty() -> ParseIntError {
    "".parse::<i32>().unwrap_err()
}

// ── Transforming ──

#[test]
fn parse_double_maps_ok() {
    assert_eq!(parse_double("21"), Ok(42));
    assert_eq!(parse_double("-4"), Ok(-8));
    assert_eq!(parse_double("abc"), Err(invalid_digit()));
}

#[test]
fn describe_error_stringifies_only_err() {
    assert_eq!(describe_error(Ok(7)), Ok(7));
    assert_eq!(
        describe_error("300".parse::<u8>()),
        Err("number too large to fit in target type".to_string())
    );
    assert_eq!(
        describe_error("".parse::<u8>()),
        Err("cannot parse integer from empty string".to_string())
    );
}

#[test]
fn parse_or_tag_wraps_in_variant() {
    assert_eq!(parse_or_tag("12"), Ok(12));
    assert_eq!(parse_or_tag("-1"), Err(ConfigError::BadNumber(invalid_digit())));
}

// ── Chaining ──

#[test]
fn parse_even_chains_a_check() {
    assert_eq!(parse_even("8"), Ok(8));
    assert_eq!(parse_even("0"), Ok(0));
    assert_eq!(parse_even("7"), Err("7 is odd".to_string()));
    assert_eq!(parse_even("x"), Err("invalid digit found in string".to_string()));
}

#[test]
fn parse_with_fallback_only_on_failure() {
    let calls = Cell::new(0);
    let counted = |s: &'static str| {
        let calls = &calls;
        move || {
            calls.set(calls.get() + 1);
            s.parse::<i32>()
        }
    };
    assert_eq!(parse_with_fallback("1", counted("2")), Ok(1));
    assert_eq!(calls.get(), 0, "the fallback must not run when primary parses");
    assert_eq!(parse_with_fallback("oops", counted("5")), Ok(5));
    assert_eq!(calls.get(), 1);
    assert_eq!(parse_with_fallback("oops", counted("")), Err(empty()), "the fallback's error wins");
    assert_eq!(calls.get(), 2);
}

#[test]
fn first_ok_prefers_a() {
    assert_eq!(first_ok::<i32, &str>(Ok(1), Ok(2)), Ok(1));
    assert_eq!(first_ok(Err("a"), Ok(2)), Ok(2));
    assert_eq!(first_ok::<i32, &str>(Err("a"), Err("b")), Err("b"));
}

#[test]
fn then_second_needs_a_to_succeed() {
    assert_eq!(then_second::<i32, char, &str>(Ok(1), Ok('x')), Ok('x'));
    assert_eq!(then_second::<i32, char, &str>(Ok(1), Err("b")), Err("b"));
    assert_eq!(then_second::<i32, char, &str>(Err("a"), Ok('x')), Err("a"));
    assert_eq!(then_second::<i32, char, &str>(Err("a"), Err("b")), Err("a"));
}

// ── Defaults ──

#[test]
fn parse_or_zero_defaults() {
    assert_eq!(parse_or_zero("17"), 17);
    assert_eq!(parse_or_zero("nope"), 0);
    assert_eq!(parse_or_zero(""), 0);
}

#[test]
fn render_formats_both_sides() {
    assert_eq!(render(Ok(3)), "value: 3");
    assert_eq!(render(Err("disk full".into())), "error: disk full");
}

// ── Inspecting ──

#[test]
fn is_small_number_checks_both() {
    assert!(is_small_number("9"));
    assert!(is_small_number("-9"));
    assert!(is_small_number("0"));
    assert!(!is_small_number("10"));
    assert!(!is_small_number("-2147483648"), "i32::MIN: careful with abs()");
    assert!(!is_small_number("one"));
}

#[test]
fn failed_because_empty_looks_at_kind() {
    let r: Result<i32, ParseIntError> = "".parse();
    assert!(failed_because_empty(&r));
    assert!(r.is_err(), "still ours afterwards");
    assert!(!failed_because_empty(&"x".parse()));
    assert!(!failed_because_empty(&Ok(0)));
}

#[test]
fn parse_logging_logs_and_passes_through() {
    let mut log = Vec::new();
    assert_eq!(parse_logging("5", |m| log.push(m)), Ok(5));
    assert_eq!(parse_logging("five", |m| log.push(m)), Err(invalid_digit()));
    assert_eq!(log, ["ok 5", "err invalid digit found in string"]);
}

// ── To Option and borrowing ──

#[test]
fn ok_value_keeps_the_value() {
    assert_eq!(ok_value(Ok(1)), Some(1));
    assert_eq!(ok_value(Err("e".into())), None);
}

#[test]
fn err_value_keeps_the_error() {
    assert_eq!(err_value(Ok(1)), None);
    assert_eq!(err_value(Err("e".into())), Some("e".to_string()));
}

#[test]
fn borrow_message_borrows_both_sides() {
    let good: Result<String, String> = Ok("fine".into());
    let bad: Result<String, String> = Err("broken".into());
    assert_eq!(borrow_message(&good), Ok("fine"));
    assert_eq!(borrow_message(&bad), Err("broken"));
    assert_eq!(good.as_deref(), Ok("fine"), "still ours");
}

// ── The ? operator ──

#[test]
fn parse_and_add_propagates_first_error() {
    assert_eq!(parse_and_add("2", "3"), Ok(5));
    assert_eq!(parse_and_add("x", ""), Err(invalid_digit()), "a fails first");
    assert_eq!(parse_and_add("1", ""), Err(empty()));
}

#[test]
fn parse_port_converts_errors() {
    assert_eq!(parse_port("8080"), Ok(8080));
    assert_eq!(parse_port(" 1 "), Ok(1));
    assert_eq!(parse_port("65535"), Ok(65535));
    assert_eq!(parse_port("0"), Err(ConfigError::OutOfRange(0)));
    assert_eq!(parse_port("70000"), Err(ConfigError::OutOfRange(70000)));
    assert_eq!(parse_port("-80"), Err(ConfigError::OutOfRange(-80)));
    assert_eq!(parse_port("http"), Err(ConfigError::BadNumber(invalid_digit())));
    assert!(
        matches!(parse_port("99999999999999999999"), Err(ConfigError::BadNumber(_))),
        "too big for an i64: that's a parse error, not OutOfRange"
    );
}

#[test]
fn parse_setting_reports_each_failure() {
    assert_eq!(parse_setting(" depth = 3 "), Ok(("depth".to_string(), 3)));
    assert_eq!(parse_setting("x=-12"), Ok(("x".to_string(), -12)));
    assert_eq!(parse_setting("depth"), Err(ConfigError::MissingEquals));
    assert_eq!(parse_setting("  = 4"), Err(ConfigError::EmptyKey));
    assert_eq!(parse_setting("depth ="), Err(ConfigError::BadNumber(empty())));
    assert_eq!(parse_setting("a = 1 = 2"), Err(ConfigError::BadNumber(invalid_digit())));
}

// ── Result<(), E> ──

#[test]
fn check_all_below_reports_first_offender() {
    assert_eq!(check_all_below(&[1, 5, 3], 5), Ok(()));
    assert_eq!(check_all_below(&[], 0), Ok(()));
    assert_eq!(check_all_below(&[1, 9, 7], 5), Err(ConfigError::OutOfRange(9)));
}

#[test]
fn apply_settings_stops_at_first_error() {
    let mut config = HashMap::new();
    assert_eq!(apply_settings(&["a = 1", "b = 2", "a = 3"], &mut config), Ok(()));
    assert_eq!(config, HashMap::from([("a".to_string(), 3), ("b".to_string(), 2)]));

    let mut config = HashMap::new();
    let r = apply_settings(&["a = 1", "oops", "c = 3"], &mut config);
    assert_eq!(r, Err(ConfigError::MissingEquals));
    assert_eq!(config, HashMap::from([("a".to_string(), 1)]), "earlier lines stay, later don't run");
}
