/// # References
///
/// Like C++, Rust has references:
///
/// Some notes:
///
/// - We must dereference `ref_x` when assigning to it, similar
///   to C and C++ pointers.
///
/// - Rust will auto-dereference in some cases, in particular when
///   invoking methods.
///
/// - References that are declared as `mut` can be bound to different
///   values over their lifetime.
///
/// - Note the difference between `let mut ref_x: &i32` and
///   `let ref_x: &mut i32`. The first one represents a mutable reference
///   which can be bound to different values, while the second represents
///   a reference to a mutable value.
///
/// ## Dangling References:
///
/// Rust will statically forbid dangling references:
///
/// ```
/// fn main() {
///   let ref_x: &i32;
///   {
///      let x: i32 = 10;
///      ref_x = &x;
///   }
///   println!("ref_x: {ref_x}"); // error
/// }
/// ```
///
/// - A reference is said to “borrow” the value it refers to.
///
/// - Rust is tracking the lifetimes of all references to ensure they
///   live long enough.
///

#[allow(dead_code, unused_variables, unused_assignments)]
pub fn main() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x; // reference is non-mutable
    *ref_x = 20;

    // println!("x: {x}");

    let mut y: i32 = 100;

    let mut mut_ref = &x; //  reference is mutable
    mut_ref = &y;

    let mut mut_ref_mut: &mut i32 = &mut x; // reference is mutable and points to a mutable object

    *mut_ref_mut = 70;

    mut_ref_mut = &mut y;
    *mut_ref_mut = 50;

    println!("y: {}", *mut_ref_mut);
}
