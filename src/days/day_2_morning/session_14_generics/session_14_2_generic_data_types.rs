/// # 14.2 Generic Data Types
///
/// You can use generics to abstract over the concrete field type:
///
/// Why `T` is specified twice in `impl<T> Point<T> {}`? Isn’t that
/// redundant?
///
/// - This is because it is a generic implementation section for generic
///   type. They are independently generic.
///
/// - It means these methods are defined for any `T`.
///
/// - It is possible to write `impl Point<u32> { .. }`. Noe the `Point`
///   is still generic and you can use `Point<f64>`, but methods
///   in this block will only be available for `Point<u32>`.
///

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    // fn set_x(&mut self, x: T)
}

pub fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
    println!("coords: {:?}", integer.coords());
}
