/// # String
///
/// `String` is the standard heap-allocated growable **UTF-8**
/// string buffer:
///
/// `String` implements `Deref<Target = str>`, which means that
/// you can call all `str` methods on a `String`.
///
/// - `String::new` returns a new empty string, use `String::with_capacity`
///   when you know how much data you want to push to the string.
///
/// - `String::len` returns the size of the `String` in bytes (which can be
///   different from its length in characters).
///
/// - `String::chars` returns an iterator over the actual characters. Note that
///   a char can be different from what a human will consider a “character” due
///   to grapheme clusters.
///
/// - When people refer to strings they could either be talking about `&str` or
///   `String`.
///
/// - When a type implements `Deref<Target = T>`, the compiler will let you
///   transparently call methods from `T`.
///
/// - `String` implements `Deref<Target = str>` which transparently gives it
///   access to `str`’s methods.
///
/// ```
///
/// let mut s = String::from("Hello");
///
/// // let t: str = *s; // won't compile because size of `t`(which is string),
///                     // is unknown.
///
/// let p: &str = &*s; // compile because `p` is pointer
///
/// let p: &str = s.deref();
///
///
/// assert_eq!(p, q);
///
/// ```
///
/// - `String` is implemented as a wrapper around a vector of bytes, many of the
///   operations you see supported on vectors are also supported on `String`,
///   but with some extra guarantees.
///
/// ## Indexing of String:
///
/// Indexing into a string is not available in Rust. The reason for this is that
/// Rust strings are encoded in UTF-8 internally, so the concept of indexing itself
/// would be ambiguous, and people would misuse it: byte indexing is fast, but
/// almost always incorrect (when your text contains non-ASCII symbols, byte indexing
/// may leave you inside a character, which is really bad if you need text processing),
/// while char indexing is not free because UTF-8 is a variable-length encoding, so
/// you have to traverse the entire string to find the required code point.
///
/// - If you are certain that your strings contain ASCII characters only, you can use
///   the `as_bytes()` method on `&str` which returns a byte slice, and then index into
///   this slice:
///
///   ```
///
///     let b: u8 = string.as_bytes()[i];
///     let c: char = b as char;  // if you need to get the character as a
///                               // unicode code point
///
///   ```
///
/// - If you do need to index code points, you have to use the `char()` iterator:
///
///   ```
///
///   string.chars().nth(i).unwrap()
///
///   ```
///
/// - Finally, in many cases of text processing, it is actually necessary to
///   work with **grapheme clusters** rather than with code points or bytes.
///   With the help of the unicode-segmentation crate, you can index into
///   grapheme clusters as well:
///
///   ```
///
///   use unicode_segmentation::UnicodeSegmentation
///
///   let string: String = ...;
///   UnicodeSegmentation::graphemes(&string, true).nth(i).unwrap()
///
///   ```

#[allow(dead_code, unused_variables)]
pub fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let first = first_word(&my_string[..6]);
    println!("first word: {first}");

    let p = first.len() + 1;
    let second = first_word(&my_string[p..]);
    println!("second word: {second}");
}

fn first_word(s: &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
