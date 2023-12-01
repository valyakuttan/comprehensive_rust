/// # 10. 5 Type Aliases
///
/// A type alias creates a name for another type. The two types
/// can be used interchangeably.
///

enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// Aliases are more useful with long, complex types:
use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;

pub fn main() {}
