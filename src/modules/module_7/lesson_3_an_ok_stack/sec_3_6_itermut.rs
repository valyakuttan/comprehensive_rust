#![allow(dead_code)]

/// # IterMut
///
///  Our implementation of `Iterator` for `Iter`:
///
/// ```
///
/// impl<'a, T> Iterator for Iter<'a, T> {
///     type Item = &'a T;
///
///     fn next(&mut self) -> Option<Self::Item> { /* stuff */ }
/// }
///
/// ```
///
/// Can be desugar to:
///
/// ```
///
/// impl<'a, T> Iterator for Iter<'a, T> {
///     type Item = &'a T;

///     fn next<'b>(&'b mut self) -> Option<&'a T> { /* stuff */ }
/// }
///
/// ```
///
/// The signature of `next` establishes no constraint between the lifetime
/// of the input and the output. This is definitely fine for shared
/// references because the whole point is that you can have tons of them at
/// once. However mutable references can't coexist. The whole point is that
/// they're exclusive.
///
/// If we change everything in  `Iter` as mutable, it becomes:
///
/// ```
///
/// pub struct IterMut<'a, T> {
///     next: Option<&'a mut Node<T>>,
/// }
///
/// impl<T> List<T> {
///     pub fn iter_mut(&self) -> IterMut<'_, T> {
///         IterMut { next: self.head.as_deref_mut() }
///     }
/// }
///
/// impl<'a, T> Iterator for IterMut<'a, T> {
///     type Item = &'a mut T;
///
///     fn next(&mut self) -> Option<Self::Item> {
///         self.next.map(|node| {
///             self.next = node.next.as_deref_mut();
///             &mut node.elem
///         })
///     }
/// }
///
/// ```
///
/// Which gives the errors
///
/// ```
///
/// error[E0596]: cannot borrow `self.head` as mutable,
/// as it is behind a `&` reference
/// ....
///
/// error[E0507]: cannot move out of borrowed content
/// ````
///
/// For the first error you can't upgrade a shared reference to a mutable
/// one, so `iter_mut` needs to take `&mut self`.
///
/// ## Move semantics and Copy types
///
/// In the **ownership** model `Copy` types are known to be perfectly
/// copyable by a bitwise copy. As such, they have a super power: when
/// moved, the old value is still usable. As a consequence, you can even
/// move a `Copy` type out of a reference without replacement.
///
/// Shared references are also Copy! Because `&` is copy, `Option<&>` is
/// also Copy. So when we did `self.next.map` it was fine because the
/// `Option` was just copied. Now we can't do that, because `&mut` isn't
/// Copy (if you copied an `&mut`, you'd have two `&mut`'s to the same
/// location in memory, which is forbidden). Instead, we should properly
/// `take` the `Option` to get it.
///
/// ```
///
/// fn next(&mut self) -> Option<Self::Item> {
///     self.next.take().map(|node| {
///         self.next = node.next.as_deref_mut();
///         &mut node.elem
///     })
/// }
///
/// ```

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub fn main() {}
