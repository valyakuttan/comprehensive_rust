/// # 19.4 Move Semantics
///
/// ## Moves in Assignment Statements
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
/// ## Moves in Function Calls
///
/// When you pass a value to a function, the value is
/// assigned to the function parameter. This transfers ownership:
///
/// - With the first call to `say_hello`, `main` gives up ownership of name.
///   Afterwards, `name` cannot be used anymore within `main`.
///
/// - The heap memory allocated for `name` will be freed at the end of the
///   `say_hello` function.
///
/// - `main` can retain ownership if it passes name as a reference
///   (`&name`) and if `say_hello` accepts a reference as a parameter.
///
/// - Alternatively, `main` can pass a clone of name in the first call
///   (`name.clone()`).
///
/// - **Rust** makes it harder than **C++** to inadvertently create copies by
///   making move semantics the default, and by forcing programmers to make
///   clones explicit.
///

pub fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;

    //println!("s1: {s1}"); // error: borrow of moved value s1
    println!("s2: {s2}");

    say_hello(s2);

    // println!("s2: {s2}"); // error: borrow of moved value s2
}

fn say_hello(name: String) {
    println!("Hello {name}")
}
