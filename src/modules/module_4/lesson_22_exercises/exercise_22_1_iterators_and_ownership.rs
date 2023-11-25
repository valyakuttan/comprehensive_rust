/// # Iterators and Ownership
///
/// The ownership model of Rust affects many APIs. An example of this
/// is the `Iterator` and `IntoIterator` traits.
///
/// ## Iterator
///
/// Traits are like interfaces: they describe behavior (methods) for a
/// type. The `Iterator` trait simply says that you can call `next`
/// until you get `None` back:
///
/// ```
///
/// pub trait Iterator {
///     type Item;
///     fn next(&mut self) -> Option<Self::Item>;
/// }
///
/// ```
///
/// You use this trait like this:
///
/// ```
///
/// fn main() {
///     let v: Vec<i8> = vec![10, 20, 30];
///     let mut iter = v.iter();
///
///     println!("v[0]: {:?}", iter.next());
///     println!("v[1]: {:?}", iter.next());
///     println!("v[2]: {:?}", iter.next());
///     println!("No more items: {:?}", iter.next());
/// }
///
/// ```
///
/// - For an iterator over a collection containing elements of type `T`, type
///   of the object returned by `next()` is `Option<&T>`.
///
/// - We need a mutable reference to an iterator to call `next()`, since `next()`
///   will mutate the `Iterator` object.
///
/// ## IntoIterator
///
/// The `Iterator` trait tells you how to iterate once you have created an
/// iterator. The related trait `IntoIterator` tells you how to create the
/// iterator:
///
/// ```
///
/// pub trait IntoIterator {
///     type Item;
///     type IntoIter: Iterator<Item = Self::Item>;
///
///     fn into_iter(self) -> Self::IntoIter;
/// }
///
/// ```
///
/// The syntax here means that every implementation of `IntoIterator` must
/// declare two types:
///
/// - `Item`: the type we iterate over, such as `i8`,
///
/// - `IntoIter`: the Iterator type returned by the `into_iter` method.
///
/// Note that `IntoIter` and `Item` are linked: the iterator must have the
/// same `Item` type, which means that it returns `Option<Item>`.
///
/// # `for` Loops
///
/// A `for` loop call `into_iter()` on an expression and iterates over the
/// resulting iterator:
///
/// ```
///
/// fn main() {
///     let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
///
///     for word in &v {
///         println!("word: {word}");
///     }
///
///     for word in v {
///         println!("word: {word}");
///     }
/// }
///
/// ```
///

#[allow(dead_code)]
pub fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        println!("word: {word}");
    }

    for word in v {
        println!("word: {word}");
    }
}
