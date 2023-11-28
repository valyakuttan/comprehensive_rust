#![allow(dead_code)]

/// # 3.1 Using Option
///
/// Our definition of `Link` is:
///
/// ```
///
/// enum Link {
///     Empty,
///     More(Box<Node>),
/// }
///
/// ```
///
/// Just `Option<Box<Node>>`.  `Option` has some really nice methods
/// that we've been manually implementing ourselves. Let's not do that,
/// and replace everything with `Option`. First, we'll do it naively by
/// just renaming everything to use `Some` and `None`:
///
///
/// ```
///
/// use std::mem;
///
/// pub struct List {
///     head: Link,
/// }
///
/// // yay type aliases!
/// type Link = Option<Box<Node>>;
///
/// struct Node {
///     elem: i32,
///     next: Link,
/// }
///
/// impl List {
///     pub fn new() -> Self {
///         List { head: None }
///     }
///
///     pub fn push(&mut self, elem: i32) {
///         let new_node = Box::new(Node {
///             elem: elem,
///             next: mem::replace(&mut self.head, None),
///         });
///
///         self.head = Some(new_node);
///     }
///
///     pub fn pop(&mut self) -> Option<i32> {
///         match mem::replace(&mut self.head, None) {
///             None => None,
///             Some(node) => {
///                 self.head = node.next;
///                 Some(node.elem)
///             }
///         }
///     }
/// }
///
/// impl Drop for List {
///     fn drop(&mut self) {
///         let mut cur_link = mem::replace(&mut self.head, None);
///         while let Some(mut boxed_node) = cur_link {
///             cur_link = mem::replace(&mut boxed_node.next, None);
///         }
///     }
/// }
///
/// ```
///
/// - First `mem::replace(&mut option, None)`is such an incredibly common idiom that
///   `Option` actually just went ahead and made it a method: `take`.
///
/// - Second, `match option { None => None, Some(x) => Some(y) }` is such an incredibly
///   common idiom that it was called `map`. `map` takes a function to execute on the
///   `x` in the `Some(x)` to produce the `y` in `Some(y)`. We could write a proper fn
///   and pass it to map, but we'd much rather write what to do inline.
///
/// - The way to do this is with a closure. Closures are anonymous functions with an
///   extra super-power: they can refer to local variables outside the closure! This
///   makes them super useful for doing all sorts of conditional logic. The only place
///   we do a `match` is in pop, so let's just rewrite that:
///
/// ```
///
/// pub fn pop(&mut self) -> Option<i32> {
///     self.head.take().map(|node| {
///         self.head = node.next;
///         node.elem
///     })
/// }
///
/// ```
///

pub fn main() {}
