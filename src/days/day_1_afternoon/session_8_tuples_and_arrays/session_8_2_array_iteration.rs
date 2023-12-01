#![allow(dead_code)]

/// # 8.2 Array Iteration
///
/// The `for` statement supports iterating over arrays (but not tuples).
///
/// The `assert_ne!`, `assert_eq!` and `assert!` macros always checked
/// while, debug-only variants like `debug_assert!` compile to nothing
/// in release builds.
///

pub fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}
