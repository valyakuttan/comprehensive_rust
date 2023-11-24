/// # Methods
///
/// Rust allows you to associate functions with your
/// new types. You do this with an `impl` block:
///
/// - Methods are called on an instance of a type
///   (such as a struct or enum), the first parameter
///   represents the instance as `self`.
///
/// - Developers may choose to use methods to take advantage of
///   method receiver syntax and to help keep them more organized.
///   By using methods we can keep all the implementation code in
///   one predictable place.
///
/// - `self` when used as first method argument, is a
///   shorthand for `self: Self`. There are also
///   `&self`, which is equivalent to `self: &Self`,
///   and `&mut self`, which is equivalent to `self: &mut Self`.
///
/// - `Self` in method arguments is syntactic sugar for the receiving
///   type of the method (i.e. the type whose `impl` this method is in).
///   This also allows for generic types without too much repetition.
///
/// - `self` can be used like other structs and dot notation can be
///   used to refer to individual fields.
///
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {
        assert_eq!(self.name.as_str(), "Peter");
        assert_eq!(self.age, 27);
    }
}

#[allow(dead_code)]
pub fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    peter.say_hello();
    //peter.say_hello();
}
