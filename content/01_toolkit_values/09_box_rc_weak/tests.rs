// Tests for Box, Rc and Weak — shared by template.rs, solution.rs and your attempt.
use super::*;

fn num(n: i64) -> Expr {
    Expr::Num(n)
}

fn neg(e: Expr) -> Expr {
    Expr::Neg(Box::new(e))
}

fn mul(a: Expr, b: Expr) -> Expr {
    Expr::Mul(Box::new(a), Box::new(b))
}

// ── Trait objects ──

#[test]
fn make_shape_boxes_by_kind() {
    let c = make_shape("circle", 1.0).expect("circle is known");
    assert_eq!(c.name(), "circle");
    assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
    let s = make_shape("square", 3.0).expect("square is known");
    assert_eq!(s.name(), "square");
    assert!((s.area() - 9.0).abs() < 1e-9);
    assert!(make_shape("hexagon", 1.0).is_none());
}

#[test]
fn total_area_sums_a_mixed_collection() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square { side: 2.0 }), Box::new(Circle { r: 1.0 }), Box::new(Square { side: 1.0 })];
    assert!((total_area(&shapes) - (5.0 + std::f64::consts::PI)).abs() < 1e-9);
    assert_eq!(total_area(&[]), 0.0);
}

#[test]
fn largest_borrows_the_biggest() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square { side: 1.5 }), Box::new(Circle { r: 1.0 }), Box::new(Square { side: 1.7 })];
    let big = largest(&shapes).expect("non-empty");
    assert_eq!(big.name(), "circle", "pi > 1.7^2 = 2.89");
    assert!(largest(&[]).is_none());
}

// ── A recursive enum ──

#[test]
fn add_boxes_both_sides() {
    assert_eq!(add(num(1), num(2)), Expr::Add(Box::new(num(1)), Box::new(num(2))));
}

#[test]
fn eval_walks_the_tree() {
    // (2 + 3) * -(4)
    let e = mul(add(num(2), num(3)), neg(num(4)));
    assert_eq!(eval(&e), -20);
    assert_eq!(eval(&num(7)), 7);
    assert_eq!(eval(&neg(neg(num(7)))), 7);
}

#[test]
fn depth_counts_the_longest_path() {
    assert_eq!(depth(&num(1)), 1);
    assert_eq!(depth(&add(num(1), num(2))), 2);
    assert_eq!(depth(&mul(add(num(1), neg(num(2))), num(3))), 4);
}

#[test]
fn cancel_double_neg_moves_out_of_boxes() {
    assert_eq!(cancel_double_neg(neg(neg(num(5)))), num(5));
    assert_eq!(cancel_double_neg(neg(neg(neg(num(5))))), neg(num(5)), "an odd count leaves one");
    assert_eq!(cancel_double_neg(neg(neg(neg(neg(num(5)))))), num(5));
    let deep = add(num(1), mul(neg(neg(num(2))), neg(num(3))));
    assert_eq!(cancel_double_neg(deep), add(num(1), mul(num(2), neg(num(3)))));
}

#[test]
fn cancel_double_neg_sees_through_an_inner_pair() {
    // --(--1 + 2): pairs at two levels
    let e = neg(neg(add(neg(neg(num(1))), num(2))));
    assert_eq!(cancel_double_neg(e), add(num(1), num(2)));
}

// ── Rc ──

#[test]
fn share_adds_owners_not_copies() {
    let rc = Rc::new(String::from("shared"));
    let others = share(&rc, 3);
    assert_eq!(others.len(), 3);
    assert!(others.iter().all(|o| Rc::ptr_eq(o, &rc)));
    assert_eq!(owners(&rc), 4);
    drop(others);
    assert_eq!(owners(&rc), 1);
}

#[test]
fn owners_counts_strong_refs() {
    let a = Rc::new(5);
    assert_eq!(owners(&a), 1);
    let b = Rc::clone(&a);
    assert_eq!(owners(&b), 2);
    let _w = Rc::downgrade(&a);
    assert_eq!(owners(&a), 2, "a Weak doesn't count as an owner");
}

#[test]
fn same_allocation_is_identity_not_equality() {
    let a: Rc<str> = Rc::from("hi");
    let b = Rc::clone(&a);
    let c: Rc<str> = Rc::from("hi");
    assert!(same_allocation(&a, &b));
    assert!(!same_allocation(&a, &c), "equal text, different allocations");
}

