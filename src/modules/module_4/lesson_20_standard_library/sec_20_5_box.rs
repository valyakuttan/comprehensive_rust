/// # Box
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

#[allow(dead_code)]
pub fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
    println!("five: {}", five); // * is unnecessary

}
