/// # Implicit Conversions
///
/// Rust will not automatically apply implicit conversions between types
/// (unlike C++). You can see this in a program like the one given below.
/// 
/// The Rust integer types all implement the `From<T>` and `Into<T>` traits
/// to let us convert between them. The `From<T>` trait has a single `from()`
/// method and similarly, the `Into<T>` trait has a single `into()` method.
/// Implementing these traits is how a type expresses that it can be
/// converted into another type.
/// 
/// The standard library has an implementation of `From<i8>` for `i16`, which
/// means that we can convert a variable `x` of type `i8` to an `i16` by
/// calling `i16::from(x)`. Or, simpler, with `x.into()`, because
/// `From<i8>` for i16 implementation automatically create an
/// implementation of `Into<i16>` for `i8`.
/// 
/// The same applies for your own From implementations for your own types,
/// so it is sufficient to only implement `From` to get a respective
/// `Into` implementation automatically.



fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

#[allow(dead_code)]
pub fn main() {
    let x: i8 = 15;
    let y: u8 = 100;

    println!("{x} * {y} = {}", multiply(x.into(), i16::from(y)));
}