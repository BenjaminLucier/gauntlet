// Remove Nth Node From End of List — reference solution.
// Count the length, walk a `&mut List` cursor to position len - n, and splice
// that node out; the head needs no special case. Time O(len), space O(1).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn remove_nth_from_end(mut head: List, n: usize) -> List {
    let len = std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
    if n == 0 || n > len {
        return head;
    }
    let index = len - n;
    let mut slot = &mut head;
    for _ in 0..index {
        slot = &mut slot.as_mut().expect("index < len").next;
    }
    let removed = slot.take().expect("index < len");
    *slot = removed.next;
    head
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
