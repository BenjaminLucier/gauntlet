// Box, Rc and Weak — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

// ── Trait objects ───────────────────────────────────────────────────────────

pub trait Shape {
    fn name(&self) -> &'static str;
    fn area(&self) -> f64;
}

pub struct Circle {
    pub r: f64,
}

pub struct Square {
    pub side: f64,
}

impl Shape for Circle {
    fn name(&self) -> &'static str {
        "circle"
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

impl Shape for Square {
    fn name(&self) -> &'static str {
        "square"
    }
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

/// `"circle"` (radius `size`) or `"square"` (side `size`), boxed behind the
/// trait; `None` for any other kind. (`Box<dyn Trait>`)
pub fn make_shape(kind: &str, size: f64) -> Option<Box<dyn Shape>> {
    todo!()
}

/// The sum of the areas. (dynamic dispatch through `Box<dyn Shape>`)
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    todo!()
}

/// Borrow the shape with the largest area, or `None` if there are none.
/// (`Box::as_ref`, `f64::total_cmp`)
pub fn largest(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    todo!()
}

// ── A recursive enum ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Num(i64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

/// Build `a + b`. (`Box::new`)
pub fn add(a: Expr, b: Expr) -> Expr {
    todo!()
}

/// Evaluate the expression. (match through `&Box<Expr>` — it derefs)
pub fn eval(e: &Expr) -> i64 {
    todo!()
}

/// The number of nodes on the longest root-to-leaf path; a `Num` has depth 1.
/// (recursion through `&Box<Expr>`, which derefs to `&Expr`; `Ord::max`)
pub fn depth(e: &Expr) -> usize {
    todo!()
}

/// Remove every `Neg(Neg(x))` pair, anywhere in the tree, without cloning:
/// move the children out of their boxes. (`*boxed` moves out of a `Box`)
pub fn cancel_double_neg(e: Expr) -> Expr {
    todo!()
}

// ── Rc ──────────────────────────────────────────────────────────────────────

/// `n` more owners of the same value — no deep copies. (`Rc::clone`)
pub fn share<T>(rc: &Rc<T>, n: usize) -> Vec<Rc<T>> {
    todo!()
}

/// How many `Rc`s currently own this value. (`Rc::strong_count`)
pub fn owners<T>(rc: &Rc<T>) -> usize {
    todo!()
}

/// Do `a` and `b` point at the same allocation (not merely equal text)?
/// (`Rc::ptr_eq`)
pub fn same_allocation(a: &Rc<str>, b: &Rc<str>) -> bool {
    todo!()
}

/// The inner value if `rc` is its only owner, otherwise `rc` back.
/// (`Rc::try_unwrap`)
pub fn into_inner_if_unique(rc: Rc<Vec<i32>>) -> Result<Vec<i32>, Rc<Vec<i32>>> {
    todo!()
}

/// The inner value: moved out if `rc` is the only owner, cloned otherwise.
/// (`Rc::unwrap_or_clone`)
pub fn take_or_clone(rc: Rc<Vec<i32>>) -> Vec<i32> {
    todo!()
}

/// Push `x` onto the vector without affecting other owners: clone it first
/// only if it's shared. (`Rc::make_mut`, clone-on-write)
pub fn push_cow(rc: &mut Rc<Vec<i32>>, x: i32) {
    todo!()
}

// ── Weak parent pointers ────────────────────────────────────────────────────

thread_local! {
    static DROPPED: Cell<usize> = const { Cell::new(0) };
}

/// How many `TreeNode`s have been dropped on this thread so far.
pub fn dropped_nodes() -> usize {
    DROPPED.with(Cell::get)
}

/// Children are owned (`Rc`); the parent is only observed (`Weak`), so a
/// parent and child never keep each other alive.
#[derive(Debug)]
pub struct TreeNode {
    pub name: String,
    pub parent: RefCell<Weak<TreeNode>>,
    pub children: RefCell<Vec<Rc<TreeNode>>>,
}

impl Drop for TreeNode {
    fn drop(&mut self) {
        DROPPED.with(|d| d.set(d.get() + 1));
    }
}

impl TreeNode {
    /// A node with no parent and no children. (`Weak::new`)
    pub fn new(name: &str) -> Rc<TreeNode> {
        todo!()
    }
}

/// Make `child` a child of `parent`: point its parent link at `parent` and
/// push it onto `parent`'s children. (`Rc::downgrade`)
pub fn add_child(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
    todo!()
}

/// The parent's name, or `None` for a root or a parent that's been dropped.
/// (`Weak::upgrade`)
pub fn parent_name(node: &TreeNode) -> Option<String> {
    todo!()
}

/// Names from the root down to `node`, inclusive. (`Weak::upgrade` in a loop)
pub fn path(node: &Rc<TreeNode>) -> Vec<String> {
    todo!()
}

/// How many `Weak` pointers observe this node. (`Rc::weak_count`)
pub fn watchers(node: &Rc<TreeNode>) -> usize {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
