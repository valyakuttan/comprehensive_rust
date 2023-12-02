/// # 20.1 `Box<T>`
///
/// - `Box` is an owned pointer to data on the heap.
///
/// - `Box<T>` implements `Deref<Target = T>`, which means that you
///   can call methods from `T` directly on a `Box<T>`.
///
/// - `Box` is like `std::unique_ptr` in C++, except that it’s guaranteed
///   to be not `null`.
///
/// - A `Box` can be useful when you:
///
///   - have a type whose size that can’t be known at compile time, but the
///     Rust compiler wants to know an exact size.
///
///   - want to transfer ownership of a large amount of data. To avoid copying
///     large amounts of data on the stack, instead store the data on the heap
///     in a `Box` so only the pointer is moved.
///
///  
/// ## Box with Recursive Data Structures
///
/// Recursive data types or data types with dynamic sizes
/// need to use a `Box`:
///
/// ```
///
/// enum List<T> {
///     Cons(T, Box<List<T>>),
///     Nil,
/// }
///
/// ```
///
///
///
///              Stack                           Heap
///  -----------------------     --------------------------------------
/// |      list             |   |                                      |
/// |      -------------    |   |   ------------     ---------------   |
/// |     | Cons | 1 | ===========>| Cons | 2 | ==> | Nil | // | // |  |
/// |      -------------    |   |   ------------     ---------------   |
/// |                       |   |                                      |
///  -----------------------     --------------------------------------
///
///
///
///
/// - If `Box` was not used and we attempted to embed a `List`
///   directly into the `List`, the compiler would not compute
///   a fixed size of the struct in memory (`List` would be of
///   infinite size).
///
/// - `Box` solves this problem as it has the same size as a regular
///   pointer and just points at the next element of the `List` in the heap.
///
/// ## Niche Optimization
///
/// A `Box` cannot be empty, so the pointer is always valid and non-null.
/// This allows the compiler to optimize the memory layout:
///
///
/// So after the optimization the layout becomes
///
///         Stack                           Heap
///  -----------------     -----------------------------
/// |      list       |   |                             |
/// |      -------    |   |  ------    -----------      |
/// |     | 1 | ===========>| 2 | ==> | // | null |     |
/// |      ------     |   |  ------    -----------      |
/// |                 |   |                             |
///  -----------------     -----------------------------
///
///

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}
