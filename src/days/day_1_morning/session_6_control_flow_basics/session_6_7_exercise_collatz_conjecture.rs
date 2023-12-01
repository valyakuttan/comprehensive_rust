#![allow(dead_code)]

/// # 6.7 Exercise: Collatz Sequence
///
/// The Collatz Sequence is defined as follows, for an arbitrary
/// $n_1$ greater than zero:
///
/// - If $n_i$ is 1, then the sequence terminates at $n_i$.
///
/// - If $n_i$ is even, then $n_{i+1}$ = $n_i$ / 2.
///
/// - If $n_i$ is odd, then $n_{i+1} = 3 * $n_i$ + 1.
///

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;

    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

pub fn main() {
    let n = 127;

    println!("{}", collatz_length(n));
}
