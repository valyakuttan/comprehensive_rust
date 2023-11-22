/// # Stack And Heap Example
/// 
/// Creating a String puts fixed-sized metadata on the stack and
/// dynamically sized data, the actual string, on the heap:
/// 
/// ```
/// fn main() {
///     let s1 = String::from("Hello");
/// }
/// ```
///
///              Stack                                             Heap
///  --------------------------                 ---------------------------------------
/// |                          |               |                                       |
/// |      s1                  |               |                                       |
/// |      ---------------     |               |           ------------------          |
/// |     | ptr      | =================================> | H | e | l | l | o|         |            |    |
/// |     | len      |  5 |    |               |           ------------------          |
/// |     | capacity |  5 |    |               |                                       |
/// |     |          |    |    |               |                                       |
/// |      ---------------     |               |                                       |
/// |                          |               |                                       |
/// |                          |               |                                       |
///  --------------------------                 ---------------------------------------
/// 
/// - A `String` is backed by a `Vec`, so it has a capacity and length and can grow if
///   mutable via reallocation on the heap.
/// 
/// We can inspect the memory layout with `unsafe` Rust.
/// 

#[allow(dead_code)]
pub fn main() {
    let mut s1 = String::from("Hello");
    s1.push(' ');
    s1.push_str("world");
    // DON'T DO THIS AT HOME! For educational purposes only.
    // String provides no guarantees about its layout, so this could lead to
    // undefined behavior.
    unsafe {
        let (ptr, capacity, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
    }
}

