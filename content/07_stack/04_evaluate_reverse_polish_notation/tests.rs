// Tests for Evaluate Reverse Polish Notation — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(tokens: &[&str], expected: i32) {
    assert_eq!(eval_rpn(tokens), Ok(expected), "tokens = {tokens:?}");
}

fn check_err(tokens: &[&str], expected: RpnError) {
    assert_eq!(eval_rpn(tokens), Err(expected), "tokens = {tokens:?}");
}

fn as_strs(owned: &[String]) -> Vec<&str> {
    owned.iter().map(String::as_str).collect()
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

    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + self.below((hi - lo + 1) as u64) as i64
    }
}

enum Expr {
    Num(i64),
    Op(u8, Box<Expr>, Box<Expr>),
}

/// Oracle: evaluate an expression tree directly, in i64, checking every
/// intermediate value. Children are evaluated left then right, then the
/// operator — the same order in which postfix tokens are read, so the first
/// error found here is the first error the evaluator meets.
fn brute_eval(e: &Expr) -> Result<i64, RpnError> {
    let v = match e {
        Expr::Num(n) => *n,
        Expr::Op(op, l, r) => {
            let (a, b) = (brute_eval(l)?, brute_eval(r)?);
            match op {
                b'+' => a + b,
                b'-' => a - b,
                b'*' => a * b,
                _ => {
                    if b == 0 {
                        return Err(RpnError::DivisionByZero);
                    }
                    a / b
                }
            }
        }
    };
    if i32::try_from(v).is_err() {
        return Err(RpnError::Overflow);
    }
    Ok(v)
}

fn to_postfix(e: &Expr, out: &mut Vec<String>) {
    match e {
        Expr::Num(n) => out.push(n.to_string()),
        Expr::Op(op, l, r) => {
            to_postfix(l, out);
            to_postfix(r, out);
            out.push((*op as char).to_string());
        }
    }
}

fn random_expr(rng: &mut Rng, depth: u32, max_abs: i64) -> Expr {
    if depth == 0 || rng.below(3) == 0 {
        Expr::Num(rng.range(-max_abs, max_abs))
    } else {
        let op = b"+-*/"[rng.below(4) as usize];
        let l = random_expr(rng, depth - 1, max_abs);
        let r = random_expr(rng, depth - 1, max_abs);
        Expr::Op(op, Box::new(l), Box::new(r))
    }
}

/// Oracle for arbitrary (possibly malformed) sequences of small numbers and
/// `+`/`-`: tracks the stack in i64 and reports structural errors.
fn brute_structure(tokens: &[&str]) -> Result<i32, RpnError> {
    let mut stack: Vec<i64> = Vec::new();
    for t in tokens {
        match *t {
            "+" | "-" => {
                if stack.len() < 2 {
                    return Err(RpnError::MissingOperand);
                }
                let b = stack.pop().unwrap_or_default();
                let a = stack.pop().unwrap_or_default();
                stack.push(if *t == "+" { a + b } else { a - b });
            }
            n => stack.push(n.parse().expect("oracle only sees small numbers")),
        }
    }
    match stack.len() {
        0 => Err(RpnError::Empty),
        1 => Ok(stack[0] as i32),
        _ => Err(RpnError::TooManyOperands),
    }
}

#[test]
fn example_1() {
    check(&["2", "1", "+", "3", "*"], 9);
}

#[test]
fn example_2() {
    check(&["4", "13", "5", "/", "+"], 6);
}

#[test]
fn example_3() {
    check(&["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"], 22);
}

#[test]
fn example_errors() {
    check_err(&["1", "x", "+"], RpnError::InvalidToken("x".to_string()));
    check_err(&["1", "+"], RpnError::MissingOperand);
    check_err(&["1", "0", "/"], RpnError::DivisionByZero);
    check_err(&["2147483647", "1", "+"], RpnError::Overflow);
    check_err(&[], RpnError::Empty);
    check_err(&["1", "2"], RpnError::TooManyOperands);
}

#[test]
fn single_number() {
    check(&["42"], 42);
    check(&["-200"], -200);
    check(&["0"], 0);
}

#[test]
fn single_number_at_i32_limits() {
    check(&["2147483647"], i32::MAX);
    check(&["-2147483648"], i32::MIN);
}

