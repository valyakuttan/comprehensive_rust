#![allow(dead_code, clippy::needless_doctest_main)]

/// # Option and Result
///
/// These types represent optional data:
///
/// ```
/// fn main() {
///     let numbers = vec![10, 20, 30];
///     let first: Option<&i8> = numbers.first();
///     println!("first: {first:?}");
///
///     let arr: Result<[i8; 3], Vec<i8>> = numbers.try_into();
///     println!("arr: {arr:?}");
/// }
/// ```
///
/// - `Option` and `Result` are widely used not just in the standard library.
///
/// - `Option<&T>` has zero space overhead compared to `&T`.
///
/// - `Result` is the standard type to implement error handling.
///
/// - `try_into` attempts to convert the vector into a fixed-sized array.
///   This can fail:
///
///   - If the vector has the right size, `Result::Ok` is returned with the array.
///
///   - Otherwise, `Result::Err` is returned with the original vector.
///
/// ## The question mark operator, `?`
/// 
/// The question mark operator, `?`, hides some of the boilerplate of
/// propagating values up the call stack.
/// 
/// 
/// It replaces the code below:
/// ```
/// 
/// fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
///     let a = stack.pop();
///     let b = stack.pop();
///
///     match (a, b) {
///         (Some(x), Some(y)) => Some(x + y),
///         _ => None,
///     }
/// }
///
/// ```
/// 
/// With this:
/// 
/// ```
/// 
/// fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
///     Some(stack.pop()? + stack.pop()?)
/// }
/// 
/// ```
/// 
/// - Ending the expression with `?` will result in the `Some`’s unwrapped value,
///   unless the result is `None`, in which case `None` is returned early from
///   the enclosing function.
/// 
/// - `?` can be used in functions that return `Option` because of the early return
///   of `None` that it provides.
/// 

#[allow(dead_code)]
pub fn main() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {first:?}");

    let cnumbers = numbers.clone();

    let arr3: Result<[i8; 3], Vec<i8>> = numbers.try_into();
    println!("arr: {arr3:?}");

    let arr2: Result<[i8; 2], Vec<i8>> = cnumbers.try_into();
    println!("arr: {arr2:?}");
}
