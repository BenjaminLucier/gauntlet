// Tests for Task Scheduler — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Time-step simulation: each unit, run the available task with the most
/// remaining copies (an optimal greedy), otherwise idle.
fn brute_least_interval(tasks: &[u8], cooldown: usize) -> usize {
    let mut counts = [0usize; 256];
    for &t in tasks {
        counts[usize::from(t)] += 1;
    }
    let mut ready_at = [0usize; 256];
    let mut left = tasks.len();
    let mut t = 0;
    while left > 0 {
        let mut best: Option<usize> = None;
        for i in 0..256 {
            if counts[i] > 0 && ready_at[i] <= t && best.is_none_or(|b| counts[i] > counts[b]) {
                best = Some(i);
            }
        }
        if let Some(i) = best {
            counts[i] -= 1;
            ready_at[i] = t + cooldown + 1;
            left -= 1;
        }
        t += 1;
    }
    t
}

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    fn letter(&mut self, letters: usize) -> u8 {
        b'A' + self.below(letters) as u8
    }
}

#[test]
fn example_1() {
    assert_eq!(least_interval(b"AAABBB", 2), 8);
}

#[test]
fn example_2() {
    assert_eq!(least_interval(b"ACABDB", 1), 6);
}

#[test]
fn example_3() {
    assert_eq!(least_interval(b"AAABBB", 3), 10);
}

#[test]
fn empty_takes_no_time() {
    assert_eq!(least_interval(b"", 5), 0);
    assert_eq!(least_interval(&[], 0), 0);
}

#[test]
fn single_task() {
    assert_eq!(least_interval(b"Z", 0), 1);
    assert_eq!(least_interval(b"Z", 100), 1);
}

#[test]
fn cooldown_zero_means_no_idle() {
    assert_eq!(least_interval(b"AAAABBBCCD", 0), 10);
}

#[test]
fn single_letter_repeated() {
    // A _ _ A _ _ A
    assert_eq!(least_interval(b"AAA", 2), 7);
    assert_eq!(least_interval(b"AA", 100), 102);
}

#[test]
fn several_letters_tied_for_max() {
    // ABC ABC ABC
    assert_eq!(least_interval(b"AAABBBCCC", 2), 9);
    // ABC_ ABC_ ABC
    assert_eq!(least_interval(b"AAABBBCCC", 3), 11);
}

#[test]
fn formula_below_task_count_trap() {
    // (2-1)*(2+1)+2 = 5, but there are 8 tasks and no idling is needed.
    assert_eq!(least_interval(b"AABBCDEF", 2), 8);
    // Many distinct letters with cooldown 1.
    assert_eq!(least_interval(b"ABCDEFGHIJKLMNOPQRSTUVWXYZA", 1), 27);
}

#[test]
fn ties_more_than_cooldown_plus_one() {
    // 4 letters tied at 2 with cooldown 1: no idling, 8 units.
    assert_eq!(least_interval(b"AABBCCDD", 1), 8);
}

#[test]
fn fillers_reduce_idles() {
    // A B C A D E A -> 7
    assert_eq!(least_interval(b"AAABCDE", 2), 7);
    // A B C A D _ A -> 7
    assert_eq!(least_interval(b"AAABCD", 2), 7);
}

#[test]
fn labels_outside_a_to_z() {
    // Lowercase, digits and non-letter bytes are labels too: indexing with
    // `t - b'A'` underflows on b'0' and overflows a 26-slot table on b'z'.
    assert_eq!(least_interval(b"aaA", 1), 3);
    assert_eq!(least_interval(b"00z", 2), 4);
    assert_eq!(least_interval(&[0, 255, 0, 255], 1), 4);
    assert_eq!(least_interval(&[0, 0, 0], 1), 5);
}

#[test]
fn uppercase_and_lowercase_are_different_labels() {
    assert_eq!(least_interval(b"aA", 50), 2);
    assert_eq!(least_interval(b"aa", 50), 52);
}

#[test]
fn randomized_against_simulation() {
    let mut rng = Rng(0x0688_0688);
    for _ in 0..600 {
        let len = rng.below(21);
        let letters = rng.below(6) + 1;
        let tasks: Vec<u8> = (0..len).map(|_| rng.letter(letters)).collect();
        let cooldown = rng.below(7);
        assert_eq!(
            least_interval(&tasks, cooldown),
            brute_least_interval(&tasks, cooldown),
            "tasks={:?}, cooldown={cooldown}",
            String::from_utf8_lossy(&tasks)
        );
    }
}

#[test]
fn large_single_letter() {
    assert_eq!(least_interval(&[b'A'; 10_000], 100), 9_999 * 101 + 1);
}

#[test]
fn large_mixed() {
    let mut rng = Rng(0xfeed_0068);
    let tasks: Vec<u8> = (0..10_000).map(|_| rng.letter(26)).collect();
    for cooldown in [0, 1, 25, 30, 100] {
        assert_eq!(
            least_interval(&tasks, cooldown),
            brute_least_interval(&tasks, cooldown),
            "cooldown={cooldown}"
        );
    }
}
