#![allow(dead_code)]

/// # 5.3 Arithmetic
///
/// Arithmetic is very similar to other languages, with similar precedence.
///
/// - In C and C++ overflow of signed integers is actually undefined, and
///   might do different things on different platforms or compilers. In Rust,
///   it’s defined.
///
/// - An overflow panics (checked) in a debug build and wraps in a release
///   build. There are other options, such as overflowing, saturating, and
///   carrying. These are accessed with method syntax,
///   e.g., `(a * b).saturating_add(b * c).saturating_add(c * a)`.
///
/// - The compiler will detect overflow of constant expressions, which is
///   why the example requires a separate function.
///
///   ```
///
///     fn overflow(a: u8, b: u8) -> u8 {
///         a * b
///     }
///
///     let _x: u8 = overflow(100, 100); // compiler won't detect overflow
///
///      // let _y: u8 = 100 * 100; compiler will detect overflow
///
///   ```
///

pub fn main() {
    let x: u8 = 250;
    let a: u8 = x.saturating_add(100); // a becomes 255 = 2^8 - 1
    assert_eq!(a, 255);
}
