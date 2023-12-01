/// # 9.1 Shared References
///
/// A reference provides a way to access another value without taking
/// responsibility for the value, and is also called “borrowing”. Shared
/// references are read-only, and the referenced data cannot change.
///
/// ```
///
/// fn shared_reference() {
///     let a = 'A';
///     let b = 'B';
///     let mut r: &char = &a;
///     println!("r: {}", *r);
///     r = &b;
///     println!("r: {}", *r);
/// }
///
/// ```
///
/// A shared reference to a type `T` has type `&T`. A reference value is
/// made with the `&` operator. The `*` operator “dereferences” a reference,
/// yielding its value.
///
/// Rust will statically forbid dangling references:
///
/// ```
/// fn x_axis(x: i32) -> &(i32, i32) {
///     let point = (x, 0);
///     return &point;
/// }
///
/// // error: return a borrowed value without borrowing any value.
///  
/// ```
///
/// - A reference is said to “borrow” the value it refers to.
///
/// - References are implemented as pointers, and a key advantage is that
///   they can be much smaller than the thing they point to.
///
/// - Rust does not automatically create references for you - the `&` is
///   always required.
///
/// - Rust will auto-dereference in some cases, in particular when invoking
///   methods (try `r.count_ones()`). There is no need for an `->` operator
///   like in C++.
///
/// - In this example, `r` is mutable so that it can be reassigned
///   (`r = &b`). Note that this re-binds `r`, so that it refers to
///   something else. This is different from C++, where assignment
///   to a reference changes the referenced value.
///
/// - A shared reference does not allow modifying the value it refers
///   to, even if that value was mutable. Try `*r = 'X`'.
///
/// - Rust is tracking the lifetimes of all references to ensure they
///   live long enough. Dangling references cannot occur in safe Rust.
///   `x_axis` would return a reference to point, but point will be
///   deallocated when the function returns, so this will not compile.
///
///

pub fn main() {}
