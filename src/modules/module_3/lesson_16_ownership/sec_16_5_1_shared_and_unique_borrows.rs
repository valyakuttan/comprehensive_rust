/// # Shared and Unique Borrows
///
/// Rust puts constraints on the ways you can borrow values:
///
/// - Either you can have one or more `&T` values at any given time, or
///
/// - Or you can have exactly one `&mut T` value.
///
/// ```
/// fn main() {
///     let mut a: i32 = 10;
///     let b: &i32 = &a;
///
///     {
///         let c: &mut i32 = &mut a;
///         *c = 20;
///     }
///
///     println!("a: {a}");
///     println!("b: {b}");
/// }
/// ```
///
/// - The above code does not compile because `a` is borrowed as mutable
///   (through `c`) and as immutable (through `b`) at the same time.
///
/// - If we move the `println!` statement for `b` before the scope
///   that introduces `c` the compiler realizes that `b` is only
///   ever used before the new mutable borrow of `a`` through `c`.
///   This is a feature of the borrow checker called “non-lexical lifetimes”.
///   And the code will compile

#[allow(dead_code)]
pub fn main() {}
