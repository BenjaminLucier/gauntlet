// Reverse Nodes in k-Group — reference solution.
// Count the length; for each of the len / k full groups, pop k nodes onto a
// reversed chain, write it into a `&mut List` cursor and advance the cursor k
// nodes; finally attach the leftover nodes. Time O(n), space O(1) extra.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn reverse_k_group(head: List, k: usize) -> List {
    if k <= 1 {
        return head;
    }
    let len = std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count();

    let mut rest = head;
    let mut result = None;
    let mut tail = &mut result;
    for _ in 0..len / k {
        let mut group = None;
        for _ in 0..k {
            let mut node = rest.expect("a full group has k nodes");
            rest = std::mem::replace(&mut node.next, group);
            group = Some(node);
        }
        *tail = group;
        for _ in 0..k {
            tail = &mut tail.as_mut().expect("the group just written has k nodes").next;
        }
    }
    *tail = rest;
    result
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
