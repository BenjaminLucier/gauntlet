// Reverse Integer — reference solution.
// Pop digits with % and / (negative digits for negative x) and push them with
// checked_mul / checked_add, so `?` returns None the moment the result would
// leave i32. Time O(log10 |x|), space O(1); no 64-bit arithmetic.

pub fn reverse(mut x: i32) -> Option<i32> {
    let mut r: i32 = 0;
    while x != 0 {
        r = r.checked_mul(10)?.checked_add(x % 10)?;
        x /= 10;
    }
    Some(r)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
