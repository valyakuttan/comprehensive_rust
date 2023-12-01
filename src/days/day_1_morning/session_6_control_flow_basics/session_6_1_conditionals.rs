#![allow(dead_code)]

/// # 6.1 Conditionals
///
/// Much of the Rust syntax is similar to C, C++ or Java:
///
/// - Blocks are delimited by curly braces.
///
/// - Line comments are started with //, block comments are delimited
///   by `/* ... */`.
///
/// - Keywords like `if` and `while` work the same.
///
/// - Variable assignment is done with `=`, comparison is done with `==`.
///
/// ## `if` expressions
///
/// `if` expressions are exactly like `if` statements in other languages:
///
/// ```
///
/// fn if_statement() {
///     let x = 10;
///     if x < 20 {
///         println!("small");
///     } else if x < 100 {
///         println!("biggish");
///     } else {
///         println!("huge");
///     }
/// }
///
/// ```
///
/// In addition, you can use `if` as an expression. The last expression of each
/// block becomes the value of the `if` expression:
///
/// Because `if` is an expression and must have a particular type, both of its
/// branch blocks must have the same type.
///
/// When `if` is used in an expression, the expression must have a `;` to
/// separate it from the next statement.
///

pub fn main() {
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}
