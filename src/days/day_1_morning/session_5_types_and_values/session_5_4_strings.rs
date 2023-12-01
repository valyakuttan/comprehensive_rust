#![allow(dead_code)]

/// # 5.4 Strings
///
/// Rust has two types to represent strings. Both always store **UTF-8**
/// encoded strings.
///
/// - `String` - a modifiable, owned string.
///
/// - `&str` - a read-only string. String literals have this type.
///
/// - `String` is a user-defined type with a constructor (`::new()`) and
///   methods like `s.push_str(..)`.
///
/// - The `&` in `&str` indicates that this is a reference. `&str` means
///   “a read-only string”.
///
/// - Raw strings allow you to create a `&str` value with
///   escapes disabled: `r"\n" == "\\n"`. You can embed double-quotes
///   by using an equal amount of # on either side of the quotes:
///
///   ```
///
///   fn raw_string() {
///       println!(r#"<a href="link.html">link</a>"#);
///       println!("<a href=\"link.html\">link</a>");
///   }
///
///   ```
///
/// - Byte strings allow you to create a `&[u8]` value directly:
///
///   ```
///
///   fn byte_string() {
///       println!("{:?}", b"abc");
///       println!("{:?}", &[97, 98, 99]);
///   }
///
///   ```
///

pub fn main() {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
}
