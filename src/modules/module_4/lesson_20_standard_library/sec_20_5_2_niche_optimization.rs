#![allow(dead_code, clippy::needless_doctest_main)]

/// # Niche Optimization
/// 
/// A `Box` cannot be empty, so the pointer is always valid and non-null.
/// This allows the compiler to optimize the memory layout:
/// 
/// ```
/// 
/// #[derive(Debug)]
/// enum List<T> {
///     Cons(T, Box<List<T>>),
///     Nil,
/// }
///
/// fn main() {
///     let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
///     println!("{list:?}");
///}
/// 
/// ```
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

#[allow(dead_code)]
pub fn main() {}