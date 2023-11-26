#![allow(dead_code)]

/// # 2.1 Basic Data Layout
///
/// If we borrow the functional language's definition of `List`
///
/// ```
///
/// List a = Empty | Elem a (List a)
///
/// ```
/// to Rust, it will be something like
///
/// ```
///
/// pub enum List {
///     Empty,
///     Elem(i32, List),
/// }
///
/// ````
///
/// which won't compile because of the recursive nature of the
/// definition, since compiler can't figure out the size of the
/// `List`, which is needed to layout the `enum` in memory.
///
/// So our definition must be something like:
///
/// ```
///
/// pub enum List {
///     Empty,
///     Elem(i32, Box<List>),
/// }
///
/// ```
///
/// This will compile but the definition is awful. Consider a list
/// with two elements.
///
/// ```
///  
/// [] = Stack
/// () = Heap
///
/// [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
///
/// ```
///
/// - We're allocating a node that just says "I'm not actually a Node"
///
/// - One of our nodes isn't heap-allocated at all.
///
///  Consider the following potential layout for a list:
///
/// ```
///
/// [ptr] -> (Elem A, ptr) -> (Elem B, *null*)
///
/// ```
///
/// In this layout, we have:
///
/// - All nodes are heap-allocated.
///
/// - The absence of the junk from the first layout.
///
/// Even though `Empty` is a single bit of information, it necessarily consumes
/// enough space for a pointer and an element, because it has to be ready to
/// become an `Elem` at any time. Therefore the first layout heap allocates an
/// extra element that's just full of junk, consuming a bit more space than the
/// second layout.
///
/// Consider an alternate layout:
///
/// ```
///
/// pub enum List {
///     Empty,
///     ElemThenEmpty(i32),
///     ElemThenNotEmpty(i32, Box<List>),
/// }
///
/// ```
///
/// This will totally avoids allocating the Empty case, reducing the total number
/// of heap allocations by one. But in doing so it manages to waste even more space!
/// This is because the previous layout took advantage of the null pointer
/// optimization.
///
/// Every enum has to store a tag to specify which variant of the enum its bits
/// represent. However, if we have a special kind of enum:
///
/// ```
///
/// enum Foo {
///     A,
///    B(ContainsANonNullPtr),
/// }
///
/// ```
///
/// The null pointer optimization kicks in, which eliminates the space needed for
/// the tag. If the variant is `A`, the whole enum is set to all 0's. Otherwise,
/// the variant is `B`. This works because `B` can never be all 0's, since it
/// contains a non-zero pointer.
///
/// Now consider the definition:
///
/// ```
///
/// struct Node {
///     elem: i32,
///     next: List,
/// }
///
/// pub enum List {
///     Empty,
///     More(Box<Node>),
/// }
///
/// ```
///
/// Which satisfies the properties:
///
/// - Tail of a list never allocates extra junk.
///
/// - `enum` is in delicious null-pointer-optimized form.
///
/// - All elements are uniformly allocated.
///
/// But since `Node` is private and `List` is public the compiler will
/// spits out an error message, because we are leaking an implementation
/// detail.
///
/// Let's make `List` a `struct`, so that we can hide the implementation
/// details. Because `List` is a struct with a single field, its size is
/// the same as that field, a zero-cost abstraction.
///

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
}

pub fn main() {}
