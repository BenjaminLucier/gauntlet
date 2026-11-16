// Tests for Encode and Decode Strings — shared by template.rs, solution.rs and your attempt.
use super::*;

fn roundtrip(strs: &[&str]) {
    let encoded = encode(strs);
    let decoded = decode(&encoded);
    if strs.len() <= 50 {
        assert_eq!(decoded, Ok(strs.to_vec()), "strs = {strs:?}, encoded = {encoded:?}");
    } else {
        let decoded = decoded.expect("decoding our own encoding must succeed");
        assert_eq!(decoded.len(), strs.len(), "wrong number of strings decoded");
        assert!(decoded == strs, "round trip of a {}-string list failed", strs.len());
    }
}

fn roundtrip_owned(strs: &[String]) {
    let refs: Vec<&str> = strs.iter().map(String::as_str).collect();
    // Encoding owned Strings and borrowed &strs must agree.
    assert_eq!(encode(strs), encode(&refs));
    roundtrip(&refs);
}

/// Obviously-correct reference encoder for the specified format.
fn brute_encode(strs: &[String]) -> String {
    strs.iter().map(|s| format!("{}#{s}", s.len())).collect()
}

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// Characters chosen to break naive formats: delimiters, digits, escapes,
/// whitespace, NUL, and multi-byte UTF-8.
const NASTY: &[char] = &[
    'a', 'b', 'z', '0', '1', '9', '#', ',', ';', ':', '|', '\\', '"', ' ', '\n', '\t', '\0',
    'é', 'ß', '日', '本', '🦀', '\u{200b}', '\u{ffff}',
];

#[test]
fn example_format() {
    assert_eq!(encode(&["ab", "", "#1"]), "2#ab0#2##1");
    assert_eq!(decode("2#ab0#2##1"), Ok(vec!["ab", "", "#1"]));
}

#[test]
fn example_1() {
    roundtrip(&["neet", "code", "love", "you"]);
}

#[test]
fn example_2() {
    roundtrip(&["we", "say", ":", "yes"]);
}

#[test]
fn example_byte_lengths() {
    let owned = vec![String::from("日本"), String::from("🦀")];
    assert_eq!(encode(&owned), "6#日本4#🦀");
}

#[test]
fn example_3_empty_list() {
    roundtrip(&[]);
    assert_eq!(encode::<&str>(&[]), "");
    assert_eq!(decode(""), Ok(vec![]));
}

#[test]
fn example_errors() {
    assert_eq!(decode("3#ab"), Err(DecodeError::Truncated));
    assert_eq!(decode("12"), Err(DecodeError::MissingSeparator));
    assert_eq!(decode("x#"), Err(DecodeError::InvalidLength));
    assert_eq!(decode("1#é"), Err(DecodeError::NotCharBoundary));
}

#[test]
fn single_empty_string_is_not_empty_list() {
    // Trap: [] and [""] must encode differently.
    roundtrip(&[""]);
    assert_ne!(encode::<&str>(&[]), encode(&[""]));
    assert_eq!(encode(&[""]), "0#");
}

#[test]
fn several_empty_strings() {
    roundtrip(&["", "", ""]);
    roundtrip(&["", "a", ""]);
}

#[test]
fn contains_hash_delimiter() {
    roundtrip(&["#", "##", "a#b", "#3", "3#abc"]);
}

#[test]
fn looks_like_length_prefix() {
    // Trap: strings that look like your own encoding.
    roundtrip(&["4#abcd", "0#", "12#", "1#1#1#"]);
}

#[test]
fn digits_only() {
    roundtrip(&["123", "0", "007", "99999999999999999999"]);
}

#[test]
fn common_separators() {
    roundtrip(&[",", ";", "|", "\n", " ", "\t", "\\", "\"", "a,b", "a;b"]);
}

#[test]
fn nul_and_control_chars() {
    roundtrip(&["\0", "a\0b", "\u{1}\u{2}\u{7f}"]);
}

#[test]
fn unicode() {
    // Trap: mixing up byte length and char count.
    roundtrip(&["日本語", "héllo", "🦀🦀", "a🦀b", "ß#é"]);
    assert_eq!(encode(&["é"]), "2#é");
}

#[test]
fn length_ten_and_more() {
    // Trap: single-digit length prefixes.
    let long = "y".repeat(123);
    roundtrip(&["abcdefghij", "x", &long]);
    assert!(encode(&[&long]).starts_with("123#"));
}

#[test]
fn distinct_lists_encode_differently() {
    let lists: Vec<Vec<&str>> = vec![
        vec!["a"],
        vec!["a", "b"],
        vec!["ab"],
        vec!["", "ab"],
        vec!["ab", ""],
    ];
    let encodings: Vec<String> = lists.iter().map(|l| encode(l)).collect();
    for i in 0..encodings.len() {
        for j in i + 1..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "{:?} and {:?} encode the same",
                lists[i], lists[j]
            );
        }
    }
}

#[test]
fn decode_accepts_leading_zeros() {
    assert_eq!(decode("002#ab00#"), Ok(vec!["ab", ""]));
}

#[test]
fn decode_missing_separator() {
    assert_eq!(decode("5"), Err(DecodeError::MissingSeparator));
    assert_eq!(decode("abc"), Err(DecodeError::MissingSeparator));
    // The first entry is fine; the error is in the second.
    assert_eq!(decode("1#a7"), Err(DecodeError::MissingSeparator));
}

