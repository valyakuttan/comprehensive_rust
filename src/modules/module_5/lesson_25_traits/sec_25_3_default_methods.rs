/// # Default Methods
///
/// Traits can implement behavior in terms of other trait methods:
///
/// - Traits may specify pre-implemented (default) methods and methods
///   that users are required to implement themselves. Methods with default
///   implementations can rely on required methods.

trait NotEquals {
    fn not_equals(&self, other: &Self) -> bool;
}

impl<T> NotEquals for T where T: Equals {
    fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

trait Equals {
    fn equals(&self, other: &Self) -> bool;
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equals(&self, other: &Centimeter) -> bool {
        self.0 == other.0
    }
}

pub fn main() {
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equals(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equals(&b));
}
