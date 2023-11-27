
/// # Generic Methods
/// 
/// You can declare a generic type on your `impl` block:
///

#[derive(Debug)]
struct Point<T, U>(T, U);

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.0
    }

    fn y(&self) -> &U {
        &self.1
    }

    fn set_x(&mut self, x: T) {
        self.0 = x;
    }

    fn set_y(&mut self, y: U) {
        self.1 = y;
    }
}

pub fn main() {
    let mut p = Point(5, 10.5);
    println!("p.x = {}, p.y = {}", p.x(), p.y());

    p.set_y(5.3);
    println!("p.x = {}, p.y = {}", p.x(), p.y());
}
