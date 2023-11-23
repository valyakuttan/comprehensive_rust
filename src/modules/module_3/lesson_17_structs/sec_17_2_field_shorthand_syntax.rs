/// # Field Shorthand Syntax
///
/// If you already have variables with the right names,
/// then you can create the struct using a shorthand:
///
/// ```
/// #[derive(Debug)]
/// struct Person {
///     name: String,
///     age: u8,
/// }
///
/// impl Person {
///     fn new(name: String, age: u8) -> Person {
///         Person { name, age }
///     }
/// }
///
/// fn main() {
///     let peter = Person::new(String::from("Peter"), 27);
///     println!("{peter:?}");
/// }
/// ```
///
/// - The `new` function could be written using `Self` as a type, as
///   it is interchangeable with the struct type name.
///
/// ```
/// impl Person {
///     fn new(name: String, age: u8) -> Self {
///         Self { name, age }
///     }
/// }
/// ```
///

/// - By implementing the `Default` `trait` for the `struct`. We can set
///   default values for the fields. Then we can set values for some fields
///   and use the default values for the other fields.
///
/// ```
/// #[derive(Debug)]
/// struct Person {
///     name: String,
///     age: u8,
/// }
///
/// impl Default for Person {
///     fn default() -> Person {
///         Person {
///             name: "Bot".to_string(),
///             age: 0,
///         }
///     }
/// }
///
/// fn create_default() {
///     let tmp = Person {
///         ..Person::default()
///     };
///     let tmp = Person {
///         name: "Sam".to_string(),
///         ..Person::default()
///     };
/// }
/// ```
///
/// - Methods are defined in the impl block.
///
/// - We can use struct update syntax to define a new structure using
///   old values.
///
/// - Use {:#?} when printing structs to request the Debug representation.
///

#[allow(dead_code)]
pub fn main() {
    let peter = Person {
        name: String::from("Peter"),
        ..Person::default()
    };

    println!("{peter:#?}");
}

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
impl Default for Person {
    fn default() -> Person {
        Person {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}
