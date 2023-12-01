/// # 9.3 Exercise: Geometry
///
/// We will create a few utility functions for 3-dimensional geometry,
/// representing a point as `[f64;3]`. It is up to you to determine
/// the function signatures.
///

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

fn magnitude(v: &[f64; 3]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

fn normalize(v: &mut [f64; 3]) {
    let m = magnitude(v);
    v.iter_mut().for_each(|x| {
        if m != 0.0 {
            *x /= m
        }
    })
}

// Use the following `main` to test your work.

pub fn main() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
