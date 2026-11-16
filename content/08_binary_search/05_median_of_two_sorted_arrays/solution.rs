// Median of Two Sorted Arrays — reference solution.
// Binary search the cut i in the shorter slice (j = half - i in the other)
// until both left parts are <= both right parts; values are widened to i64.
// Time O(log(min(m, n))), space O(1).

/// The element just left of a cut after `k` elements, or -∞ if there is none.
fn left_of(s: &[i32], k: usize) -> i64 {
    k.checked_sub(1).map_or(i64::MIN, |k| i64::from(s[k]))
}

/// The element just right of a cut after `k` elements, or +∞ if there is none.
fn right_of(s: &[i32], k: usize) -> i64 {
    s.get(k).map_or(i64::MAX, |&x| i64::from(x))
}

pub fn median_of_sorted(a: &[i32], b: &[i32]) -> Option<f64> {
    let (a, b) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    let total = a.len() + b.len();
    if total == 0 {
        return None;
    }
    let half = total.div_ceil(2);
    let (mut lo, mut hi) = (0, a.len());
    loop {
        let i = lo + (hi - lo) / 2;
        let j = half - i;
        let (a_left, a_right) = (left_of(a, i), right_of(a, i));
        let (b_left, b_right) = (left_of(b, j), right_of(b, j));
        if a_left > b_right {
            hi = i - 1; // a_left is a real element, so i >= 1
        } else if b_left > a_right {
            lo = i + 1;
        } else {
            let left_max = a_left.max(b_left);
            if total % 2 == 1 {
                return Some(left_max as f64);
            }
            let right_min = a_right.min(b_right);
            return Some((left_max + right_min) as f64 / 2.0);
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
