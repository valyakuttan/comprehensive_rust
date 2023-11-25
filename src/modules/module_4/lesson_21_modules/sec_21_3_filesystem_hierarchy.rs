#![allow(dead_code)]
/// # Filesystem Hierarchy
///
/// Omitting the module content will tell Rust to look for
/// it in another file:
///
/// ```
///
/// mod garden;
///
/// ```
///
/// Will tells Rust that the `garden` module content is found at
/// `src/garden.rs`. Similarly, a `garden::vegetables` module can be
/// found at `src/garden/vegetables.rs`.
///
/// The crate root is in:
///
/// - `src/lib.rs` (for a library crate)
///
/// - `src/main.rs` (for a binary crate)
///
/// Modules defined in files can be documented, too, using “inner doc comments”.
/// These document the item that contains them.
///
/// ```
///
/// //! This module implements the garden, including a highly performant germination
/// //! implementation.
///
/// // Re-export types from this module.
/// pub use seeds::SeedPacket;
/// pub use garden::Garden;
///
/// /// Sow the given seed packets.
/// pub fn sow(seeds: Vec<SeedPacket>) { todo!() }
///
/// /// Harvest the produce in the garden that is ready.
/// pub fn harvest(garden: &mut Garden) { todo!() }
///
/// ```

pub fn main() {}
