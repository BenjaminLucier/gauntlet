// Box, Rc and Weak — reference solution.
// Box for owned indirection (trait objects, recursive types), Rc for shared
// ownership, Weak for back-pointers that must not keep their target alive.

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

pub fn make_shape(kind: &str, size: f64) -> Option<Box<dyn Shape>> {
    match kind {
        "circle" => Some(Box::new(Circle { r: size })),
        "square" => Some(Box::new(Square { side: size })),
        _ => None,
    }
}

pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

pub fn largest(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    shapes.iter().map(Box::as_ref).max_by(|a, b| a.area().total_cmp(&b.area()))
}

// ── A recursive enum ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Num(i64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

pub fn add(a: Expr, b: Expr) -> Expr {
    Expr::Add(Box::new(a), Box::new(b))
}

pub fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Neg(x) => -eval(x),
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

pub fn depth(e: &Expr) -> usize {
    match e {
        Expr::Num(_) => 1,
        Expr::Neg(x) => 1 + depth(x),
        Expr::Add(a, b) | Expr::Mul(a, b) => 1 + depth(a).max(depth(b)),
    }
}

pub fn cancel_double_neg(e: Expr) -> Expr {
    match e {
        Expr::Num(n) => Expr::Num(n),
        Expr::Neg(x) => match *x {
            Expr::Neg(inner) => cancel_double_neg(*inner),
            other => Expr::Neg(Box::new(cancel_double_neg(other))),
        },
        Expr::Add(a, b) => Expr::Add(Box::new(cancel_double_neg(*a)), Box::new(cancel_double_neg(*b))),
        Expr::Mul(a, b) => Expr::Mul(Box::new(cancel_double_neg(*a)), Box::new(cancel_double_neg(*b))),
    }
}

// ── Rc ──────────────────────────────────────────────────────────────────────

pub fn share<T>(rc: &Rc<T>, n: usize) -> Vec<Rc<T>> {
    (0..n).map(|_| Rc::clone(rc)).collect()
}

pub fn owners<T>(rc: &Rc<T>) -> usize {
    Rc::strong_count(rc)
}

pub fn same_allocation(a: &Rc<str>, b: &Rc<str>) -> bool {
    Rc::ptr_eq(a, b)
}

pub fn into_inner_if_unique(rc: Rc<Vec<i32>>) -> Result<Vec<i32>, Rc<Vec<i32>>> {
    Rc::try_unwrap(rc)
}

pub fn take_or_clone(rc: Rc<Vec<i32>>) -> Vec<i32> {
    Rc::unwrap_or_clone(rc)
}

pub fn push_cow(rc: &mut Rc<Vec<i32>>, x: i32) {
    Rc::make_mut(rc).push(x);
}

// ── Weak parent pointers ────────────────────────────────────────────────────

thread_local! {
    static DROPPED: Cell<usize> = const { Cell::new(0) };
}

/// How many `TreeNode`s have been dropped on this thread so far.
pub fn dropped_nodes() -> usize {
    DROPPED.with(Cell::get)
}

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
    pub fn new(name: &str) -> Rc<TreeNode> {
        Rc::new(TreeNode {
            name: name.to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }
}

pub fn add_child(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
    *child.parent.borrow_mut() = Rc::downgrade(parent);
    parent.children.borrow_mut().push(child);
}

pub fn parent_name(node: &TreeNode) -> Option<String> {
    node.parent.borrow().upgrade().map(|p| p.name.clone())
}

pub fn path(node: &Rc<TreeNode>) -> Vec<String> {
    let mut names = vec![node.name.clone()];
    let mut current = node.parent.borrow().upgrade();
    while let Some(n) = current {
        names.push(n.name.clone());
        current = n.parent.borrow().upgrade();
    }
    names.reverse();
    names
}

pub fn watchers(node: &Rc<TreeNode>) -> usize {
    Rc::weak_count(node)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
