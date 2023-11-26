#![allow(dead_code)]

/// # Push
///
/// Let's write pushing a value onto a list. `push` mutates
/// the list, so we'll want to take `&mut self`. We also
/// need to take an `i32` to `push`:
///
/// ```
///
/// impl List {
///     pub fn push(&mut self, elem: i32) {
///         todo!()
///     }
/// }
///
/// ```
///
/// If we fill in the method body as
///
/// ```
///
/// pub fn push(&mut self, elem: i32) {
///     let new_node = Box::new(Node {
///         elem: elem,
///         next: self.head,
///     });
///
///     self.head = Link::More(new_node);
/// }
///
/// ```
///
/// The compiler will give the error message:
///
/// ```
///
/// error[E0507]: cannot move out of `self.head` which is behind a mutable reference
///
/// ```
///
/// Rust wont allow this for various reasons -- the most serious being exception safety.
/// So we need some other way to do that. The function `mem::replace` is incredibly
/// useful in this situation. It lets us steal a value out of a borrow by replacing
/// it with another value.
///
/// ```
/// use std::mem;
///
/// pub fn push(&mut self, elem: i32) {
///     let new_node = Box::new(Node {
///         elem,
///         next: mem::replace(&mut self.head, Link::Empty),
///     });
///
///     self.head = Link::More(new_node);
/// }
///
/// ```
/// 
/// Here we replace `self.head` temporarily with `Link::Empty` before
/// replacing it with the new head of the list. 
///  

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
    
}

pub fn main() {}
