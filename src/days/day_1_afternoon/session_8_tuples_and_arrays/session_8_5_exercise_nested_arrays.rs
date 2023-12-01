#![allow(dead_code, clippy::needless_range_loop)]

/// # 8.5 Exercise: Nested Arrays
///
/// Arrays can contain other arrays:
///
/// ```
///
/// let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
///
/// ```
///
/// Use an array such as the above to write a function `transpose`
/// which will transpose a matrix (turn rows into columns):
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
