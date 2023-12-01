#![allow(dead_code)]

/// # 13.4 Trait Objects
///
/// Trait objects allow for values of different types, for instance in
/// a collection:
///
///
/// - Types that implement a given trait may be of different sizes. This
///   makes it impossible to have things like `Vec<dyn Pet>` in the
///   example above.
///
/// - `dyn Pet` is a way to tell the compiler about a dynamically sized
///   type that implements `Pet`.
///
/// - In the example, `pets` is allocated on the stack and the vector data
///   is on the heap. The two vector elements are fat pointers:
///
/// - A **fat pointer** is a double-width pointer. It has two components:
///   - A pointer to the actual object and
///   - A pointer to the virtual method table (**vtable**) for the Pet
///     implementation of that particular object.
///
/// - The data for the `Dog` named Fido is the name and age fields. The
///   `Cat` has a lives field.
///
///

struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
} // No name needed, cats won't respond anyway.

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

pub fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog {
            name: String::from("Fido"),
            age: 5,
        }),
    ];

    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }

    println!(
        "Size of Dog: {}, Cat: {}",
        std::mem::size_of::<Dog>(),
        std::mem::size_of::<Cat>()
    );
    println!(
        "Size of &Dog: {}, &Cat: {}",
        std::mem::size_of::<&Dog>(),
        std::mem::size_of::<&Cat>()
    );
    println!("Size of &dyn Pet: {}", std::mem::size_of::<&dyn Pet>());
    println!(
        "Size of Box<&dyn Pet>: {}",
        std::mem::size_of::<Box<dyn Pet>>()
    );
}
