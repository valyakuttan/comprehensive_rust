/// # Methods
///
/// Methods are functions associated with a type. The `self` argument of
/// a method is an instance of the type it is associated with:
///
/// - While technically, Rust does not have custom constructors, static
///   methods are commonly used to initialize structs (but don’t have to).
///   The actual constructor, `Rectangle { width, height }`, could be called directly.
///

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        // static method no `self` parameter
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }

    fn square(width: u32) -> u32 {
        // a static method with a parameter
        width * width
    }
}

#[allow(dead_code)]
pub fn main() {
    let mut rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());

    let sq = Rectangle::new(5, 5);
    println!("area of square: {}", sq.area());

    println!("area of square of width 5 is: {}", Rectangle::square(5));
}
