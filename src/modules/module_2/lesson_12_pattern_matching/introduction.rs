/// # Pattern Matching
///
/// The `match` keyword lets you match a value against one or more patterns.
/// The comparisons are done from top to bottom and the first match wins.
///
/// Inside a pattern these characters assume special meaning:
///
/// - `|` as an or
///
/// - `..` represents a range, which can expand as much as it needs to be
///
/// - `1..=5` represents an inclusive range
///
/// - `_` is a wild card
///
/// 
/// ## Refutability: Whether a Pattern Might Fail to Match
/// 
/// Patterns come in two forms: **refutable** and **irrefutable**. Patterns that will
/// match for any possible value passed are irrefutable. An example would be `x` in
/// the statement `let x = 5;` because `x` matches anything and therefore cannot fail
/// to match. Patterns that can fail to match for some possible value are refutable. An
/// example would be `Some(x)` in the expression if `let Some(x) = a_value` because if
/// the value in the a_value variable is `None` rather than `Some`, the `Some(x)` pattern
/// will not match.
/// 
/// Function parameters, `let` statements, and `for`` loops can only accept irrefutable
/// patterns, because the program cannot do anything meaningful when values don’t match. The
/// `if let` and `while let` expressions accept refutable and irrefutable patterns, but the
/// compiler warns against irrefutable patterns because by definition they’re intended to handle
/// possible failure: the functionality of a conditional is in its ability to perform differently
/// depending on success or failure.

#[allow(dead_code)]
pub fn main() {
    let input = 'x';

    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        _ => println!("Something else"),
    }
}
