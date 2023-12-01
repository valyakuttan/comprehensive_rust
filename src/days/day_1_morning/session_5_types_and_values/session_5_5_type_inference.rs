#![allow(dead_code)]

/// # 5.5 Type Inference
///
/// Rust will look at how the variable is used to determine the type:
///
/// When nothing constrains the type of an integer literal, Rust defaults
/// to `i32`. This sometimes appears as `{integer}` in error messages.
/// Similarly, floating-point literals default to `f64`.
///

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

pub fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    //takes_u32(y); // error y is of type i8;

    // let x = 3.14;
    // let y = 20;
    //assert_eq!(x, y);
    // error: no implementation for `{float} == {integer}`
}
