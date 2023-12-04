/// # 25.3 `FromIterator`
///
/// `FromIterator` lets you build a collection from an `Iterator`.
///
/// `Iterator` implements
///
/// ```
///
/// fn collect<B>(self) -> B
///     where
///         B: FromIterator<Self::Item>,
///         Self: Sized,
///
/// ```
///
/// There are two ways to specify `B` for this method:
///
/// - With the “turbofish”: `some_iterator.collect::<COLLECTION_TYPE>()`,
///   as shown. The `_` shorthand used here lets Rust infer the type of
///   the `Vec` elements.
///
/// - With type inference: let prime_squares: Vec<_> = some_iterator.collect()``.
///
/// There are basic implementations of `FromIterator` for `Vec`,
/// `HashMap`, etc.
///
/// There are also implementations which let you do cool things like
/// convert an `Iterator<Item = Result<V, E>>` into a `Result<Vec<V>, E>`.
///
/// ```
///
///    let v1: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
///    let result = v1.into_iter().collect::<Result<Vec<_>, _>>();
///    println!("{result:?}");
///
///    let v1 = vec![Ok(1), Ok(2), Ok(3), Err("ouch!"), Err("nah..")];
///    let result = v1.into_iter().collect::<Result<Vec<_>, _>>();
///    println!("{result:?}");
///
/// ```
///

pub fn main() {
    let primes = vec![2, 3, 5, 7];

    let prime_squares = primes
        .into_iter()
        .map(|prime| prime * prime)
        .collect::<Vec<_>>();

    println!("prime_squares: {prime_squares:?}");
}