#[test]
fn decode_invalid_length() {
    assert_eq!(decode("#"), Err(DecodeError::InvalidLength)); // empty length
    assert_eq!(decode("#abc"), Err(DecodeError::InvalidLength));
    assert_eq!(decode("-1#a"), Err(DecodeError::InvalidLength));
    assert_eq!(decode(" 1#a"), Err(DecodeError::InvalidLength));
    assert_eq!(decode("1 #a"), Err(DecodeError::InvalidLength));
    assert_eq!(decode("٣#abc"), Err(DecodeError::InvalidLength)); // non-ASCII digit
    // Doesn't fit in a usize.
    assert_eq!(
        decode("99999999999999999999999999#"),
        Err(DecodeError::InvalidLength)
    );
}

#[test]
fn plus_sign_is_not_a_digit() {
    // Trap: `str::parse::<usize>` accepts "+1".
    assert_eq!(decode("+1#a"), Err(DecodeError::InvalidLength));
}

#[test]
fn decode_truncated() {
    assert_eq!(decode("1#"), Err(DecodeError::Truncated));
    assert_eq!(decode("2#ab3#cd"), Err(DecodeError::Truncated));
    assert_eq!(decode("4#🦀"), Ok(vec!["🦀"]));
    assert_eq!(decode("5#🦀"), Err(DecodeError::Truncated));
}

#[test]
fn decode_not_char_boundary() {
    assert_eq!(decode("1#🦀"), Err(DecodeError::NotCharBoundary));
    assert_eq!(decode("3#🦀"), Err(DecodeError::NotCharBoundary));
    assert_eq!(decode("2#a日"), Err(DecodeError::NotCharBoundary));
}

#[test]
fn decoded_slices_borrow_from_input() {
    let encoded = String::from("3#abc0#2#xy");
    let decoded = decode(&encoded).expect("valid encoding");
    assert_eq!(decoded, ["abc", "", "xy"]);
    let range = encoded.as_bytes().as_ptr_range();
    for piece in &decoded {
        assert!(
            range.contains(&piece.as_ptr()) || piece.is_empty(),
            "{piece:?} was copied instead of borrowed"
        );
    }
}

#[test]
fn encode_accepts_strings_and_strs() {
    let owned: Vec<String> = vec!["x".into(), "yz".into()];
    let borrowed: [&str; 2] = ["x", "yz"];
    assert_eq!(encode(&owned), "1#x2#yz");
    assert_eq!(encode(&borrowed), "1#x2#yz");
    assert_eq!(encode(&owned[..1]), "1#x");
}

#[test]
fn randomized_roundtrip() {
    let mut rng = Rng(0x0F0F_1234_ABCD_9876);
    for _ in 0..500 {
        let n = rng.below(8) as usize;
        let strs: Vec<String> = (0..n)
            .map(|_| {
                let len = rng.below(12) as usize;
                (0..len)
                    .map(|_| NASTY[rng.below(NASTY.len() as u64) as usize])
                    .collect()
            })
            .collect();
        assert_eq!(encode(&strs), brute_encode(&strs), "strs = {strs:?}");
        roundtrip_owned(&strs);
    }
}

#[test]
fn randomized_garbage_never_panics() {
    // Decoding arbitrary input either succeeds and re-encodes to an equivalent
    // list, or reports an error — it must never panic.
    let alphabet = ['0', '1', '2', '3', '#', 'a', 'é', '🦀', '+'];
    let mut rng = Rng(0x5EED_0000_1111_2222);
    for _ in 0..1000 {
        let len = rng.below(10) as usize;
        let input: String = (0..len)
            .map(|_| alphabet[rng.below(alphabet.len() as u64) as usize])
            .collect();
        if let Ok(decoded) = decode(&input) {
            assert_eq!(
                decode(&encode(&decoded)),
                Ok(decoded.clone()),
                "input = {input:?}"
            );
        }
    }
}

#[test]
fn max_size_strings() {
    let strs: Vec<String> = (0..199)
        .map(|i| {
            (0..999)
                .map(|j| NASTY[(i * 31 + j * 17) % NASTY.len()])
                .collect()
        })
        .collect();
    roundtrip_owned(&strs);
}

#[test]
fn max_size_list() {
    // 2 * 10^4 strings of up to 1000 bytes, about 15 MB encoded. Copying the
    // rest of the input for every string decoded, or the output so far for
    // every string encoded, moves about 10^11 bytes; a single pass moves
    // 1.5 * 10^7.
    let n = 20_000;
    let strs: Vec<String> = (0..n)
        .map(|i| match i % 4 {
            0 => String::new(),
            1 => format!("{i}#").repeat(150),
            2 => "🦀#,".repeat(166 - i % 7),
            _ => "x".repeat(999 - i % 40),
        })
        .collect();
    let encoded = encode(&strs);
    let decoded = decode(&encoded).unwrap_or_else(|e| panic!("n = {n}: decoding our own encoding failed: {e:?}"));
    assert_eq!(decoded.len(), n, "wrong number of strings decoded");
    assert!(decoded == strs, "round trip of a {n}-string list failed");
}
