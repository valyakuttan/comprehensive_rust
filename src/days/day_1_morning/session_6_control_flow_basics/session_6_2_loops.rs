#![allow(dead_code)]

/// # 6.2 Loops
///
/// There are three looping keywords in Rust: `while`, `loop`, and `for`:
///
/// ## while
///
/// The `while` keyword works much like in other languages, executing the
/// loop body as long as the condition is true.
///
/// ```
///
/// fn while_loop() {
///     let mut x = 200;
///     while x >= 10 {
///         x /= 2;
///     }
///     println!("Final x: {x}");
/// }
///
/// ```
///
/// ## `for`
///
/// The `for` loop iterates over ranges of values:
///
/// ```
///
/// fn for_loop() {
///     for x in 1..5 {
///         println!("x: {x}");
///     }
/// }
///
/// ```
///
/// ## loop
///
/// The `loop` statement just loops forever, until a `break`.
///

pub fn main() {
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}
