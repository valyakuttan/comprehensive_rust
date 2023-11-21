/// # Variant Payloads
/// 
/// You can define richer enums where the variants carry data.
/// You can then use the `match` statement to extract the data
/// from each variant:
/// 
/// - The values in the enum variants can only be accessed after
///   being pattern matched. The pattern binds references to the
///   fields in the “match arm” after the `=>`.
/// 
/// - The expression is matched against the patterns from top to
///   bottom. There is no fall-through like in C or C++.
///
/// - The match expression has a value. The value is the last
///   expression in the match arm which was executed.
/// 
/// - Starting from the top we look for what pattern matches the value
///   then run the code following the arrow. Once we find a match, we stop.
/// 
/// - match inspects a hidden discriminant field in the enum.
/// 
/// - It is possible to retrieve the discriminant by calling
///   `std::mem::discriminant()`
/// 
/// - This is useful, for example, if implementing `PartialEq` for structs
///   where comparing field values doesn’t affect equality.
/// 
/// - `WebEvent::Click { ... }` is not exactly the same as `WebEvent::Click(Click)`
///   with a top level `struct Click { ... }`. The inlined version cannot implement
///   traits, for example.
/// 

enum WebEvent {
    PageLoad,                 // Variant without payload
    KeyPress(char),           // Tuple struct variant
    Click { x: i64, y: i64 }, // Full struct variant
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad       => println!("page loaded"),
        WebEvent::KeyPress(c)    => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

#[allow(dead_code)]
pub fn main() {
    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(press);
    inspect(click);
}

