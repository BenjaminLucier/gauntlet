// Tests for Chars and UTF-8 — shared by template.rs, solution.rs and your attempt.
use super::*;

// ── Lengths ──

#[test]
fn byte_and_char_len_differ_beyond_ascii() {
    assert_eq!(byte_and_char_len("hello"), (5, 5));
    assert_eq!(byte_and_char_len("café"), (5, 4));
    assert_eq!(byte_and_char_len("日本"), (6, 2));
    assert_eq!(byte_and_char_len("🦀"), (4, 1));
    assert_eq!(byte_and_char_len(""), (0, 0));
}

#[test]
fn utf8_width_one_to_four() {
    assert_eq!(utf8_width('a'), 1);
    assert_eq!(utf8_width('é'), 2);
    assert_eq!(utf8_width('日'), 3);
    assert_eq!(utf8_width('🦀'), 4);
}

#[test]
fn is_plain_ascii_checks_every_char() {
    assert!(is_plain_ascii("Hello, world! 123"));
    assert!(is_plain_ascii(""));
    assert!(!is_plain_ascii("naïve"));
}

// ── Indexing by character ──

#[test]
fn char_at_counts_characters_not_bytes() {
    assert_eq!(char_at("日本語", 1), Some('本'));
    assert_eq!(char_at("a🦀b", 2), Some('b'));
    assert_eq!(char_at("abc", 3), None);
}

#[test]
fn char_prefix_never_splits_a_char() {
    assert_eq!(char_prefix("héllo", 2), "hé");
    assert_eq!(char_prefix("日本語", 1), "日");
    assert_eq!(char_prefix("ab", 5), "ab");
    assert_eq!(char_prefix("ab", 0), "");
}

#[test]
fn char_slice_by_character_index() {
    assert_eq!(char_slice("héllo", 1, 3), Some("él"));
    assert_eq!(char_slice("日本語", 0, 3), Some("日本語"), "end may equal the length");
    assert_eq!(char_slice("🦀🦀🦀", 2, 2), Some(""));
    assert_eq!(char_slice("abc", 1, 4), None);
    assert_eq!(char_slice("abc", 2, 1), None, "backwards");
}

#[test]
fn byte_slice_rejects_bad_boundaries() {
    assert_eq!(byte_slice("héllo", 0, 3), Some("hé"));
    assert_eq!(byte_slice("héllo", 0, 2), None, "byte 2 is inside é");
    assert_eq!(byte_slice("abc", 1, 9), None);
    assert_eq!(byte_slice("abc", 3, 3), Some(""));
}

#[test]
fn truncate_bytes_backs_off_to_a_boundary() {
    assert_eq!(truncate_bytes("hello", 3), "hel");
    assert_eq!(truncate_bytes("日本語", 4), "日", "byte 4 is inside 本");
    assert_eq!(truncate_bytes("日本語", 6), "日本");
    assert_eq!(truncate_bytes("🦀", 3), "");
    assert_eq!(truncate_bytes("short", 100), "short");
    assert_eq!(truncate_bytes("é", 2), "é", "exactly the length");
    assert_eq!(truncate_bytes("", 0), "");
}

// ── Classifying and converting chars ──

#[test]
fn digit_value_and_digit_char_round_trip() {
    assert_eq!(digit_value('7', 10), Some(7));
    assert_eq!(digit_value('f', 16), Some(15));
    assert_eq!(digit_value('F', 16), Some(15));
    assert_eq!(digit_value('9', 8), None, "not an octal digit");
    assert_eq!(digit_char(11, 16), Some('b'));
    assert_eq!(digit_char(1, 2), Some('1'));
    assert_eq!(digit_char(10, 10), None);
}

#[test]
fn sum_digits_ignores_the_rest() {
    assert_eq!(sum_digits("a1b2c3"), 6);
    assert_eq!(sum_digits("no digits"), 0);
    assert_eq!(sum_digits("٣3"), 3, "to_digit(10) only accepts ASCII 0-9");
}

#[test]
fn alnum_only_keeps_unicode_letters() {
    assert_eq!(alnum_only("Hello, world!"), "Helloworld");
    assert_eq!(alnum_only("日本 é-2!"), "日本é2");
    assert_eq!(alnum_only("🦀 !"), "");
}

#[test]
fn ascii_counts_by_class() {
    assert_eq!(ascii_counts("ab 12\tc!"), (3, 2, 2));
    assert_eq!(ascii_counts("é 日"), (0, 0, 1), "non-ASCII letters don't count");
    assert_eq!(ascii_counts(""), (0, 0, 0));
}

#[test]
fn rot13_rotates_letters_only() {
    assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
    assert_eq!(rot13("xyz ABC"), "klm NOP");
    assert_eq!(rot13(&rot13("Round trip é")), "Round trip é");
}

// ── Bytes and code points ──

#[test]
fn utf8_bytes_encodes() {
    assert_eq!(utf8_bytes('A'), [0x41]);
    assert_eq!(utf8_bytes('é'), [0xC3, 0xA9]);
    assert_eq!(utf8_bytes('🦀'), [0xF0, 0x9F, 0xA6, 0x80]);
}

#[test]
fn decode_borrows_or_fails() {
    assert_eq!(decode(b"plain"), Ok("plain"));
    assert_eq!(decode(&[0xE6, 0x97, 0xA5]), Ok("日"));
    let err = decode(&[b'o', b'k', 0xFF, b'!']).expect_err("0xFF is never valid UTF-8");
    assert_eq!(err.valid_up_to(), 2);
}

#[test]
fn decode_lossy_replaces_bad_bytes() {
    assert_eq!(decode_lossy(b"fine"), "fine");
    assert_eq!(decode_lossy(&[b'a', 0xFF, b'b']), "a\u{FFFD}b");
    assert_eq!(decode_lossy(&[0xE6, 0x97]), "\u{FFFD}", "a truncated character");
}

#[test]
fn code_points_and_back() {
    assert_eq!(code_points("Aé🦀"), [0x41, 0xE9, 0x1F980]);
    assert_eq!(from_code_point(0x65E5), Some('日'));
    assert_eq!(from_code_point(0xD800), None, "a surrogate isn't a char");
    assert_eq!(from_code_point(0x11_0000), None, "beyond Unicode");
}

#[test]
fn latin1_to_string_maps_bytes_to_code_points() {
    assert_eq!(latin1_to_string(b"plain"), "plain");
    assert_eq!(latin1_to_string(&[0x63, 0x61, 0x66, 0xE9]), "café");
    assert_eq!(latin1_to_string(&[0xFF]), "ÿ", "0xFF is invalid UTF-8 but valid Latin-1");
}

#[test]
fn reverse_chars_by_code_point() {
    assert_eq!(reverse_chars("hello"), "olleh");
    assert_eq!(reverse_chars("日本🦀"), "🦀本日");
    assert_eq!(reverse_chars(""), "");
    // A combining accent is its own char, so it moves to the other side.
    assert_eq!(reverse_chars("e\u{301}x"), "x\u{301}e");
}
