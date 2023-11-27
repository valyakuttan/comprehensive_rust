#![allow(
    dead_code,
    clippy::needless_doctest_main,
    clippy::needless_range_loop,
    clippy::while_let_on_iterator
)]

/// # break and continue
///
/// - If you want to exit a loop early, use `break`,
///
/// - If you want to immediately start the next iteration use `continue`.
///
/// - Both `continue` and `break` can optionally take a label argument
///   which is used to break out of nested loops:

#[allow(dead_code)]
pub fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}
