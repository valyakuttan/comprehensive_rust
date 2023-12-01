/// # 12.2 Let Control Flow
///
/// Rust has a few control flow constructs which differ from other
/// languages. They are used for pattern matching:
///
/// - `if let` expressions
///
/// -` while let` expressions
///
/// - `match` expressions
///
/// ## `if let` expressions
///
/// The `if let` expression lets you execute different code
/// depending on whether a value matches a pattern:
///
/// - Unlike match, `if let` does not have to cover all branches.
///   This can make it more concise than match.
///
/// - A common usage is handling `Some` values when working with `Option`.
///
/// - Unlike match, `if let` does not support guard clauses for
///   pattern matching.
///
/// - A similar `let-else` construct allows to do a destructuring
///   assignment, or if it fails, execute a block which is required
///   to abort normal control flow
///
/// ```
///
/// fn sleep_for(secs: f32) {
///     let dur = if let Ok(dur) = std::time::Duration::try_from_secs_f32(secs) {
///         dur
///     } else {
///         std::time::Duration::from_millis(500)
///     };
///     std::thread::sleep(dur);
///     println!("slept for {:?}", dur);
/// }
///
/// fn if_let_example() {
///     sleep_for(-10.0);
///     sleep_for(0.8);
/// }
///
/// ```
///
/// For the common case of matching a pattern and returning from the
/// function, use `let else`. The “else” case must diverge (`return`,
/// `break`, or `panic` - anything but falling off the end of the block).
///  
/// ```
///
/// fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
///     let s = if let Some(s) = maybe_string {
///         s
///     } else {
///         return Err(String::from("got None"));
///     };
///
///     let first_byte_char = if let Some(first_byte_char) = s.chars().next() {
///         first_byte_char
///     } else {
///         return Err(String::from("got empty string"));
///     };
///
///     if let Some(digit) = first_byte_char.to_digit(16) {
///         Ok(digit)
///     } else {
///         Err(String::from("not a hex digit"))
///     }
/// }
///
/// fn let_else_example() {
///     println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
/// }
///
/// ```
///
/// if-lets can pile up, as shown. The let-else construct supports flattening
/// this nested code.
///
/// The rewritten version is:
///
/// ```
///
/// fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
///     let Some(s) = maybe_string else {
///         return Err(String::from("got None"));
///     };
///
///     let Some(first_byte_char) = s.chars().next() else {
///         return Err(String::from("got empty string"));
///     };
///
///     let Some(digit) = first_byte_char.to_digit(16) else {
///         return Err(String::from("not a hex digit"));
///     };
///
///     Ok(digit)
/// }
///
/// ```
///
/// ## `while let` expression
///
/// Like with `if let`, there is a `while let` variant which
/// repeatedly tests a value against a pattern:
///
/// Here `String::pop` returns `Some(c)` until the string is empty,
/// after which it will return `None`. The while let lets us keep
/// iterating through all items.
///

pub fn main() {
    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        println!("character: {c}");
    }
    // (There are more efficient ways to reverse a string!)

    let word: &str = "loẅks";
    let drow: String = word
        // Split the string into an Iterator of &strs, where each element is an
        // extended grapheme cluster.
        .chars()
        // Reverse the order of the grapheme iterator.
        .rev()
        // Collect all the chars into a new owned String.
        .collect();

    assert_eq!(drow, "skẅol");
}
