/// # 29.1 Panics
///
/// Rust handles fatal errors with a “panic”.
///
/// Rust will trigger a panic if a fatal error happens at runtime:
///
/// ```
///
/// fn panic_example() {
///     let v = vec![10, 20, 30];
///     println!("v[100]: {}", v[100]);
/// }
///
/// ```
///
/// - Panics are for unrecoverable and unexpected errors.
///
/// - Panics are symptoms of bugs in the program.
///
/// - Runtime failures like failed bounds checks can panic
///
/// - Assertions (such as `assert!`) panic on failure
///
/// - Purpose-specific panics can use the `panic!` macro.
///
/// - A panic will “unwind” the stack, dropping values just as if the
///   functions had returned.
///
/// - Use non-panicking APIs (such as `Vec::get`) if crashing is not acceptable.
///
/// By default, a panic will cause the stack to unwind. The unwinding can be caught:
///
/// ```
/// use std::panic;
///
/// fn catch_panic_example() {
///     let result = panic::catch_unwind(|| {
///         "No problem here!"
///     });
///     println!("{result:?}");
///
///     let result = panic::catch_unwind(|| {
///         panic!("oh no!");
///     });
///     println!("{result:?}");
/// }
///
/// ```
///
/// - Catching is unusual; do not attempt to implement exceptions with `catch_unwind`!
///
/// - This can be useful in servers which should keep running even if a single
///  request crashes.
///
/// - This does not work if `panic = 'abort'` is set in your `Cargo.toml`.
///
use std::panic;

pub fn main() {
    let result = panic::catch_unwind(|| "No problem here!");
    println!("{result:?}");

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    println!("{result:?}");
}
