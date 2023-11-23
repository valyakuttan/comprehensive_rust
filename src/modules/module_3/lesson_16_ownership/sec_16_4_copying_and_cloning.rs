/// # Copying and Cloning
///
/// While move semantics are the default, certain types
/// are copied by default:
///
/// ```
/// fn main() {
///     let x = 42;
///     let y = x;
///     println!("x: {x}");
///     println!("y: {y}");
/// }
/// ```

/// These types implement the `Copy` trait.
///
/// You can opt-in your own types to use copy semantics:
///
/// ```
/// #[derive(Copy, Clone, Debug)]
/// struct Point(i32, i32);
///
/// fn main() {
///     let p1 = Point(3, 4);
///     let p2 = p1;
///     println!("p1: {p1:?}");
///     println!("p2: {p2:?}");
/// }
/// ```
///
/// - After the assignment, both `p1` and `p2` own their own data.
///
/// - We can also use `p1.clone()` to explicitly copy the data.

/// Copying and cloning are not the same thing:
///
/// - Copying refers to bitwise copies of memory regions and
///   does not work on arbitrary objects.
///
/// - Copying does not allow for custom logic
///   (unlike copy constructors in C++).
///
/// - Cloning is a more general operation and also allows
///   for custom behavior by implementing the `Clone` trait.
///
/// - Copying does not work on types that implement the `Drop` trait.
///

#[derive(Clone, Debug)] // can't derive Copy trait because of String
struct Foo(i32, String);

#[allow(dead_code)]
pub fn main() {
    let f1 = Foo(1, "Rust".to_string());
    // let f2 = f1; // can't copy
    let f2 = f1.clone();

    assert_eq!(f2.0, f1.0);
}
