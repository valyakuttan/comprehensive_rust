#![allow(dead_code)]

/// # 14.5 Exercise: Generic min
///
/// In this short exercise, you will implement a generic `min` function
/// that determines the minimum of two values, using a `LessThan` trait.
///

trait LessThan {
    /// Return true if self is less than other.
    fn less_than(&self, other: &Self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Citation {
    author: &'static str,
    year: u32,
}

impl LessThan for Citation {
    fn less_than(&self, other: &Self) -> bool {
        // if self.author < other.author {
        //     true
        // } else if self.author > other.author {
        //     false
        // } else {
        //     self.year < other.year
        // }

        match self.author.cmp(other.author) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => self.year < other.year,
            std::cmp::Ordering::Greater => false,
        }
    }
}

// TODO: implement the `min` function used in `main`.
fn min<T: LessThan>(a: T, b: T) -> T {
    if a.less_than(&b) {
        a
    } else {
        b
    }
}

pub fn main() {
    let cit1 = Citation {
        author: "Shapiro",
        year: 2011,
    };
    let cit2 = Citation {
        author: "Baumann",
        year: 2010,
    };
    let cit3 = Citation {
        author: "Baumann",
        year: 2019,
    };
    debug_assert_eq!(min(cit1, cit2), cit2);
    debug_assert_eq!(min(cit2, cit3), cit2);
    debug_assert_eq!(min(cit1, cit3), cit3);
}
