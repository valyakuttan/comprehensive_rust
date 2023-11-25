/// # Modules
///
/// An `impl` blocks let us namespace functions to a type.
/// Similarly, `mod` lets us namespace types and functions:
///
/// - Packages provide functionality and include a `Cargo.toml`
///   file that describes how to build a bundle of crates.
///
/// - Crates are a tree of modules, where a binary crate creates
///   an executable and a library crate compiles to a library.
///
/// - Modules define organization, scope.
///

mod foo {
    pub fn do_something() {
        println!("In the foo module");
    }
}

mod bar {
    pub fn do_something() {
        println!("In the bar module");
    }
}

#[allow(dead_code)]
pub fn main() {
    foo::do_something();
    bar::do_something();
}
