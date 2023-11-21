/// # Enums
/// 
/// The `enum` keyword allows the creation of a type which has a
/// few different variants:
/// 
/// - Enumerations allow you to collect a set of values under one type
/// 
/// - The `enum` type `CoinFlip` has two variants `Heads`
///   and `Tails`.
/// 
/// ## Comparison of Structs and Enums: 
/// 
/// - In both, you can have a simple version without fields
///   (unit struct) or one with different types of fields (variant payloads).
/// 
/// - In both, associated functions are defined within an `impl` block.
/// 
/// - You could even implement the different variants of an enum with
///   separate structs but then they wouldn’t be the same type as they
///   would if they were all defined in an `enum`.
/// 


fn generate_random_number() -> i32 {
    // Implementation based on https://xkcd.com/221/
    4  // Chosen by fair dice roll. Guaranteed to be random.
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

#[allow(dead_code)]
pub fn main() {
    println!("You got: {:?}", flip_coin());
}