#[test]
fn negative_number_token_is_not_an_operator() {
    check(&["-3", "4", "+"], 1);
    check(&["3", "-4", "-"], 7);
}

#[test]
fn subtraction_operand_order() {
    check(&["10", "3", "-"], 7);
    check(&["3", "10", "-"], -7);
}

#[test]
fn division_operand_order() {
    check(&["12", "4", "/"], 3);
    check(&["4", "12", "/"], 0);
}

#[test]
fn division_truncates_toward_zero_not_floor() {
    check(&["-7", "2", "/"], -3);
    check(&["7", "-2", "/"], -3);
    check(&["-7", "-2", "/"], 3);
    check(&["-1", "200", "/"], 0);
}

#[test]
fn multiplication_with_negatives() {
    check(&["-200", "-200", "*"], 40_000);
    check(&["-200", "200", "*"], -40_000);
}

#[test]
fn large_intermediate_near_i32_limits() {
    // 200^4 = 1.6e9, still fits in i32.
    check(&["200", "200", "*", "200", "*", "200", "*"], 1_600_000_000);
    check(&["-200", "200", "*", "200", "*", "200", "*"], -1_600_000_000);
}

#[test]
fn invalid_tokens() {
    for bad in ["x", "1.5", "", "++", "2 3", "**", "1e3", "abc"] {
        check_err(&["1", bad], RpnError::InvalidToken(bad.to_string()));
    }
}

#[test]
fn number_too_big_for_i32_is_invalid_token() {
    check_err(&["2147483648"], RpnError::InvalidToken("2147483648".to_string()));
    check_err(&["-2147483649", "1", "+"], RpnError::InvalidToken("-2147483649".to_string()));
}

#[test]
fn missing_operand_variants() {
    check_err(&["+"], RpnError::MissingOperand);
    check_err(&["-"], RpnError::MissingOperand);
    check_err(&["5", "*"], RpnError::MissingOperand);
    check_err(&["1", "2", "+", "/"], RpnError::MissingOperand);
}

#[test]
fn too_many_operands_variants() {
    check_err(&["1", "2", "3", "+"], RpnError::TooManyOperands);
    check_err(&["0", "0"], RpnError::TooManyOperands);
}

#[test]
fn division_by_zero_variants() {
    check_err(&["0", "0", "/"], RpnError::DivisionByZero);
    check_err(&["7", "3", "3", "-", "/"], RpnError::DivisionByZero);
    // Dividing zero is fine.
    check(&["0", "5", "/"], 0);
}

#[test]
fn overflow_in_every_operator() {
    check_err(&["2147483647", "1", "+"], RpnError::Overflow);
    check_err(&["-2147483648", "1", "-"], RpnError::Overflow);
    check_err(&["65536", "65536", "*"], RpnError::Overflow);
    check_err(&["0", "-2147483648", "-"], RpnError::Overflow);
}

#[test]
fn min_divided_by_minus_one_overflows_trap() {
    // The one i32 division that overflows: -2^31 / -1 = 2^31.
    check_err(&["-2147483648", "-1", "/"], RpnError::Overflow);
    check(&["-2147483648", "1", "/"], i32::MIN);
    check(&["-2147483647", "-1", "/"], i32::MAX);
}

#[test]
fn intermediate_overflow_even_if_final_result_fits() {
    check_err(&["2147483647", "1", "+", "1", "-"], RpnError::Overflow);
}

#[test]
fn first_error_wins() {
    check_err(&["1", "0", "/", "x"], RpnError::DivisionByZero);
    check_err(&["x", "+"], RpnError::InvalidToken("x".to_string()));
    check_err(&["+", "x"], RpnError::MissingOperand);
    check_err(&["2147483647", "1", "+", "+"], RpnError::Overflow);
    check_err(&["1", "2", "3", "+", "y"], RpnError::InvalidToken("y".to_string()));
}

#[test]
fn works_with_owned_and_split_tokens() {
    let owned: Vec<String> = ["3", "4", "+", "2", "*"].iter().map(|t| t.to_string()).collect();
    assert_eq!(eval_rpn(&as_strs(&owned)), Ok(14));
    let tokens: Vec<&str> = "5 1 2 + 4 * + 3 -".split_whitespace().collect();
    assert_eq!(eval_rpn(&tokens), Ok(14));
}

