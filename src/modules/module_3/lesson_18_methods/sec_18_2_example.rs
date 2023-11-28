/// # Example
///
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

#[allow(dead_code)]
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
