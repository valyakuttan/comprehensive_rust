#![allow(dead_code)]
/// # 10.1 Named Structs
///
/// Like C and C++, Rust has support for custom structs:
///
/// ```
///
/// struct Person {
///     name: String,
///     age: u8,
/// }
///
/// fn struct_example() {
///     let mut peter = Person {
///         name: String::from("Peter"),
///         age: 27,
///     };
///     println!("{} is {} years old", peter.name, peter.age);
///    
///     peter.age = 28;
///     println!("{} is {} years old", peter.name, peter.age);
///    
///     let jackie = Person {
///         name: String::from("Jackie"),
///         ..peter
///     };
///
///     println!("{} is {} years old", jackie.name, jackie.age);
/// }
///
/// ```
///
/// - Structs work like in C or C++.
///
///    - Like in C++, and unlike in C, no `typedef` is needed to define a type.
///
///    - Unlike in C++, there is no inheritance between structs.
///
/// - Methods are defined in an `impl` block.
///
/// - Zero-sized structs (e.g. `struct Foo;`) might be used when implementing
///   a trait on some type but don’t have any data that you want to store in
///   the value itself.
///
/// - The syntax `..peter` allows us to copy the majority of the fields from the
///   old struct without having to explicitly type it all out. It must always be
///   the last element.
///

#[allow(dead_code)]
pub fn main() {}
