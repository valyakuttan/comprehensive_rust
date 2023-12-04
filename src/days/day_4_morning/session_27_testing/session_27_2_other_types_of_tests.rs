/// # 27.2 Other Types of Tests
///
/// ## Integration Tests
///
/// If you want to test your library as a client, use an integration test.
///
/// Create a `libraray_module_name.rs` file under `tests/`:
///
/// ```
///
/// // tests/my_library.rs
/// use my_library::init;
///
/// #[test]
/// fn test_init() {
///     assert!(init().is_ok());
/// }
///
/// ```
///
/// These tests only have access to the public API of your crate.
///
/// ## Documentation Tests
///
/// Rust has built-in support for documentation tests:
///
/// - Code blocks in /// comments are automatically seen as Rust code.
/// 
/// - The code will be compiled and executed as part of `cargo test`.
/// 
/// - Adding # in the code will hide it from the docs, but will still
///   compile/run it
/// 

/// Shortens a string to the given length.
///
/// ```
/// # use playground::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}
