#![allow(dead_code, clippy::needless_doctest_main, clippy::needless_range_loop)]

/// # `if` expression
///
/// you can use `if` as an expression. The last expression of each block
/// becomes the value of the if expression:
///
/// ```
/// fn main() {
///     let mut x = 10;
///     x = if x % 2 == 0 {
///         x / 2
///     } else {
///        3 * x + 1
///    };
/// }
/// ```
///
///
/// Because `if` is an expression and must have a particular
/// type, both of its branch blocks must have the same type.

#[allow(dead_code)]
pub fn main() {}
