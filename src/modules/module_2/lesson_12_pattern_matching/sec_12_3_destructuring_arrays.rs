/// # Destructuring Arrays
/// 
/// You can destructure arrays, tuples, and slices by matching
/// on their elements:
///
/// - Destructuring of slices of unknown length also works with patterns
///   of fixed length.
/// 
/// - You can match against the tail with patterns like `[.., b]` and `[a@..,b]`
///   where `b` matches against an element while `a` against a slice
/// 

#[rustfmt::skip]
fn inspect(slice: &[i32]) {
    print!("Tell me about {slice:?}: ");
    match slice {
        &[0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        &[1, ..]   => println!("First is 1 and the rest were ignored"),
        &[.., b] => println!("Last element is {b}"),
        _          => println!("All elements were ignored"),
    }
}

#[rustfmt::skip]
#[allow(dead_code)]
pub fn main() {
    let triple = [0, -2, 3];
    print!("Tell me about {triple:?}: ");

    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..]   => println!("First is 1 and the rest were ignored"),
        [a@.., 1] => print!("First part is {a:?}, last element is 1"),
        _         => println!("All elements were ignored"),
    }

    inspect(&[0, -2, 3]);
    inspect(&[0, -2, 3, 4]);
}
