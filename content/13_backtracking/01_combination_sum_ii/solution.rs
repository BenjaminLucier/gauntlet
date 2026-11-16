// Combination Sum II — reference solution.
// Sort a copy; backtrack over the remaining suffix, skipping a value equal to
// the previous one at the same depth so each combination is generated once.
// Time O(2^n) worst case, space O(n) besides the output.

pub fn combination_sum_ii(candidates: &[u32], target: u32) -> Vec<Vec<u32>> {
    fn go(sorted: &[u32], remaining: u32, path: &mut Vec<u32>, out: &mut Vec<Vec<u32>>) {
        if remaining == 0 {
            out.push(path.clone());
            return;
        }
        for (i, &c) in sorted.iter().enumerate() {
            if c > remaining {
                break; // sorted: every later candidate is too large as well
            }
            if i > 0 && c == sorted[i - 1] {
                continue; // same value at the same depth: same subtree again
            }
            path.push(c);
            go(&sorted[i + 1..], remaining - c, path, out);
            path.pop();
        }
    }

    let mut sorted = candidates.to_vec();
    sorted.sort_unstable();
    let mut out = Vec::new();
    go(&sorted, target, &mut Vec::new(), &mut out);
    out
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
