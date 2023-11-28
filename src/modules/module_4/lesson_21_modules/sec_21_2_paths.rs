/// # Paths
///
/// ## Paths are resolved as follows:
///
/// As a relative path:
/// - `foo` or `self::foo` refers to `foo` in the current module,
///
/// - `super::foo` refers to `foo` in the parent module.
///
/// As an absolute path:
///
/// - ` crate::foo` refers to `foo` in the root of the current crate,
///
/// - `bar::foo` refers to `foo` in the `bar` crate.
///
///
/// ```
///
/// use std::collections::HashSet;
/// use std::mem::transmute;
/// use std::collections::BTreeSet as Set;
///
/// ```
/// A module can bring symbols from another module into scope with `use`.
///

#[allow(dead_code)]
pub fn main() {}
