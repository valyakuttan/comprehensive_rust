#![allow(dead_code)]

/// # 13.2 Traits
///
/// Rust lets you abstract over types with traits. They’re similar to
/// interfaces:
///
/// - A trait defines a number of methods that types must have in order
///   to implement the trait.
///
/// - Traits are implemented in an `impl <trait> for <type> { .. }` block.
///
/// - Traits may specify pre-implemented (provided) methods and methods
///   that users are required to implement themselves. Provided methods
///   can rely on required methods. In this case, greet is provided, and
///   relies on talk.
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

fn greet<P: Pet>(pet: &P) {
    println!("Oh you're a cutie! What's your name? {}", pet.talk());
}

pub fn main() {
    let captain_floof = Cat { lives: 9 };
    let fido = Dog {
        name: String::from("Fido"),
        age: 5,
    };

    greet(&captain_floof);
    greet(&fido);
}
