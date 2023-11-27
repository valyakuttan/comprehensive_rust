#![allow(dead_code)]

/// # Iter
///
/// The basic logic we want is to hold a pointer to the current node we
/// want to yield next. Because that node may not exist (the list is
/// empty or we're otherwise done iterating), we want that reference to
/// be an `Option`. When we yield an element, we want to proceed to the
/// current node's next node.
///
/// Let's try:
///
/// ```
///
/// pub struct Iter<T> {
///     next: Option<&Node<T>>,
/// }
///
/// impl<T> List<T> {
///     pub fn iter(&self) -> Iter<T> {
///         Iter { next: self.head.map(|node| &node) }
///     }
/// }
///
/// impl<T> Iterator for Iter<T> {
///     type Item = &T;
///     fn next(&mut self) -> Option<Self::Item> {
///         self.next.map(|node| {
///             self.next = node.next.map(|node| &node);
///             &node.elem
///         })
///     }
/// }
///
/// ```
///
/// But the compiler gives the error
///
/// ```
/// error[E0106]: missing lifetime specifier
///
/// ````
///
/// So we need to add lifetimes in function and type signatures:
///
/// ```
///
/// // Iter is generic over *some* lifetime, it doesn't care
/// pub struct Iter<'a, T> {
///     next: Option<&'a Node<T>>,
/// }
///
/// // No lifetime here, List doesn't have any associated lifetimes
/// impl<T> List<T> {
///     pub fn iter(&self) -> Iter<T> {
///        Iter { next: self.head.map(|node| &node) }
///     }
/// }
///
/// // We *do* have a lifetime here, because Iter has one that we need to define
/// impl<'a, T> Iterator for Iter<'a, T> {
///     // Need it here too, this is a type declaration
///     type Item = &'a T;
///
///     // None of this needs to change, handled by the above.
///     // Self continues to be incredibly hype and amazing
///     fn next(&mut self) -> Option<Self::Item> {
///         self.next.map(|node| {
///             self.next = node.next.map(|node| &node);
///             &node.elem
///         })
///     }
/// }
///
/// ```
///
/// We fixed our lifetime errors but now we're getting some new type errors.
///
/// ```
///
/// error[E0308]: mismatched types
///
/// ...
///
/// expected type `std::option::Option<&second::Node<T>>`
/// found type `std::option::Option<&std::boxed::Box<second::Node<T>>>`
///
/// ```
///
/// We want to be storing `&Node`'s, but we're getting `&Box<Node>`s. We just
/// need to dereference the `Box` before we take our reference:
///
/// ```
///
/// impl<T> List<T> {
///     pub fn iter<'a>(&'a self) -> Iter<'a, T> {
///         Iter { next: self.head.map(|node| &*node) }
///     }
/// }
///
/// impl<'a, T> Iterator for Iter<'a, T> {
///     type Item = &'a T;
///     fn next(&mut self) -> Option<Self::Item> {
///         self.next.map(|node| {
///             self.next = node.next.map(|node| &*node);
///             &node.elem
///         })
///     }
/// }
///
/// ```
///
/// We got the error message:
///
/// ```
///
/// error[E0515]: cannot return reference to local data `*node`
///
/// ```
///
/// We forgot `as_deref`, so we're moving the box into map, which means
/// it would be dropped, which means our references would be dangling.
///
/// Also `as_deref` added another layer of indirection we need to remove:
///

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
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

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}

pub fn main() {}
