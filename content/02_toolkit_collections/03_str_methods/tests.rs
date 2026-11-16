// Tests for str Methods — shared by template.rs, solution.rs and your attempt.
use super::*;

const NONE: &[&str] = &[];

// ── Splitting ──

#[test]
fn words_skips_any_whitespace() {
    assert_eq!(words("  the quick\tbrown\n fox "), ["the", "quick", "brown", "fox"]);
    assert_eq!(words("   "), NONE);
    assert_eq!(words(""), NONE);
}

#[test]
fn csv_fields_trims_and_keeps_empties() {
    assert_eq!(csv_fields("a, b ,c"), ["a", "b", "c"]);
    assert_eq!(csv_fields("x,,y"), ["x", "", "y"]);
    assert_eq!(csv_fields(""), [""], "one empty field");
}

#[test]
fn key_value_splits_at_the_first_equals() {
    assert_eq!(key_value("name = Ferris"), Some(("name", "Ferris")));
    assert_eq!(key_value("expr=a=b"), Some(("expr", "a=b")));
    assert_eq!(key_value("flag="), Some(("flag", "")));
    assert_eq!(key_value("no equals"), None);
}

#[test]
fn extension_uses_the_last_dot() {
    assert_eq!(extension("main.rs"), Some("rs"));
    assert_eq!(extension("archive.tar.gz"), Some("gz"));
    assert_eq!(extension("Makefile"), None);
    assert_eq!(extension(".bashrc"), None, "a leading dot isn't an extension");
    assert_eq!(extension("notes."), Some(""));
}

#[test]
fn first_fields_limits_the_pieces() {
    assert_eq!(first_fields("a:b:c:d", 2), ["a", "b:c:d"]);
    assert_eq!(first_fields("a:b:c:d", 10), ["a", "b", "c", "d"]);
    assert_eq!(first_fields("a:b", 1), ["a:b"]);
    assert_eq!(first_fields("a:b", 0), NONE, "zero pieces asked for");
}

#[test]
fn non_blank_lines_handles_crlf() {
    assert_eq!(non_blank_lines("one\n\n  \ntwo\r\nthree\n"), ["one", "two", "three"]);
    assert_eq!(non_blank_lines(""), NONE);
}

#[test]
fn tokens_splits_on_several_chars() {
    assert_eq!(tokens("a,b; c  d;"), ["a", "b", "c", "d"]);
    assert_eq!(tokens(",;, "), NONE);
}

// ── Trimming and stripping ──

#[test]
fn unquote_needs_both_quotes() {
    assert_eq!(unquote("\"hi\""), "hi");
    assert_eq!(unquote("\"\"\"\""), "\"\"", "only one pair comes off");
    assert_eq!(unquote("\"open"), "\"open");
    assert_eq!(unquote("plain"), "plain");
    assert_eq!(unquote("\""), "\"", "a lone quote isn't a pair");
}

#[test]
fn trim_leading_zeros_keeps_one_zero() {
    assert_eq!(trim_leading_zeros("007"), "7");
    assert_eq!(trim_leading_zeros("100"), "100");
    assert_eq!(trim_leading_zeros("0000"), "0");
    assert_eq!(trim_leading_zeros(""), "");
}

#[test]
fn trim_punctuation_both_ends_only() {
    assert_eq!(trim_punctuation("...wait, what?!"), "wait, what");
    assert_eq!(trim_punctuation("\"¿qué?\""), "¿qué", "¿ isn't ASCII punctuation");
    assert_eq!(trim_punctuation("!!!"), "");
}

#[test]
fn strip_comment_and_trailing_space() {
    assert_eq!(strip_comment("x = 1   # the answer"), "x = 1");
    assert_eq!(strip_comment("# whole line"), "");
    assert_eq!(strip_comment("no comment  "), "no comment");
}

// ── Searching ──

#[test]
fn starts_with_digit_checks_the_first_char() {
    assert!(starts_with_digit("3 apples"));
    assert!(!starts_with_digit("three"));
    assert!(!starts_with_digit(""));
    assert!(!starts_with_digit("٣"), "Arabic-Indic digits aren't ASCII");
}

#[test]
fn base_name_after_the_last_slash() {
    assert_eq!(base_name("/usr/local/bin/cargo"), "cargo");
    assert_eq!(base_name("file.txt"), "file.txt");
    assert_eq!(base_name("dir/"), "");
    assert_eq!(base_name("日本/語.txt"), "語.txt");
}

#[test]
fn count_occurrences_does_not_overlap() {
    assert_eq!(count_occurrences("banana", "an"), 2);
    assert_eq!(count_occurrences("aaaa", "aa"), 2, "non-overlapping");
    assert_eq!(count_occurrences("rust", "go"), 0);
}

#[test]
fn vowel_offsets_are_byte_offsets() {
    assert_eq!(vowel_offsets("Rust is fun"), [1, 5, 9]);
    assert_eq!(vowel_offsets("héllo"), [5], "é is two bytes and not an ASCII vowel");
    assert_eq!(vowel_offsets("xyz"), Vec::<usize>::new());
}

// ── Transforming ──

#[test]
fn censor_replaces_every_occurrence() {
    assert_eq!(censor("darn it, darn", "darn"), "*** it, ***");
    assert_eq!(censor("clean", "darn"), "clean");
}

#[test]
fn replace_first_stops_after_one() {
    assert_eq!(replace_first("a-b-c", "-", "+"), "a+b-c");
    assert_eq!(replace_first("abc", "x", "y"), "abc");
}

#[test]
fn shout_uses_unicode_case_mapping() {
    assert_eq!(shout("hello"), "HELLO!");
    assert_eq!(shout("straße"), "STRASSE!", "ß upper-cases to two letters");
    assert_eq!(shout(""), "!");
}

#[test]
fn same_command_ignores_ascii_case() {
    assert!(same_command("QUIT", "quit"));
    assert!(same_command("Help", "hELP"));
    assert!(!same_command("quit", "quit "));
}

#[test]
fn underline_counts_chars() {
    assert_eq!(underline("Intro"), "=====");
    assert_eq!(underline("Café"), "====", "4 chars, 5 bytes");
    assert_eq!(underline(""), "");
}

// ── Parsing ──

#[test]
fn parse_numbers_collects_into_result() {
    assert_eq!(parse_numbers("1 -2  30"), Ok(vec![1, -2, 30]));
    assert_eq!(parse_numbers(""), Ok(vec![]));
    assert!(parse_numbers("1 two 3").is_err());
    assert!(parse_numbers("99999999999").is_err(), "overflow is an error");
}

#[test]
fn parse_point_trims_and_rejects_junk() {
    assert_eq!(parse_point("3,4"), Some((3, 4)));
    assert_eq!(parse_point(" 3 , -4 "), Some((3, -4)));
    assert_eq!(parse_point("3;4"), None);
    assert_eq!(parse_point("3,x"), None);
    assert_eq!(parse_point("1,2,3"), None);
}
