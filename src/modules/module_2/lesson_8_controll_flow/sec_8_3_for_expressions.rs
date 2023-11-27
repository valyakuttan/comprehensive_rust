#![allow(dead_code, clippy::needless_doctest_main, clippy::needless_range_loop)]

/// # for loops
///
/// The `for` loop is closely related to the `while let` loop.
/// It will automatically call `into_iter()` on the expression
/// and then iterate over it:
///
/// - You can use break and continue inside the loop.
///
/// ```
/// fn main() {
///     let v = vec![10, 20, 30];
///
///     for x in v {
///         println!("x: {x}");
///     }
///     
///     for i in (0..10).step_by(2) {
///         println!("i: {i}");
///    }
/// }
/// ```
///
/// - `(0..10)` is a range that implements an `Iterator` trait.
///
/// - `step_by` is a method that returns another `Iterator` that
///    skips every other element.
///

#[allow(dead_code)]
pub fn main() {
    let mut v = vec![10; 20];
    let mut idx = 1;

    for e in v.iter_mut().skip(1).step_by(2) {
        *e *= idx;
        idx += 1;
    }

    println!("{v:?}");
}
