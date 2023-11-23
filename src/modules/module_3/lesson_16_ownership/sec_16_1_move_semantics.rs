/// # Move Semantics
///
/// An assignment will transfer ownership between variables:
///
/// - The assignment of s1 to s2 transfers ownership.
/// - The heap data from s1 is reused for s2.
/// - When s1 goes out of scope, nothing happens: it does not own anything.
/// - When s2 goes out of scope, the string data is freed.
/// - There is always exactly one variable binding which owns a value.
///
/// - It is only the ownership that moves. Whether any machine code is
///   generated to manipulate the data itself is a matter of optimization,
///   and such copies are aggressively optimized away.
///
/// - Simple values (such as integers) can be marked `Copy`.
///
/// - In Rust, clones are explicit (by using `clone`).
///

#[allow(dead_code)]
pub fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}
