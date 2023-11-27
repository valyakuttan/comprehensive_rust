#![allow(dead_code, clippy::needless_doctest_main, clippy::needless_range_loop)]

/// # Modern Features
/// 
/// Rust is built with all the experience gained in the last decades.
/// 
/// Key features:
/// 
/// - **Zero-cost** abstractions, similar to C++, means that you don’t
///   have to ‘pay’ for higher-level programming constructs with memory
///   or CPU. For example, writing a loop using `for` should result in
///   roughly the same low level instructions as using the
///   `.iter().fold()` construct.
/// 
/// - It may be worth mentioning that Rust enums are **Algebraic Data Types**,
///   also known as **sum types**, which allow the type system to express things
///   like `Option<T>` and `Result<T, E>`.
/// 
/// - The Rust compiler is significantly more talkative than other compilers. It
///   will often provide you with actionable feedback, ready to copy-paste into your code.
/// 

pub fn main() { // Program entry point
    let mut x: i32 = 6; // Mutable variable binding
    print!("{x}"); // Macro for printing, like printf
    while x != 1 {
        // No parenthesis around expression
        if x % 2 == 0 {
            // Math like in other languages
            x /= 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}
