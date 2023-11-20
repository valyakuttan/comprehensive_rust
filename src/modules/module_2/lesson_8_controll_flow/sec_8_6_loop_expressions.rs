/// # loop expressions
///
/// Finally, there is a `loop` keyword which creates an endless loop.
/// Here you must either `break` or `return` to stop the loop:
///
/// `loop` is the only looping construct which returns a non-trivial value.
/// This is because it’s guaranteed to be entered at least once (unlike
/// `while` and `for` loops).
///

#[allow(dead_code)]
pub fn main() {
    let mut x = 10;

    let y = loop {
        if x < 2 {
            break x;
        }
        x /= 2;
    };

    println!("y: {y}");
}
