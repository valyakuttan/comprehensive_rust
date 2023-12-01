/// # 12.1 Destructuring Structs
///
/// Like tuples, structs and enums can also be destructured by matching:
///
/// ## Structs
///
/// ```
///
/// struct Foo {
///     x: (u32, u32),
///     y: u32,
/// }
///
/// fn struct_example() {
///     let _foo = Foo { x: (1, 2), y: 3 };
///     match _foo {
///         Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
///         Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
///         Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
///     }
/// }
///
/// ```
///
/// ## Enums
///
/// Patterns can also be used to bind variables to parts of your values. This
/// is how you inspect the structure of your types.
///

enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

fn main() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}
