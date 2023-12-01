#![allow(dead_code)]

/// # 6.6 Macros
///
/// Macros are expanded into Rust code during compilation, and can take
/// a variable number of arguments. They are distinguished by a `!` at
/// the end. The Rust standard library includes an assortment of useful
/// macros.
///
/// - `println!(format, ..)` prints a line to standard output, applying
///   formatting described in `std::fmt`.
///
/// - `format!(format, ..)` works just like `println!` but returns the result
///   as a string.
///
/// - `dbg!(expression)` logs the value of the expression and returns it.
///
/// - `todo!()` marks a bit of code as not-yet-implemented. If executed, it
///   will panic.
///
/// - `unreachable!()` marks a bit of code as unreachable. If executed, it
///   will panic.
///

fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizz_buzz(_n: u32) -> u32 {
    todo!()
}

pub fn main() {
    let n = 13;
    println!("{n}! = {}", factorial(4));
}
