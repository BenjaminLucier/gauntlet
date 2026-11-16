// Task Scheduler — reference solution.
// Count each label; the most frequent labels force (max - 1) frames of length
// cooldown + 1 plus a final frame of the tied labels, and the answer is never
// below tasks.len(). Time O(m), space O(1).

pub fn least_interval(tasks: &[u8], cooldown: usize) -> usize {
    let mut counts = [0usize; 256];
    for &t in tasks {
        counts[usize::from(t)] += 1;
    }
    let max = counts.iter().copied().max().unwrap_or(0);
    if max == 0 {
        return 0;
    }
    let ties = counts.iter().filter(|&&c| c == max).count();
    ((max - 1) * (cooldown + 1) + ties).max(tasks.len())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
