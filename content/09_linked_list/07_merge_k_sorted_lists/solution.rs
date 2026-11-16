// Merge k Sorted Lists — reference solution.
// Min-heap of (head value, list index): pop the smallest, move that node onto
// the result through a `&mut List` cursor, push the list's new head.
// Time O(N log k), space O(k).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn merge_k_lists(mut lists: Vec<List>) -> List {
    let mut heap: BinaryHeap<_> = lists
        .iter()
        .enumerate()
        .filter_map(|(i, list)| list.as_ref().map(|node| Reverse((node.val, i))))
        .collect();
    let mut head = None;
    let mut tail = &mut head;
    while let Some(Reverse((_, i))) = heap.pop() {
        let mut node = lists[i].take().expect("the heap only holds non-empty lists");
        lists[i] = node.next.take();
        if let Some(next) = &lists[i] {
            heap.push(Reverse((next.val, i)));
        }
        tail = &mut tail.insert(node).next;
    }
    head
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
