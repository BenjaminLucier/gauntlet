// Clone Graph — reference solution.
// BFS with a map from original node (by pointer identity) to its copy: a node
// is copied when first discovered and its neighbour list filled when processed.
// Time O(V + E), space O(V).

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub struct Node {
    pub val: i32,
    pub neighbors: Vec<NodeRef>,
}

pub type NodeRef = Rc<RefCell<Node>>;

impl Node {
    pub fn new(val: i32) -> NodeRef {
        Rc::new(RefCell::new(Node {
            val,
            neighbors: Vec::new(),
        }))
    }
}

pub fn clone_graph(node: &NodeRef) -> NodeRef {
    let start_copy = Node::new(node.borrow().val);
    let mut copies: HashMap<*const RefCell<Node>, NodeRef> = HashMap::new();
    copies.insert(Rc::as_ptr(node), Rc::clone(&start_copy));
    let mut queue = VecDeque::from([Rc::clone(node)]);
    while let Some(original) = queue.pop_front() {
        let copy = Rc::clone(&copies[&Rc::as_ptr(&original)]);
        // Only `original` is borrowed here, and only `copy` mutably: they're
        // different RefCells, so the borrows can't conflict.
        for neighbour in &original.borrow().neighbors {
            let neighbour_copy = copies
                .entry(Rc::as_ptr(neighbour))
                .or_insert_with(|| {
                    queue.push_back(Rc::clone(neighbour));
                    Node::new(neighbour.borrow().val)
                });
            copy.borrow_mut().neighbors.push(Rc::clone(neighbour_copy));
        }
    }
    start_copy
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
