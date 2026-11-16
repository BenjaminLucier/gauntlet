// Climbing Stairs — reference solution.
// ways(n) = ways(n-1) + ways(n-2) (Fibonacci), rolled forward in a pair of
// variables that never computes past ways(n). Time O(n), space O(1).

pub fn climb_stairs(n: u32) -> u64 {
    if n == 0 {
        return 1;
    }
    // The pair is (ways(i - 1), ways(i)), starting at i = 1; n - 1 steps reach i = n.
    let (_, ways) = (1..n).fold((1u64, 1u64), |(a, b), _| (b, a + b));
    ways
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
