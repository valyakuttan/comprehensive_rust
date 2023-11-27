#![allow(dead_code, clippy::needless_doctest_main, clippy::needless_range_loop)]

/// # Arrays and for Loops
///
/// An array can be declared as:
///
/// ```
/// let array = [10, 20, 30];
/// ```
///
///  Such an array can be printed by asking for its debug representation
///  with `{:?}`:
///
/// ```
/// fn main() {
///   let array = [10, 20, 30];
///   println!("array: {array:?}");
/// }
/// ```
///
/// Rust lets you iterate over things like arrays and ranges using the `for` keyword:
///
/// ```
/// fn main() {
///   let array = [10, 20, 30];
///   print!("Iterating over array:");
///   for n in &array {
///     print!(" {n}");
///   }
///   println!();
///
///   print!("Iterating over range:");
///   for i in 0..3 {
///     print!(" {}", array[i]);
///   }
///   println!();
/// }
///  ```
///
///

fn transpose(matrix: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{row:?}");
    }
}

#[test]
fn test_transpose() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    let transposed = transpose(&matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

pub fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(&matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
