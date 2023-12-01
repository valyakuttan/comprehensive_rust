#![allow(dead_code, clippy::needless_range_loop)]

/// # 6.4 Blocks and Scopes
///
/// ## Blocks
///
/// A block in Rust contains a sequence of expressions. Each
/// block has a value and a type, which are those of the
/// last expression of the block:
///
/// ```
///
/// fn block_example() {
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
///
/// ```
///
/// If the last expression ends with `;`, then the resulting value and type
/// is `()`. The same rule is used for functions: the value of the function
/// body is the return value:
///
/// ```
///
/// fn double(x: i32) -> i32 {
///     x + x
/// }
///
/// fn function_return value_example() {
///     println!("double: {}", double(7));
/// }
///
/// ```
///
/// ## Scopes and Shadowing
///
/// You can shadow variables, both those from outer scopes and
/// variables from the same scope:
///
///
/// - Shadowing is different from mutation, because after
///   shadowing both variable’s memory locations exist at the same time.
///   Both are available under the same name, depending where you use it
///   in the code.
///
/// - A shadowing variable can have a different type.
///
/// Shadowing looks obscure at first, but is convenient for holding on to
/// values after `.unwrap()`.
///
/// ```
///
/// let a = Some(5);
/// let a = a.unwrap();
///
/// ```
///
/// The following code demonstrates why the compiler can’t simply reuse memory
/// locations when shadowing an immutable variable in a scope, even if the type
/// does not change.
///
/// ```
///
/// fn shadow_example() {
///    let a = 1;
///    let b = &a;
///    let a = a + 1;
///    println!("{a} {b}");
/// }
/// ``

pub fn main() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}
