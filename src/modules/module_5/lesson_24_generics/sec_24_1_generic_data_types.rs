#![allow(dead_code)]

/// # Generics
///
/// Rust support generics, which lets you abstract algorithms or data
/// structures (such as sorting or a binary tree) over the types used
/// or stored.
///
/// ## Generic Data Types
///
/// You can use generics to abstract over the concrete field type:
///

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p = Point { x: 1, y: 4.0 };

    println!("{integer:?} and {float:?} and {p:?}");
}
