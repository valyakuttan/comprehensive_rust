/// # 25.2
///
/// IntoIterator
///
/// The `Iterator` trait tells you how to iterate once you have created an
/// iterator. The related trait `IntoIterator` defines how to create an
/// iterator for a type. It is used automatically by the `for` loop.
///
///
/// Every implementation of IntoIterator must declare two types:
///
/// - `Item`: the type to iterate over, such as `i8`,
///
/// - `IntoIter`: the `Iterator` type returned by the `into_iter` method.
///
/// Note that `IntoIter` and `Item` are linked: the iterator must have the
/// same `Item` type, which means that it returns `Option<Item>`
///
/// The example iterates over all combinations of `x` and `y` coordinates.
///
/// Note that you can't iterate over the grid twice in `main`.
///
/// This is because `IntoIterator::into_iter` takes ownership of `self`.
///
/// You can fix this issue by implementing `IntoIterator` for `&Grid` and
/// storing a reference to the `Grid` in `GridIter`.
///
/// The same problem can occur for standard library types:
///
/// `for e in some_vector` will take ownership of `some_vector` and iterate
/// over owned elements from that vector. Use `for e in &some_vector`
/// instead, to iterate over references to elements of `some_vector`.
///

struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl IntoIterator for Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter;
    fn into_iter(self) -> GridIter {
        GridIter {
            grid: self,
            i: 0,
            j: 0,
        }
    }
}

struct GridIter {
    grid: Grid,
    i: usize,
    j: usize,
}

impl Iterator for GridIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        self.i += 1;
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]))
    }
}

pub fn main() {
    let grid = Grid {
        x_coords: vec![3, 5, 7, 9],
        y_coords: vec![10, 20, 30, 40],
    };
    for (x, y) in grid {
        println!("point = {x}, {y}");
    }
}
