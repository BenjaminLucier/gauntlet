// Linked List Cycle — reference solution.
// Floyd's tortoise and hare over arena indices: slow moves one step, fast two;
// they meet iff there is a cycle. Time O(n), space O(1).

/// A list node stored in an arena (a slice); `next` is the index of the next node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub next: Option<usize>,
}

pub fn has_cycle(nodes: &[Node], head: Option<usize>) -> bool {
    let step = |i: Option<usize>| i.and_then(|i| nodes[i].next);
    let (mut slow, mut fast) = (head, head);
    loop {
        fast = step(step(fast));
        slow = step(slow);
        match fast {
            None => return false,
            Some(_) if fast == slow => return true,
            Some(_) => {}
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
