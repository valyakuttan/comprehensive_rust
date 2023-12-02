use std::cell::RefCell;
use std::rc::Rc;

/// # 20.2 Rc
///
/// `Rc` is a reference-counted shared pointer. Use this when you
/// need to refer to the same data from multiple places:
///
/// ```
/// 
/// pub fn rc_example() {
///     let a = Rc::new(10);
///     let b = Rc::clone(&a);
///
///     println!("a: {a}");
///     println!("b: {b}");
/// }
/// 
/// ```
/// - `Rc`’s count ensures that its contained value is valid for as
///   long as there are references.
///
/// - `Rc` in Rust is like `std::shared_ptr` in C++.
///
/// - `Rc::clone` is cheap: it creates a pointer to the same allocation
///   and increases the reference count. Does not make a deep clone and
///   can generally be ignored when looking for performance issues in code.
///
/// - `make_mut` actually clones the inner value if necessary (“clone-on-write”)
///   and returns a mutable reference.
///
/// - Use `Rc::strong_count` to check the reference count.
///
/// - `Rc::downgrade` gives you a weakly reference-counted object to create cycles
///   that will be dropped properly (likely in combination with `RefCell`).
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
