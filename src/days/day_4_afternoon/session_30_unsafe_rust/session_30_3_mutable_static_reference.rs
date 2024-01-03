/// # 30.3 Mutable Static Variables
///
/// It is safe to read an immutable static variable:
///
/// ```
///
///  static HELLO_WORLD: &str = "Hello, world!";
///
///  fn static_immutable() {
///      println!("HELLO_WORLD: {HELLO_WORLD}");
///  }
///
/// ```
///
/// However, since data races can occur, it is unsafe to read and write mutable static variables:
///
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn main() {
    add_to_counter(42);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}
