// Tests for Building Strings and Cow — shared by template.rs, solution.rs and your attempt.
use super::*;

// ── Pushing, inserting and removing ──

#[test]
fn greeting_builds_the_sentence() {
    let s = greeting("Ferris");
    assert_eq!(s, "Hello, Ferris!");
    assert_eq!(greeting(""), "Hello, !");
    assert_eq!(greeting("日本"), "Hello, 日本!");
}

#[test]
fn bracket_with_any_chars() {
    assert_eq!(bracket("x", '[', ']'), "[x]");
    assert_eq!(bracket("x", '«', '»'), "«x»");
    assert_eq!(bracket("", '(', ')'), "()");
}

#[test]
fn quote_in_place_wraps_in_quotes() {
    let mut s = String::from("hi");
    quote_in_place(&mut s);
    assert_eq!(s, "\"hi\"");
    let mut empty = String::new();
    quote_in_place(&mut empty);
    assert_eq!(empty, "\"\"");
}

#[test]
fn prefix_in_place_inserts_at_the_front() {
    let mut s = String::from("\"hi\"");
    prefix_in_place(&mut s, "say ");
    assert_eq!(s, "say \"hi\"");
    prefix_in_place(&mut s, "");
    assert_eq!(s, "say \"hi\"");
}

#[test]
fn append_words_extends() {
    let mut s = String::from(">");
    append_words(&mut s, &["a", "bc", "", "日"]);
    assert_eq!(s, ">abc日");
    append_words(&mut s, &[]);
    assert_eq!(s, ">abc日");
}

#[test]
fn remove_whitespace_edits_in_place() {
    let mut s = String::from(" a b\tc\nd ");
    remove_whitespace(&mut s);
    assert_eq!(s, "abcd");
    let mut wide = String::from("日\u{3000}本 é");
    remove_whitespace(&mut wide);
    assert_eq!(wide, "日本é", "U+3000 (ideographic space) is whitespace too");
    let mut blank = String::from(" \t ");
    remove_whitespace(&mut blank);
    assert_eq!(blank, "");
}

// ── Formatting ──

#[test]
fn hex_string_pads_each_byte() {
    assert_eq!(hex_string(&[0, 255, 16, 9]), "00ff1009");
    assert_eq!(hex_string(&[]), "");
    assert_eq!(hex_string("é".as_bytes()), "c3a9");
}

#[test]
fn numbered_list_starts_at_one() {
    assert_eq!(numbered_list(&["eggs", "milk"]), "1. eggs\n2. milk\n");
    assert_eq!(numbered_list(&[]), "");
}

#[test]
fn format_point_rounds_to_two_places() {
    assert_eq!(format_point(1.0, -2.5), "(1.00, -2.50)");
    assert_eq!(format_point(1.23456, 0.005), "(1.23, 0.01)");
}

#[test]
fn right_align_pads_but_never_cuts() {
    assert_eq!(right_align("42", 5), "   42");
    assert_eq!(right_align("toolong", 3), "toolong");
    assert_eq!(right_align("é", 3), "  é", "width counts chars, not bytes");
}

// ── Joining and collecting ──

#[test]
fn concat_parts_glues_without_separator() {
    assert_eq!(concat_parts(&["ab", "", "cd"]), "abcd");
    assert_eq!(concat_parts(&[]), "");
}

#[test]
fn join_path_puts_slashes_between() {
    assert_eq!(join_path(&["usr", "local", "bin"]), "usr/local/bin");
    assert_eq!(join_path(&["one"]), "one", "no separator for a single part");
}

#[test]
fn from_chars_builds() {
    assert_eq!(from_chars(&['r', 'u', 's', 't']), "rust");
    assert_eq!(from_chars(&['日', '🦀']), "日🦀");
    assert_eq!(from_chars(&[]), "");
}

// ── Cow ──

#[test]
fn escape_html_borrows_clean_input() {
    let clean = "just text";
    let result = escape_html(clean);
    assert!(matches!(result, Cow::Borrowed(_)), "no special chars: no allocation");
    assert_eq!(result, "just text");
    assert!(matches!(escape_html(""), Cow::Borrowed("")));
}

#[test]
fn escape_html_escapes_every_special() {
    let result = escape_html(r#"<a href="x">Tom & Jerry</a>"#);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(
        result,
        "&lt;a href=&quot;x&quot;&gt;Tom &amp; Jerry&lt;/a&gt;"
    );
    assert_eq!(escape_html("café & 日本"), "café &amp; 日本");
}

#[test]
fn ensure_trailing_slash_allocates_only_when_missing() {
    let has = ensure_trailing_slash("src/");
    assert!(matches!(has, Cow::Borrowed("src/")));
    let missing = ensure_trailing_slash("src");
    assert!(matches!(missing, Cow::Owned(_)));
    assert_eq!(missing, "src/");
    assert_eq!(ensure_trailing_slash(""), "/");
}

#[test]
fn lowercase_if_needed_checks_unicode_case() {
    assert!(matches!(lowercase_if_needed("already lower 123"), Cow::Borrowed(_)));
    assert!(matches!(lowercase_if_needed("日本"), Cow::Borrowed(_)), "no case at all");
    let changed = lowercase_if_needed("Hello ÉCOLE");
    assert!(matches!(changed, Cow::Owned(_)));
    assert_eq!(changed, "hello école");
}

#[test]
fn lowercase_if_needed_catches_titlecase() {
    let changed = lowercase_if_needed("ǅemal");
    assert!(matches!(changed, Cow::Owned(_)), "'ǅ' is title-case, not upper-case");
    assert_eq!(changed, "ǆemal");
}

#[test]
fn normalize_newlines_converts_crlf() {
    assert!(matches!(normalize_newlines("a\nb\n"), Cow::Borrowed("a\nb\n")));
    let fixed = normalize_newlines("a\r\nb\r\n");
    assert!(matches!(fixed, Cow::Owned(_)));
    assert_eq!(fixed, "a\nb\n");
    assert_eq!(normalize_newlines("lone\rcr"), "lone\rcr", "a lone \\r stays");
}

#[test]
fn escape_all_returns_owned_strings() {
    let owned: Vec<String> = escape_all(&["a<b", "plain", ""]);
    assert_eq!(owned, ["a&lt;b", "plain", ""]);
}

#[test]
fn capitalize_clones_only_on_change() {
    let mut already = Cow::Borrowed("Hello");
    capitalize(&mut already);
    assert!(matches!(already, Cow::Borrowed("Hello")), "nothing to change, still borrowed");

    let mut lower = Cow::Borrowed("hello");
    capitalize(&mut lower);
    assert!(matches!(lower, Cow::Owned(_)));
    assert_eq!(lower, "Hello");

    let mut accented: Cow<str> = Cow::Owned(String::from("été"));
    capitalize(&mut accented);
    assert_eq!(accented, "Été", "the first char may be more than one byte");

    let mut empty = Cow::Borrowed("");
    capitalize(&mut empty);
    assert_eq!(empty, "");
}
