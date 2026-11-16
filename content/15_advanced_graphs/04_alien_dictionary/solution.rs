// Alien Dictionary — reference solution.
// Derive one edge from the first differing letter of each adjacent pair
// (rejecting a longer word before its own prefix), then run Kahn's topological
// sort over the letters present. Time O(C) for C total characters, space O(26²).

use std::collections::VecDeque;

const LETTERS: usize = 26;

fn index(letter: u8) -> usize {
    usize::from(letter - b'a')
}

pub fn alien_order(words: &[&str]) -> Option<String> {
    let mut present = [false; LETTERS];
    for &letter in words.iter().flat_map(|w| w.as_bytes()) {
        present[index(letter)] = true;
    }

    let mut edge = [[false; LETTERS]; LETTERS];
    let mut in_degree = [0usize; LETTERS];
    for pair in words.windows(2) {
        let (a, b) = (pair[0].as_bytes(), pair[1].as_bytes());
        match a.iter().zip(b).find(|(x, y)| x != y) {
            Some((&x, &y)) => {
                let (x, y) = (index(x), index(y));
                if !edge[x][y] {
                    edge[x][y] = true;
                    in_degree[y] += 1;
                }
            }
            // `a` is a longer word sorted before its own prefix.
            None if a.len() > b.len() => return None,
            None => {}
        }
    }

    let mut ready: VecDeque<usize> =
        (0..LETTERS).filter(|&c| present[c] && in_degree[c] == 0).collect();
    let mut order = String::with_capacity(LETTERS);
    while let Some(c) = ready.pop_front() {
        order.push(char::from(b'a' + c as u8));
        for d in (0..LETTERS).filter(|&d| edge[c][d]) {
            in_degree[d] -= 1;
            if in_degree[d] == 0 {
                ready.push_back(d);
            }
        }
    }

    // Letters left out are on a cycle.
    let letter_count = present.iter().filter(|&&p| p).count();
    (order.len() == letter_count).then_some(order)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
