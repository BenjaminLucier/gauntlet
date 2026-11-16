// Reorder List — reference solution.
// Split after the first ceil(n/2) nodes, reverse the second half, then splice
// its nodes in after every node of the first half. Time O(n), space O(1) extra.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn reorder_list(mut head: List) -> List {
    let len = std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count();

    // Split: the first half keeps (len + 1) / 2 nodes.
    let mut cur = &mut head;
    for _ in 0..len.div_ceil(2) {
        cur = &mut cur.as_mut().expect("within the list's length").next;
    }
    let second = cur.take();

    // Reverse the second half.
    let mut rev = reverse(second);

    // Interleave: after each first-half node, splice in one node from `rev`.
    let mut cur = head.as_deref_mut();
    while let (Some(node), Some(mut r)) = (cur, rev) {
        rev = std::mem::replace(&mut r.next, node.next.take());
        let inserted = node.next.insert(r);
        cur = inserted.next.as_deref_mut();
    }
    head
}

fn reverse(mut list: List) -> List {
    let mut reversed = None;
    while let Some(mut node) = list {
        list = std::mem::replace(&mut node.next, reversed);
        reversed = Some(node);
    }
    reversed
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
