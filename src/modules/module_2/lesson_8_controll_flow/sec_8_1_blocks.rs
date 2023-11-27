#![allow(dead_code, clippy::needless_doctest_main, clippy::needless_range_loop)]

/// # Blocks
/// A block in Rust contains a sequence of expressions. Each
/// block has a value and a type, which are those of the
/// last expression of the block:
/// 
/// ```
/// fn main() {
///     let x = {
///         let y = 10;
///         println!("y: {y}");
///         let z = {
///             let w = {
///                 3 + 4
///              };
///              println!("w: {w}");
///              y * w
///         };
///         println!("z: {z}");
///         z - y
///     };
///     println!("x: {x}");
///  }
/// ```
/// 
/// If the last expression ends with ;, then the resulting value and type is `()`.
/// The same rule is used for functions: the value of the function body is
/// the return value:
/// 
/// ```
/// fn double(x: i32) -> i32 {
///     x + x
/// }
///
/// fn main() {
///     println!("double: {}", double(7));
/// }
/// ```

pub fn main() {}