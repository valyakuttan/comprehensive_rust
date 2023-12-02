use std::cell::RefCell;
use std::rc::Rc;

/// # 22.3 Interior Mutability
///
/// Rust provides a few safe means of modifying a value given only
/// a shared reference to that value. All of these replace copmile-time
/// checks with runtime checks.
///
/// ## `Cell` and `RefCell`
///
/// `Cell` and `RefCell` implement what Rust calls **interior mutability**:
/// mutation of values in an immutable context.
///
/// `Cell` is typically used for simple types, as it requires copying or
/// moving values. More complex interior types typically use `RefCell`,
/// which tracks shared and exclusive references at runtime and panics
/// if they are misused.
///
/// - To do anything with a `Node`, you must call a `RefCell` method, usually
///   `borrow` or `borrow_mut`.
///
/// - If we were using `Cell` instead of `RefCell` in this example, we would
///   have to move the `Node` out of the `Rc` to push children, then move it
///   back in. This is safe because there’s always one, un-referenced value
///   in the cell, but it’s not ergonomic.
///
///
///

#[derive(Debug, Default)]
struct Node {
    value: i64,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i64) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            ..Node::default()
        }))
    }

    fn sum(&self) -> i64 {
        self.value + self.children.iter().map(|c| c.borrow().sum()).sum::<i64>()
    }
}

pub fn main() {
    let root = Node::new(1);
    root.borrow_mut().children.push(Node::new(5));
    let subtree = Node::new(10);
    subtree.borrow_mut().children.push(Node::new(11));
    subtree.borrow_mut().children.push(Node::new(12));
    root.borrow_mut().children.push(subtree);

    println!("graph: {root:#?}");
    println!("graph sum: {}", root.borrow().sum());
}
