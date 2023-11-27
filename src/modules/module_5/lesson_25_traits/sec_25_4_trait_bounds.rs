/// # Trait Bounds
///
/// When working with generics, you often want to require the types to
/// implement some trait, so that you can call this trait’s methods.
///
/// You can do this with `T: Trait` or `impl Trait`.
///
/// - A bound can also be expressed using a `where` clause immediately
///   before the opening `{`, rather than at the type's first mention.
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

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

// struct NotClonable;

pub fn main() {
    let _foo = String::from("foo");
    let pair = duplicate(_foo);
    println!("{pair:?}");

    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");

    // let _bar = duplicate(NotClonable);
}
