/// # 10.3 Enums
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
/// Rust uses minimal space to store the discriminant.
///
/// - If necessary, it stores an integer of the smallest required size
///
///  - If the allowed variant values do not cover all bit patterns, it
///    will use invalid bit patterns to encode the discriminant
///    (the “niche optimization”). For example, `Option<&u8>` stores
///    either a pointer to an integer or NULL for the `None` variant.
///
/// -  You can control the discriminant if needed (e.g., for
///    compatibility with C):
///
/// ```
///
/// #[repr(u32)]
/// enum Bar {
///     A,  // 0
///     B = 10000,
///     C,  // 10001
/// }
///
/// fn enum_repr_example() {
///     println!("A: {}", Bar::A as u32);
///     println!("B: {}", Bar::B as u32);
///     println!("C: {}", Bar::C as u32);
/// }
///
/// ```
///
/// Without repr, the discriminant type takes 2 bytes, because
/// 10001 fits 2 bytes.
///
///

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

pub fn main() {
    let m = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);
}
