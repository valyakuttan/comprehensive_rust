/// # Enum Sizes
/// 
/// Rust enums are packed tightly, taking constraints due to
/// alignment into account:
/// 
/// - Internally Rust is using a field (discriminant) to keep
///   track of the enum variant.
/// 
/// - You can control the discriminant if needed
///   (e.g., for compatibility with C):
/// ```
/// #[repr(u32)]
/// enum Bar {
///    A,  // 0
///    B = 10000,
///    C,  // 10001
/// }
///
/// fn main() {
///    println!("A: {}", Bar::A as u32);
///    println!("B: {}", Bar::B as u32);
///    println!("C: {}", Bar::C as u32);
/// }
/// ```
/// 
/// Without repr, the discriminant type takes 2 bytes, because
/// 10001 fits 2 bytes.
/// 
/// 

use std::any::type_name;
use std::mem::{align_of, size_of};

fn dbg_size<T>() {
    println!("{}: size {} bytes, align: {} bytes",
        type_name::<T>(), size_of::<T>(), align_of::<T>());
}

#[allow(dead_code)]
enum Foo {
    A,
    B,
}

#[allow(dead_code)]
pub fn main() {
    dbg_size::<Foo>();
}