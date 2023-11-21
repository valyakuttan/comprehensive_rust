/// # Variables
/// 
/// Rust provides type safety via static typing.
/// Variable bindings are immutable by default:
/// 

#[allow(dead_code)]
pub fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20;
    // println!("x: {x}");
}