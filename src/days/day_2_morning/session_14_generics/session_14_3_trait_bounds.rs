/// # 14.3 Trait Bounds
///
/// When working with generics, you often want to require the types to
/// implement some trait, so that you can call this trait’s methods.
///
/// You can do this with `T: Trait` or `impl Trait`.
///
/// - When multiple traits are necessary, use + to join them.
///
/// A bound can also be expressed using a `where` clause immediately
/// before the opening `{`, rather than at the type's first mention.
///
/// ```
///
/// fn duplicate<T>(a: T) -> (T, T)
/// where
///     T: Clone,
/// {
///     (a.clone(), a.clone())
/// }
///
/// ```
///
/// - It declutters the function signature if you have many parameters.
///
/// - It has additional features making it more powerful. Like the type
///   on the left of “:” can be arbitrary, like `Option<T>`.
///
/// Note that Rust does not (yet) support specialization. For example,
/// given the original `duplicate`, it is invalid to add a specialized
/// `duplicate(a: u32)`.
///

fn duplicate<T: Clone + Default>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

// struct NotClonable;

pub fn main() {
    let _foo = String::from("foo");
    let pair = duplicate(_foo);
    println!("{pair:?}");

    // let _bar = duplicate(NotClonable);
}
