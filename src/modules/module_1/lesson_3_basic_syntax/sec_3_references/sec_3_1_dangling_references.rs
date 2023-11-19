/// # Dangling References
/// 
/// Rust will statically forbid dangling references:
/// 
/// - A reference is said to “borrow” the value it refers to.
/// 
/// - Rust is tracking the lifetimes of all references to ensure
///   they live long enough.
///

#[allow(unused_variables)]
pub fn main() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        // ref_x = &x; // can't assign because ref_x outlives x
    }
    // println!("ref_x: {ref_x}"); // ref_x is uninitialized
}
