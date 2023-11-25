/// # Box with Recursive Data Structures
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


#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[allow(dead_code)]
pub fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}
