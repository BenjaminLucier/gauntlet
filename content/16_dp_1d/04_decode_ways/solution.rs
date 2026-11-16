// Decode Ways — reference solution.
// ways(i) = [digit i-1 != '0'] * ways(i-1) + [digits i-2, i-1 form 10..=26] * ways(i-2),
// with ways(0) = 1, rolled forward in two variables over the ASCII bytes.
// Time O(n), space O(1).

pub fn num_decodings(digits: &str) -> u64 {
    let mut prev_digit: Option<u8> = None;
    // (ways(i - 2), ways(i - 1)) before processing digit i - 1.
    let (mut two_back, mut one_back) = (0u64, 1u64);
    for &d in digits.as_bytes() {
        let as_single = if d == b'0' { 0 } else { one_back };
        let as_pair = match (prev_digit, d) {
            (Some(b'1'), b'0'..=b'9') | (Some(b'2'), b'0'..=b'6') => two_back,
            _ => 0,
        };
        (two_back, one_back) = (one_back, as_single + as_pair);
        prev_digit = Some(d);
    }
    one_back
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
