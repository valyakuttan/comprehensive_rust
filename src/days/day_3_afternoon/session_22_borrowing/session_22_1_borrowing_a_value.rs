/// # 22.1 Borrowing
///
/// Instead of transferring ownership when calling a function,
/// you can let a function borrow the value:
///
/// ```
/// #[derive(Debug)]
/// struct Point(i32, i32);
///
/// fn add(p1: &Point, p2: &Point) -> Point {
///     Point(p1.0 + p2.0, p1.1 + p2.1)
/// }
///
/// fn main() {
///     let p1 = Point(3, 4);
///     let p2 = Point(10, 20);
///     let p3 = add(&p1, &p2);
///     println!("{p1:?} + {p2:?} = {p3:?}");
/// }
/// ```
///
/// - The add function borrows two points and returns a new point.
///
/// - The caller retains ownership of the inputs.
///
/// - printing stack addresses demonstrate that the return from `add`
///   is cheap because the compiler can eliminate the copy operation.
///   Note that both addresses are same.
///
/// - The Rust compiler can do return value optimization (RVO).
///
/// - If RVO did not happen, Rust will always perform a simple
///   and efficient `memcpy` copy.
///

#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
    let p = Point(p1.0 + p2.0, p1.1 + p2.1);
    println!("&p.0: {:p}", &p.0); // &p.0: 0x7ffdf0795248
    p
}

pub fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);

    println!("{p1:?} + {p2:?} = {p3:?}");

    println!("&p3.0: {:p}", &p3.0); // &p3.0: 0x7ffdf0795248
    println!("{p1:?} + {p2:?} = {p3:?}");
}
