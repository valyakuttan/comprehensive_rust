#![allow(
    dead_code,
    clippy::needless_doctest_main,
    clippy::needless_range_loop,
    clippy::while_let_on_iterator,
    clippy::vec_init_then_push
)]

/// # Scopes and Shadowing
/// 
/// You can shadow variables, both those from outer scopes and
///  variables from the same scope:
/// 
/// 
/// - Definition: Shadowing is different from mutation, because after
///   shadowing both variable’s memory locations exist at the same time.
///   Both are available under the same name, depending where you use it
///   in the code.
/// 
/// - A shadowing variable can have a different type.
/// 
/// - Shadowing looks obscure at first, but is convenient for holding on to
///   values after .unwrap().
/// 
/// - The following code demonstrates why the compiler can’t simply reuse memory
///   locations when shadowing an immutable variable in a scope, even if the type
///   does not change.
/// 
/// ```
/// fn main() {
///    let a = 1;
///    let b = &a;
///    let a = a + 1;
///    println!("{a} {b}");
/// }
/// ```

#[allow(dead_code)]
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
