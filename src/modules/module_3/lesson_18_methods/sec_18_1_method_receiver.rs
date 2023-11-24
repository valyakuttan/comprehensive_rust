/// # Method Receiver
///
/// The `&self` indicates that the method borrows the object immutably.
/// There are other possible receivers for a method:
///
/// - `&self`: borrows the object from the caller using a shared
///   and immutable reference. The object can be used again afterwards.
///
/// - `&mut self`: borrows the object from the caller using a unique and
///   mutable reference. The object can be used again afterwards.
///
/// - `self`: takes ownership of the object and moves it away from the
///   caller. The method becomes the owner of the object. The object
///   will be dropped (deallocated) when the method returns, unless
///   its ownership is explicitly transmitted. Complete ownership does
///   not automatically mean mutability.
///
/// - `mut self`: same as above, but the method can mutate the object.
///
/// - No receiver: this becomes a static method on the struct. Typically
///   used to create constructors which are called `new` by convention.
///
/// Beyond variants on `self`, there are also special wrapper types allowed
/// to be receiver types, such as `Box<Self>`.
///
use std::{rc::Rc, sync::Arc};

#[derive(Debug)]
struct Bar(i32);

impl Bar {
    fn consuming(self) {
        println!("self: {self:?}");
    }
    fn reference(&self) {
        println!("&self: {self:?}");
    }
    fn mut_reference(&mut self) {
        println!("&mut self: {self:?}");
    }
    fn boxed(self: Box<Bar>) {
        println!("Box value: {}", self.0);
    }
    fn ref_count(self: Rc<Bar>) {
        println!("Rc value: {}", self.0);
    }
    fn atomic_ref_count(self: Arc<Bar>) {
        println!("Arc value: {}", self.0);
    }
}

#[allow(dead_code)]
pub fn main() {
    let bar = Bar(100);

    bar.reference();
    bar.consuming(); // bar is moved

    let mut baz = Bar(100);
    baz.mut_reference();

    Box::new(Bar(50)).boxed();

    Rc::new(Bar(50)).ref_count();

    Arc::new(Bar(50)).atomic_ref_count();
}
