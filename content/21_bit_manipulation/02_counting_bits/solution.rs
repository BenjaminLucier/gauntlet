// Counting Bits — reference solution.
// DP: i has the bits of i >> 1 plus its own lowest bit, so
// ans[i] = ans[i >> 1] + (i & 1). Time O(n), space O(1) besides the output.

pub fn count_bits(n: usize) -> Vec<u32> {
    let mut ans = vec![0; n + 1];
    for i in 1..=n {
        ans[i] = ans[i >> 1] + (i & 1) as u32;
    }
    ans
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
