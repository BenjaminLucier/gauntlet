// Pow(x, n) — reference solution.
// Iterative exponentiation by squaring on |n| (taken with unsigned_abs, so
// i32::MIN is safe), using 1/x as the base when n < 0. Time O(log |n|), space O(1).

pub fn pow(x: f64, n: i32) -> f64 {
    let mut e = n.unsigned_abs();
    let mut base = if n < 0 { x.recip() } else { x };
    let mut result = 1.0;
    while e > 0 {
        if e & 1 == 1 {
            result *= base;
        }
        base *= base;
        e >>= 1;
    }
    result
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
