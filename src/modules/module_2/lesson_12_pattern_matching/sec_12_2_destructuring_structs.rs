/// # Destructuring Structs
/// 
/// You can also destructure structs:
/// 

struct Foo {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
#[allow(dead_code)]
fn main() {
    let _foo = Foo { x: (1, 2), y: 3 };
    match _foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}