#[test]
fn into_inner_if_unique_gives_rc_back_when_shared() {
    let rc = Rc::new(vec![1, 2]);
    let other = Rc::clone(&rc);
    let back = into_inner_if_unique(rc).expect_err("shared, so it can't be unwrapped");
    assert!(Rc::ptr_eq(&back, &other));
    drop(other);
    assert_eq!(into_inner_if_unique(back), Ok(vec![1, 2]));
}

#[test]
fn take_or_clone_moves_when_unique() {
    let rc = Rc::new(vec![1, 2, 3]);
    let ptr = rc.as_ptr();
    let v = take_or_clone(rc);
    assert_eq!(v, [1, 2, 3]);
    assert_eq!(v.as_ptr(), ptr, "the only owner moves the buffer out");
    let shared = Rc::new(vec![4]);
    let keep = Rc::clone(&shared);
    assert_eq!(take_or_clone(shared), [4]);
    assert_eq!(*keep, [4]);
}

#[test]
fn push_cow_clones_only_when_shared() {
    let mut a = Rc::new(vec![1]);
    let b = Rc::clone(&a);
    push_cow(&mut a, 2);
    assert_eq!(*a, [1, 2]);
    assert_eq!(*b, [1], "the other owner doesn't see the push");
    assert!(!Rc::ptr_eq(&a, &b));
    let before = Rc::as_ptr(&a);
    push_cow(&mut a, 3);
    assert_eq!(Rc::as_ptr(&a), before, "unique now: mutated in place");
    assert_eq!(*a, [1, 2, 3]);
}

// ── Weak parent pointers ──

#[test]
fn new_node_is_a_lone_root() {
    let n = TreeNode::new("root");
    assert_eq!(n.name, "root");
    assert!(n.children.borrow().is_empty());
    assert_eq!(parent_name(&n), None);
}

#[test]
fn add_child_links_both_ways() {
    let root = TreeNode::new("root");
    let kid = TreeNode::new("kid");
    add_child(&root, Rc::clone(&kid));
    assert_eq!(parent_name(&kid).as_deref(), Some("root"));
    assert_eq!(root.children.borrow().len(), 1);
    assert!(Rc::ptr_eq(&root.children.borrow()[0], &kid));
    assert_eq!(owners(&root), 1, "the child doesn't own its parent");
    assert_eq!(owners(&kid), 2);
}

#[test]
fn path_climbs_to_the_root() {
    let root = TreeNode::new("usr");
    let mid = TreeNode::new("local");
    let leaf = TreeNode::new("bin");
    add_child(&root, Rc::clone(&mid));
    add_child(&mid, Rc::clone(&leaf));
    assert_eq!(path(&leaf), ["usr", "local", "bin"]);
    assert_eq!(path(&root), ["usr"]);
}

#[test]
fn watchers_counts_weak_refs() {
    let root = TreeNode::new("root");
    assert_eq!(watchers(&root), 0);
    for name in ["a", "b", "c"] {
        add_child(&root, TreeNode::new(name));
    }
    assert_eq!(watchers(&root), 3, "each child's parent link is a Weak");
}

#[test]
fn parent_link_dies_with_the_parent() {
    let kid = TreeNode::new("kid");
    {
        let root = TreeNode::new("root");
        add_child(&root, Rc::clone(&kid));
        assert_eq!(parent_name(&kid).as_deref(), Some("root"));
    }
    assert_eq!(parent_name(&kid), None, "upgrade fails once the parent is gone");
}

#[test]
fn dropping_the_root_frees_the_whole_tree() {
    let before = dropped_nodes();
    let root = TreeNode::new("root");
    let a = TreeNode::new("a");
    add_child(&root, Rc::clone(&a));
    add_child(&a, TreeNode::new("a1"));
    add_child(&root, TreeNode::new("b"));
    let watch = Rc::downgrade(&a);
    drop(a);
    assert_eq!(dropped_nodes(), before, "still owned by root");
    drop(root);
    assert_eq!(dropped_nodes(), before + 4, "no Rc cycle: every node is dropped");
    assert!(watch.upgrade().is_none());
}
