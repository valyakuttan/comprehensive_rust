/// # Deriving Traits
///
/// Rust `derive` macros work by automatically generating code that
/// implements the specified traits for a data structure. You can let
/// the compiler derive a number of traits as follows:
///

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

pub fn main() {
    let p1 = Player::default();
    let p2 = p1.clone();
    println!(
        "Is {:?}\nequal to {:?}?\nThe answer is {}!",
        &p1,
        &p2,
        if p1 == p2 { "yes" } else { "no" }
    );
}
