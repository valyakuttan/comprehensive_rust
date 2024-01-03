/// # 27.4 GoogleTest
///
/// The `GoogleTest` crate allows for flexible test assertions using
/// matchers:
///
/// ```rust
///
/// use googletest::prelude::*;
///
/// #[googletest::test]
/// fn test_elements_are() {
///     let value = vec!["foo", "bar", "baz"];
///     expect_that!(value, elements_are!(eq("foo"), lt("xyz"), starts_with("b")));
/// }
///
/// ```
///
/// If we change the last element to "!", the test fails with a structured
/// error message pin-pointing the error:
///
/// ```
///
/// ---- test_elements_are stdout ----
/// Value of: value
/// Expected: has elements:
///   0. is equal to "foo"
///   1. is less than "xyz"
///   2. starts with prefix "!"
/// Actual: ["foo", "bar", "baz"],
///   where element #2 is "baz", which does not start with "!"
///   at src/testing/googletest.rs:6:5
/// Error: See failure output above
///
///
/// ```
///
/// - Use `cargo add googletest` to quickly add it to an existing Cargo project.
///
/// - The `use googletest::prelude::*;` line imports a number of commonly used
///   macros and types.
///
/// - A particularly nice feature is that mismatches in multi-line strings
///   strings are shown as a diff:
///
/// ```
///
/// #[test]
/// fn test_multiline_string_diff() {
///     let haiku = "Memory safety found,\n\
///                  Rust's strong typing guides the way,\n\
///                  Secure code you'll write.";
///     assert_that!(
///         haiku,
///         eq("Memory safety found,\n\
///             Rust's silly humor guides the way,\n\
///             Secure code you'll write.")
///     );
/// }
///
/// ```
///
/// shows a color-coded diff:
///
/// ```
///
/// Value of: haiku
/// Expected: is equal to "Memory safety found,\nRust's silly humor guides the way,\nSecure code you'll write."
/// Actual: "Memory safety found,\nRust's strong typing guides the way,\nSecure code you'll write.",
///   which isn't equal to "Memory safety found,\nRust's silly humor guides the way,\nSecure code you'll write."
/// Difference(-actual / +expected):
///  Memory safety found,
/// -Rust's strong typing guides the way,
/// +Rust's silly humor guides the way,
///  Secure code you'll write.
///   at src/testing/googletest.rs:17:5
///
/// ```
///

pub fn main() {}
