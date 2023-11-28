/// # `if let` expressions
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

#[allow(dead_code)]
pub fn main() {
    // if let
    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }

    // let else
    println!("{:?}", second_word_to_upper("foo bar"));
}

fn second_word_to_upper(s: &str) -> Option<String> {
    let mut it = s.split(' ');
    let (Some(_), Some(item)) = (it.next(), it.next()) else {
        return None;
    };

    Some(item.to_uppercase())
}
