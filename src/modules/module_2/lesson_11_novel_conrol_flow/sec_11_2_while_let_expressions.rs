/// # `while let` loops
/// 
/// Like with `if let`, there is a `while let` variant which
/// repeatedly tests a value against a pattern:
/// 
/// In the code below the iterator returned by `v.into_iter()` will return a
/// `Option<i32>` on every call to `next()`. It returns `Some(x)` until it is
/// done, after which it will return `None`. The `while let` lets us keep
/// iterating through all items.
/// 

#[allow(dead_code)]
pub fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}
