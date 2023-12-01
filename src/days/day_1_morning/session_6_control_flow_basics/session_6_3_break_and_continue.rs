#![allow(dead_code, clippy::while_let_on_iterator)]

/// # 6.3 break and continue
///
/// If you want to exit a loop early, use `break`. For `loop`, this can
/// take an optional expression that becomes the value of the `loop`
/// expression.
///
/// ```
///
/// fn break_example() {
///     let (mut a, mut b) = (100, 52);
///     let result = loop {
///         if a == b {
///             break a;
///         }
///         if a < b {
///             b -= a;
///         } else {
///             a -= b;
///         }
///     };
///     println!("{result}");
/// }
///
/// ```
///
/// - If you want to immediately start the next iteration use `continue`.
///
/// - Both `continue` and `break` can optionally take a label argument
///   which is used to break out of nested loops:
///
/// - Note that `loop` is the only looping construct which returns a
///   non-trivial value. This is because it’s guaranteed to be entered at
///   least once (unlike `while` and `for` loops).
///

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
