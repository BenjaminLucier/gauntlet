// Clone Graph — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub struct Node {
    pub val: i32,
    pub neighbors: Vec<NodeRef>,
}

/// A shared, mutable handle to a node; edges are `NodeRef`s too.
pub type NodeRef = Rc<RefCell<Node>>;

impl Node {
    /// A fresh node with no neighbours, already wrapped in `Rc<RefCell<…>>`.
    pub fn new(val: i32) -> NodeRef {
        Rc::new(RefCell::new(Node {
            val,
            neighbors: Vec::new(),
        }))
    }
}

pub fn clone_graph(node: &NodeRef) -> NodeRef {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
