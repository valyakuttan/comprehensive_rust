#![allow(dead_code, clippy::needless_doctest_main)]

/// # Compound Types
///
/// | Name   | Types                        | Literals                         |
/// |        |                              |                                  |
/// | Arrays | [T; N]                       | [20, 30, 40], [0; 3]             |
/// |        |                              |                                  |
/// | Tuples | (), (T,), (T1, T2), $\lodts$ | (), ('x',), ('x', 1.2), $\ldots$ |
///
/// ## Arrays:
///
/// - A value of the array type `[T; N]` holds N (a compile-time constant) elements
///   of the same type `T`. Note that the length of the array is part of its type,
///   which means that `[u8; 3]` and `[u8; 4]` are considered two different types.
///
/// - We can use literals to assign values to arrays.
///
/// - The `println!` macro asks for the debug implementation with the `?` format
///   parameter: `{}` gives the default output, `{:?}` gives the debug output.
///   Types such as integers and strings implement the default output, but arrays
///   only implement the debug output. This means that we must use debug output here.
///
/// - Adding #, eg {a:#?}, invokes a “pretty printing” format, which can be easier to read.
///
/// ```
/// fn main() {
///   let mut a: [i8; 10] = [42; 10];
///   a[5] = 0;
///   println!("a: {a:?}");
/// }
/// ```
///
///
/// ## Tuples:
///
/// - Like arrays, tuples have a fixed length.
///
/// - Tuples group together values of different types into a compound type.
///
/// - Fields of a tuple can be accessed by the period and the index of the value,
///   e.g. `t.0`, `t.1`.
///
/// - The empty tuple `()` is also known as the “unit type”. It is both a type, and
///   the only valid value of that type - that is to say both the type and its value
///   are expressed as `()`. It is used to indicate, for example, that a function or
///   expression has no return value.
///
/// ```
/// fn main() {
///   let t: (i8, bool) = (7, true);
///   println!("t.0: {}", t.0);
///   println!("t.1: {}", t.1);
/// }
/// ```
pub fn main() {}