#[test]
fn error_type_is_comparable_and_cloneable() {
    let err = RpnError::InvalidToken("?".to_string());
    assert_eq!(err.clone(), err);
    assert_ne!(RpnError::Empty, RpnError::TooManyOperands);
    assert!(format!("{err:?}").contains("InvalidToken"));
    assert_eq!(eval_rpn(&["?"]), Err(err));
}

#[test]
fn randomized_vs_tree_eval() {
    // Mostly valid expressions with small numbers, as in the classic version.
    let mut rng = Rng(0x2301_abcd_ef01);
    let mut tested = 0;
    while tested < 400 {
        let e = random_expr(&mut rng, 5, 200);
        let Ok(expected) = brute_eval(&e) else { continue };
        let mut tokens = Vec::new();
        to_postfix(&e, &mut tokens);
        let tokens = as_strs(&tokens);
        assert_eq!(eval_rpn(&tokens), Ok(expected as i32), "tokens = {tokens:?}");
        tested += 1;
    }
}

#[test]
fn randomized_errors_vs_tree_eval() {
    // Small values hit division by zero; huge ones hit overflow.
    let mut rng = Rng(0x2302_0bad_f00d);
    let mut seen_err = 0;
    for _ in 0..1500 {
        let max_abs = [3, 200, 50_000, i32::MAX as i64][rng.below(4) as usize];
        let e = random_expr(&mut rng, 4, max_abs);
        let expected = brute_eval(&e).map(|v| v as i32);
        seen_err += usize::from(expected.is_err());
        let mut tokens = Vec::new();
        to_postfix(&e, &mut tokens);
        let tokens = as_strs(&tokens);
        assert_eq!(eval_rpn(&tokens), expected, "tokens = {tokens:?}");
    }
    assert!(seen_err > 100, "generator should produce plenty of errors");
}

#[test]
fn randomized_malformed_sequences() {
    let mut rng = Rng(0x2303_5eed_5eed);
    for _ in 0..800 {
        let len = rng.below(8) as usize;
        let owned: Vec<String> = (0..len)
            .map(|_| match rng.below(3) {
                0 => "+".to_string(),
                1 => "-".to_string(),
                _ => rng.range(-9, 9).to_string(),
            })
            .collect();
        let tokens = as_strs(&owned);
        assert_eq!(eval_rpn(&tokens), brute_structure(&tokens), "tokens = {tokens:?}");
    }
}

#[test]
fn large_left_deep() {
    // 1 1 + 1 + 1 + ... : stack never grows beyond 2.
    let mut t = vec!["1"];
    for _ in 0..4999 {
        t.push("1");
        t.push("+");
    }
    assert_eq!(eval_rpn(&t), Ok(5000));
}

#[test]
fn large_right_deep() {
    // 5000 numbers then 4999 operators: stack grows to 5000.
    let mut owned: Vec<String> = (0..5000).map(|i| ((i % 401) - 200).to_string()).collect();
    owned.extend((0..4999).map(|_| "+".to_string()));
    let expected: i32 = (0..5000).map(|i| (i % 401) - 200).sum();
    assert_eq!(eval_rpn(&as_strs(&owned)), Ok(expected));
}

#[test]
fn large_alternating_minus_right_deep() {
    // a0 - (a1 - (a2 - ...)) = a0 - a1 + a2 - ...
    let n = 5000;
    let mut owned: Vec<String> = (0..n).map(|i| (i % 7).to_string()).collect();
    owned.extend((0..n - 1).map(|_| "-".to_string()));
    let expected: i32 = (0..n).map(|i| if i % 2 == 0 { i % 7 } else { -(i % 7) }).sum();
    assert_eq!(eval_rpn(&as_strs(&owned)), Ok(expected));
}

#[test]
fn large_too_many_operands() {
    let owned: Vec<String> = (0..10_000).map(|i| (i % 100).to_string()).collect();
    assert_eq!(eval_rpn(&as_strs(&owned)), Err(RpnError::TooManyOperands));
}
