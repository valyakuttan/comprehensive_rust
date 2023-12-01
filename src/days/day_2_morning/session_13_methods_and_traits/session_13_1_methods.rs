#![allow(dead_code)]

/// # 13.1 Methods
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
/// ## Method Receiver
///
/// The `&self` indicates that the method borrows the object immutably.
/// There are other possible receivers for a method:
///
/// - `&self`: borrows the object from the caller using a shared
///   and immutable reference. The object can be used again afterwards.
///
/// - `&mut self`: borrows the object from the caller using a unique and
///   mutable reference. The object can be used again afterwards.
///
/// - `self`: takes ownership of the object and moves it away from the
///   caller. The method becomes the owner of the object. The object
///   will be dropped (deallocated) when the method returns, unless
///   its ownership is explicitly transmitted. Complete ownership does
///   not automatically mean mutability.
///
/// - `mut self`: same as above, but the method can mutate the object.
///
/// - No receiver: this becomes a static method on the struct. Typically
///   used to create constructors which are called `new` by convention.
///
/// Beyond variants on `self`, there are also special wrapper types allowed
/// to be receiver types, such as `Box<Self>`.
///
/// Although the method receivers are different, the non-static functions
/// are called the same way in the `main` body. Rust enables automatic
/// referencing and dereferencing when calling methods. Rust automatically
/// adds in the `&`, `*`, `mut` so that that object matches the method
/// signature.
///

#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race {
        // No receiver, a static method
        Race {
            name: String::from(name),
            laps: Vec::new(),
        }
    }

    fn add_lap(&mut self, lap: i32) {
        // Exclusive borrowed read-write access to self
        self.laps.push(lap);
    }

    fn print_laps(&self) {
        // Shared and read-only borrowed access to self
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {
        // Exclusive ownership of self
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

pub fn main() {
    let mut race = Race::new("Monaco Grand Prix");

    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);

    race.print_laps();

    race.finish();
    // race.add_lap(42); // error since race's ownership has moved
}
