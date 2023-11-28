#![allow(dead_code)]

/// # 2.7 Drop
///
/// Like C++, Rust uses destructors to automatically clean up resources
/// when they're done with. A type has a destructor if it implements a
/// trait called `Drop`. Traits are Rust's fancy term for interfaces.
/// The `Drop` trait has the following interface:
///
/// ```
///
/// pub trait Drop {
///     fn drop(&mut self);
/// }
///
/// ```
///
/// For the `List`, automatic handling of dropping objects is bad since
/// we can't drop the contents of the Box after deallocating, so there's
/// no way to drop in a tail-recursive manner! Instead we're going to have
/// to manually write an iterative drop for List that hoists nodes out of
/// their boxes.
///
/// ```
///
/// impl Drop for List {
///     fn drop(&mut self) {
///         let mut cur_link = mem::replace(&mut self.head, Link::Empty);
///
///         // `while let` == "do this thing until this pattern doesn't match"
///         while let Link::More(mut boxed_node) = cur_link {
///
///             cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
///             // boxed_node goes out of scope and gets dropped here;
///             // but the Node's `next` field has been set to Link::Empty
///             // so no unbounded recursion occurs.
///         }
///     }
/// }
///
/// ```
///

pub fn main() {}
