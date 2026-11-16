// Tests for Multiply Strings — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.next() % (hi - lo + 1)
    }
}

/// Big-int oracle: schoolbook multiplication on little-endian digit vectors —
/// for each digit of `b`, add `a * digit` shifted into the accumulator.
fn brute_multiply(a: &str, b: &str) -> String {
    let a: Vec<u32> = a.bytes().rev().map(|c| u32::from(c - b'0')).collect();
    let b: Vec<u32> = b.bytes().rev().map(|c| u32::from(c - b'0')).collect();
    let mut acc: Vec<u32> = vec![0];
    for (shift, &d) in b.iter().enumerate() {
        // partial = a * d, shifted
        let mut partial = vec![0u32; shift];
        let mut carry = 0;
        for &x in &a {
            let v = x * d + carry;
            partial.push(v % 10);
            carry = v / 10;
        }
        if carry > 0 {
            partial.push(carry);
        }
        // acc += partial
        let len = acc.len().max(partial.len());
        let mut sum = Vec::with_capacity(len + 1);
        let mut carry = 0;
        for i in 0..len {
            let v = acc.get(i).copied().unwrap_or(0) + partial.get(i).copied().unwrap_or(0) + carry;
            sum.push(v % 10);
            carry = v / 10;
        }
        if carry > 0 {
            sum.push(carry);
        }
        acc = sum;
    }
    while acc.len() > 1 && acc.last() == Some(&0) {
        acc.pop();
    }
    acc.iter().rev().map(|&d| char::from(b'0' + d as u8)).collect()
}

fn random_num(rng: &mut Rng, len: usize) -> String {
    (0..len)
        .map(|i| {
            let d = if i == 0 && len > 1 { rng.range(1, 9) } else { rng.range(0, 9) };
            char::from(b'0' + d as u8)
        })
        .collect()
}

fn check(a: &str, b: &str, expected: &str) {
    assert_eq!(multiply(a, b).as_deref(), Ok(expected), "a = {a}, b = {b}");
}

#[test]
fn example_1() {
    assert_eq!(multiply("2", "3"), Ok("6".to_string()));
}

#[test]
fn example_2() {
    check("123", "456", "56088");
}

#[test]
fn example_errors() {
    assert_eq!(multiply("12", ""), Err(NumberError::Empty));
    assert_eq!(multiply("-5", "3"), Err(NumberError::InvalidDigit('-')));
}

#[test]
fn zero_times_zero() {
    check("0", "0", "0");
}

#[test]
fn zero_no_leading_zeros_trap() {
    check("0", "12345", "0");
    check("98765", "0", "0");
}

#[test]
fn leading_zeros_in_input_are_ignored() {
    check("007", "6", "42");
    check("000", "000", "0");
    check("0001", "0001", "1");
    check("010", "10", "100");
}

#[test]
fn times_one() {
    check("1", "1", "1");
    check("1", "987654321", "987654321");
    check("987654321", "1", "987654321");
}

#[test]
fn single_digits_carry() {
    check("9", "9", "81");
    check("5", "2", "10");
}

#[test]
fn nines() {
    check("999", "999", "998001");
    check("99", "9", "891");
}

#[test]
fn powers_of_ten() {
    check("10", "10", "100");
    check("1000", "100000", "100000000");
}

#[test]
fn product_shorter_than_m_plus_n() {
    // 2 digits * 2 digits = 3 digits; the top slot stays 0 and must be stripped.
    check("12", "34", "408");
}

#[test]
fn beyond_u64() {
    check(
        "18446744073709551615",
        "18446744073709551615",
        "340282366920938463426481119284349108225",
    );
}

#[test]
fn empty_input_is_an_error() {
    assert_eq!(multiply("", "5"), Err(NumberError::Empty));
    assert_eq!(multiply("5", ""), Err(NumberError::Empty));
    assert_eq!(multiply("", ""), Err(NumberError::Empty));
}

#[test]
fn invalid_digits_are_reported() {
    assert_eq!(multiply("12a", "3"), Err(NumberError::InvalidDigit('a')));
    assert_eq!(multiply("+5", "3"), Err(NumberError::InvalidDigit('+')));
    assert_eq!(multiply("1 2", "3"), Err(NumberError::InvalidDigit(' ')));
    assert_eq!(multiply("3", "4.5"), Err(NumberError::InvalidDigit('.')));
    assert_eq!(multiply("3", "0x10"), Err(NumberError::InvalidDigit('x')));
}

#[test]
fn first_bad_char_and_a_before_b() {
    assert_eq!(multiply("1x2y", "3"), Err(NumberError::InvalidDigit('x')));
    assert_eq!(multiply("1x", "y"), Err(NumberError::InvalidDigit('x')));
    assert_eq!(multiply("", "y"), Err(NumberError::Empty));
    assert_eq!(multiply("y", ""), Err(NumberError::InvalidDigit('y')));
}

#[test]
fn non_ascii_digits_are_rejected_whole() {
    // Arabic-Indic three and a fullwidth five are digits to Unicode, not to us;
    // the error must carry the full char, not one of its UTF-8 bytes.
    assert_eq!(multiply("٣", "2"), Err(NumberError::InvalidDigit('٣')));
    assert_eq!(multiply("2", "1５"), Err(NumberError::InvalidDigit('５')));
    assert_eq!(multiply("12é", "2"), Err(NumberError::InvalidDigit('é')));
}

#[test]
fn all_small_pairs_vs_u128() {
    for a in 0u128..=120 {
        for b in [0u128, 1, 7, 10, 99, 100, 12345] {
            check(&a.to_string(), &b.to_string(), &(a * b).to_string());
        }
    }
}

#[test]
fn randomized_vs_u128() {
    let mut rng = Rng(0x0f0f_1e1e_2d2d_3c3c);
    for _ in 0..500 {
        let la = rng.range(1, 19) as usize;
        let lb = rng.range(1, 19) as usize;
        let a = random_num(&mut rng, la);
        let b = random_num(&mut rng, lb);
        let x: u128 = a.parse().expect("test input is a valid number");
        let y: u128 = b.parse().expect("test input is a valid number");
        check(&a, &b, &(x * y).to_string());
    }
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(0x7777_aaaa_5555_3333);
    for _ in 0..200 {
        let la = rng.range(1, 60) as usize;
        let lb = rng.range(1, 60) as usize;
        let a = if rng.range(0, 20) == 0 {
            "0".to_string()
        } else {
            random_num(&mut rng, la)
        };
        let b = random_num(&mut rng, lb);
        check(&a, &b, &brute_multiply(&a, &b));
    }
}

#[test]
fn large_200_nines_squared() {
    // (10^200 - 1)^2 = 99..9 8 00..0 1  (199 nines, 8, 199 zeros, 1)
    let a = "9".repeat(200);
    let expected = format!("{}8{}1", "9".repeat(199), "0".repeat(199));
    check(&a, &a, &expected);
}

#[test]
fn large_random_200_digits_vs_brute_force() {
    let mut rng = Rng(0x2468_1357_0000_ffff);
    for _ in 0..10 {
        let a = random_num(&mut rng, 200);
        let b = random_num(&mut rng, 200);
        check(&a, &b, &brute_multiply(&a, &b));
    }
}
