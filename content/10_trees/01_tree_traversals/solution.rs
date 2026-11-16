// Tree Traversals — reference solution.
// Recursive walks append to one shared Vec; the iterative ones keep a Vec of
// borrowed nodes as the stack (postorder = reversed root-right-left preorder);
// level order drains a VecDeque one level per round. All O(n) time, O(h) stack.

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

#[derive(Clone, Copy)]
enum Order {
    Pre,
    In,
    Post,
}

fn walk(node: Option<&TreeNode>, order: Order, out: &mut Vec<i32>) {
    let Some(node) = node else { return };
    if let Order::Pre = order {
        out.push(node.val);
    }
    walk(node.left.as_deref(), order, out);
    if let Order::In = order {
        out.push(node.val);
    }
    walk(node.right.as_deref(), order, out);
    if let Order::Post = order {
        out.push(node.val);
    }
}

fn collect(root: Option<&TreeNode>, order: Order) -> Vec<i32> {
    let mut out = vec![];
    walk(root, order, &mut out);
    out
}

pub fn preorder(root: Option<&TreeNode>) -> Vec<i32> {
    collect(root, Order::Pre)
}

pub fn inorder(root: Option<&TreeNode>) -> Vec<i32> {
    collect(root, Order::In)
}

pub fn postorder(root: Option<&TreeNode>) -> Vec<i32> {
    collect(root, Order::Post)
}

pub fn preorder_iter(root: Option<&TreeNode>) -> Vec<i32> {
    let mut out = vec![];
    let mut stack: Vec<&TreeNode> = root.into_iter().collect();
    while let Some(node) = stack.pop() {
        out.push(node.val);
        // Right first, so the left subtree is popped (and emitted) first.
        stack.extend(node.right.as_deref());
        stack.extend(node.left.as_deref());
    }
    out
}

pub fn inorder_iter(root: Option<&TreeNode>) -> Vec<i32> {
    let mut out = vec![];
    let mut stack: Vec<&TreeNode> = vec![];
    let mut cur = root;
    loop {
        // Walk down the left spine, remembering every node on the way.
        while let Some(node) = cur {
            stack.push(node);
            cur = node.left.as_deref();
        }
        let Some(node) = stack.pop() else { break };
        out.push(node.val);
        cur = node.right.as_deref();
    }
    out
}

pub fn postorder_iter(root: Option<&TreeNode>) -> Vec<i32> {
    // Node, right, left — exactly postorder backwards.
    let mut out = vec![];
    let mut stack: Vec<&TreeNode> = root.into_iter().collect();
    while let Some(node) = stack.pop() {
        out.push(node.val);
        stack.extend(node.left.as_deref());
        stack.extend(node.right.as_deref());
    }
    out.reverse();
    out
}

pub fn level_order(root: Option<&TreeNode>) -> Vec<Vec<i32>> {
    let mut levels = vec![];
    let mut queue: VecDeque<&TreeNode> = root.into_iter().collect();
    while !queue.is_empty() {
        let row: Vec<i32> = (0..queue.len())
            .map(|_| {
                let node = queue.pop_front().expect("the level has this many nodes");
                queue.extend(node.left.as_deref());
                queue.extend(node.right.as_deref());
                node.val
            })
            .collect();
        levels.push(row);
    }
    levels
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
