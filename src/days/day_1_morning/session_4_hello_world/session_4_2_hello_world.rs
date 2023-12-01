#![allow(dead_code)]

/// # 4.2 Hello World!
///
/// - Functions are introduced with `fn``.
///
/// - Blocks are delimited by curly braces like in C and C++.
///
/// - The `main` function is the entry point of the program.
///
/// -  Rust has hygienic macros, `println!` is an example of this.
///
/// - Rust strings are **UTF-8** encoded and can contain any Unicode
///   character.
///
/// - Rust uses macros for situations where you want to have a variable
///   number of arguments (no function overloading).
///
/// - Macros being **hygienic** means they don’t accidentally capture
///   identifiers from the scope they are used in. Rust macros are
///   actually only **partially hygienic**.

pub fn main() {
    println!("Hello 🌍!");
}
