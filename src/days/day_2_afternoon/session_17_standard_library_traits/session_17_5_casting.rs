/// # 17.5 Casting
///
/// Rust has no implicit type conversions, but does support explicit casts
/// with `as`. These generally follow C semantics where those are defined.
///
/// The results of `as` are always defined in Rust and consistent across
/// platforms. This might not match your intuition for changing sign or
/// casting to a smaller type – check the docs, and comment for clarity.
///
/// `as` is similar to a C++ static cast. Use of `as` in cases where data
/// might be lost is generally discouraged, or at least deserves an
/// explanatory comment.
///
/// This is common in casting integers to `usize` for use as an index.
///

pub fn main() {
    let value: i64 = 1000;
    println!("as u16: {}", value as u16);
    println!("as i16: {}", value as i16);
    println!("as u8: {}", value as u8);
}
