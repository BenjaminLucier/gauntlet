// Daily Temperatures — reference solution.
// Monotonic stack of unresolved day indices (non-increasing temperatures); a
// warmer day pops and resolves them. Each index is pushed and popped once: O(n) time, O(n) space.

pub fn daily_temperatures(temperatures: &[i32]) -> Vec<Option<usize>> {
    let mut answer = vec![None; temperatures.len()];
    let mut waiting: Vec<usize> = Vec::new();
    for (today, &temp) in temperatures.iter().enumerate() {
        while let Some(&day) = waiting.last() {
            if temperatures[day] >= temp {
                break;
            }
            waiting.pop();
            answer[day] = Some(today - day);
        }
        waiting.push(today);
    }
    answer
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
