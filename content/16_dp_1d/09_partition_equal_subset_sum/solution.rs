// Partition Equal Subset Sum — reference solution.
// 0/1 knapsack reachability for target = total / 2, iterating sums downwards
// so each element is used once. Time O(n * sum), space O(sum).

pub fn can_partition(nums: &[u32]) -> bool {
    let total: usize = nums.iter().map(|&x| x as usize).sum();
    if !total.is_multiple_of(2) {
        return false;
    }
    let target = total / 2;
    let mut reachable = vec![false; target + 1];
    reachable[0] = true;
    for &x in nums {
        let x = x as usize;
        for s in (x..=target).rev() {
            reachable[s] |= reachable[s - x];
        }
        if reachable[target] {
            return true;
        }
    }
    reachable[target]
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
