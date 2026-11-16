// Tests for Partition Labels — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Try every set of cut positions; keep the valid partition with most pieces.
fn brute_partition_lengths(s: &str) -> Vec<usize> {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return Vec::new();
    }
    let mut best = vec![n];
    for mask in 0u32..1 << (n - 1) {
        let mut pieces: Vec<&[u8]> = Vec::new();
        let mut start = 0;
        for i in 0..n - 1 {
            if (mask >> i) & 1 == 1 {
                pieces.push(&b[start..=i]);
                start = i + 1;
            }
        }
        pieces.push(&b[start..]);
        let valid = pieces.iter().enumerate().all(|(p, piece)| {
            pieces[p + 1..].iter().all(|other| !piece.iter().any(|c| other.contains(c)))
        });
        if valid && pieces.len() > best.len() {
            best = pieces.iter().map(|p| p.len()).collect();
        }
    }
    best
}

fn lengths(s: &str) -> Vec<usize> {
    let pieces = partition_labels(s);
    assert_eq!(pieces.concat(), s, "pieces must concatenate back to the input");
    pieces.iter().map(|p| p.len()).collect()
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

#[test]
fn example_1() {
    assert_eq!(
        partition_labels("ababcbacadefegdehijhklij"),
        ["ababcbaca", "defegde", "hijhklij"]
    );
}

#[test]
fn example_2() {
    assert_eq!(partition_labels("eccbbbbdec"), ["eccbbbbdec"]);
}

#[test]
fn empty_string_has_no_pieces() {
    assert!(partition_labels("").is_empty());
}

#[test]
fn single_char() {
    assert_eq!(partition_labels("z"), ["z"]);
}

#[test]
fn all_distinct() {
    assert_eq!(partition_labels("abcdef"), ["a", "b", "c", "d", "e", "f"]);
}

#[test]
fn all_same() {
    assert_eq!(partition_labels("aaaa"), ["aaaa"]);
}

#[test]
fn first_and_last_equal() {
    assert_eq!(partition_labels("abcdea"), ["abcdea"]);
}

#[test]
fn nested_extension() {
    // 'a' ends at 2, but 'b' (seen inside) ends at 4, then 'c' ends at 5.
    assert_eq!(partition_labels("abacbcd"), ["abacbc", "d"]);
}

#[test]
fn trailing_single_piece() {
    // Trap: forgetting to emit the final piece.
    assert_eq!(partition_labels("aab"), ["aa", "b"]);
}

#[test]
fn repeated_blocks() {
    assert_eq!(partition_labels("aabbccaa"), ["aabbccaa"]);
    assert_eq!(partition_labels("aabbcc"), ["aa", "bb", "cc"]);
}

#[test]
fn other_ascii_characters() {
    // Uppercase, digits and punctuation are distinct characters too.
    assert_eq!(partition_labels("aA1!1"), ["a", "A", "1!1"]);
    assert_eq!(partition_labels("a b a"), ["a b a"]);
    assert_eq!(partition_labels("Zz"), ["Z", "z"]);
}

#[test]
fn pieces_borrow_from_input() {
    // The pieces point into the original string — no copies.
    let owned = String::from("abba-cd-c");
    let pieces = partition_labels(&owned);
    assert_eq!(pieces, ["abba", "-cd-c"]);
    let base = owned.as_ptr() as usize;
    assert_eq!(pieces[0].as_ptr() as usize, base);
    assert_eq!(pieces[1].as_ptr() as usize, base + 4);
}

#[test]
fn random_against_brute_force() {
    let mut rng = Rng(0x7777_1111_3333_9999);
    for _ in 0..400 {
        let n = rng.below(13) as usize;
        let k = 1 + rng.below(5) as u8;
        let s: String = (0..n).map(|_| char::from(b'a' + rng.below(u64::from(k)) as u8)).collect();
        assert_eq!(lengths(&s), brute_partition_lengths(&s), "s: {s:?}");
    }
}

#[test]
fn large_input() {
    let s: String = (0..500).map(|i| char::from(b'a' + (i / 20) as u8 % 26)).collect();
    // 25 blocks of 20 equal letters ('a'..='y').
    assert_eq!(lengths(&s), vec![20; 25]);
}

#[test]
fn large_one_piece() {
    let mut s: String = (0..498).map(|i| char::from(b'a' + (i % 26) as u8)).collect();
    s.insert(0, 'z');
    s.push('z');
    // The first and last letters are equal, so it is all one piece.
    assert_eq!(partition_labels(&s), [s.as_str()]);
}

#[test]
fn large_input_in_95_runs() {
    // 7 * 10^4 bytes in 95 runs, one per printable ASCII character, in a
    // shuffled order with random lengths. Looking up each byte's last
    // occurrence by scanning from the end costs ~n^2 / 2 = 2.5 * 10^9 byte
    // comparisons; a table of last positions is one pass.
    let n = 70_000;
    let mut rng = Rng(0x1ABE_15ED_0018_0004);
    let mut alphabet: Vec<u8> = (b' '..=b'~').collect();
    for i in (1..alphabet.len()).rev() {
        alphabet.swap(i, rng.below(i as u64 + 1) as usize);
    }
    // 94 distinct cut points split 0..n into 95 non-empty runs.
    let mut cuts: Vec<usize> = Vec::new();
    while cuts.len() < 94 {
        let c = 1 + rng.below(n as u64 - 1) as usize;
        if !cuts.contains(&c) {
            cuts.push(c);
        }
    }
    cuts.sort_unstable();
    cuts.push(n);
    let mut want = Vec::new();
    let mut bytes = Vec::with_capacity(n);
    let mut from = 0;
    for (&to, &b) in cuts.iter().zip(&alphabet) {
        want.push(to - from);
        bytes.extend_from_slice(&vec![b; to - from]);
        from = to;
    }
    let s = String::from_utf8(bytes).expect("ASCII");
    assert_eq!(lengths(&s), want, "n = {n}");
}
