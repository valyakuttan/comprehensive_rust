/// 19.5 Clone
///
/// Sometimes you want to make a copy of a value. The `Clone` trait
/// accomplishes this.
///
/// The idea of `Clone` is to make it easy to spot where heap allocations
/// are occurring. Look for `.clone()` and a few others like `Vec::new` or
/// `Box::new`.
///
/// It’s common to “clone your way out” of problems with the borrow checker,
/// and return later to try to optimize those clones away.
///

#[derive(Default)]
struct Backends {
    hostnames: Vec<String>,
    weights: Vec<f64>,
}

impl Backends {
    fn set_hostnames(&mut self, hostnames: &[String]) {
        self.hostnames = hostnames.to_owned();
        self.weights = hostnames.iter().map(|_| 1.0).collect();
    }
}

pub fn main() {
    let v = vec![String::from("Hello"), String::from("Rust")];

    let back = &mut Backends::default();
    back.set_hostnames(&v);
}
