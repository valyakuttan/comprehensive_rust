/// # Tuple Structs
///
/// If the field names are unimportant, you can use a tuple struct:
///
/// ```
/// struct Point(i32, i32);
///
/// fn main() {
///    let p = Point(17, 23);
///    println!("({}, {})", p.0, p.1);
/// }
/// ```
///
/// This is often used for single-field wrappers (called newtypes):
///
/// ```
/// struct PoundsOfForce(f64);
/// struct Newtons(f64);
///
/// fn compute_thruster_force() -> PoundsOfForce {
///     todo!("Ask a rocket scientist at NASA")
/// }
///
/// fn set_thruster_force(force: Newtons) {
///     // ...
/// }
///
/// fn main() {
///     let force = compute_thruster_force();
///     set_thruster_force(force);
/// }
/// ```
///
/// - Newtypes are a great way to encode additional information about the value
///   in a primitive type, for example:
/// 
///   - The number is measured in some units: `Newtons` in the example above.
/// 
///   - The value passed some validation when it was created, so you no longer
///     have to validate it again at every use: `PhoneNumber(String)` or
///     `OddNumber(u32)`.
/// 

use std::convert::From;

const POUNDS_TO_NEWTONS: f64 = 4.44822162;
struct PoundsOfForce(f64);
struct Newtons(f64);

impl From<PoundsOfForce> for Newtons {
    fn from(value: PoundsOfForce) -> Self {
        Newtons(POUNDS_TO_NEWTONS * value.0)
    }
}
fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    assert_eq!(force.0 > 0.0, true);
}

#[allow(dead_code)]
pub fn main() {
    let force = compute_thruster_force();
    set_thruster_force(force.into());
}
