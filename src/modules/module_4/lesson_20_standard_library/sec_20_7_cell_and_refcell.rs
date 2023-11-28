/// # `Cell` and `RefCell`
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
use std::cell::RefCell;
use std::rc::Rc;

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

#[allow(dead_code)]
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
