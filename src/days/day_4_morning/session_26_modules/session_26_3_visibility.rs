/// # 26.3 Visibility
///
/// Modules are a privacy boundary:
///
/// - Module items are private by default (hides implementation details).
///
/// - Parent and sibling items are always visible.
///
/// - In other words, if an item is visible in module `foo`,
///   it’s visible in all the descendants of `foo`.
///
/// - Use the `pub` keyword to make modules public.
///
/// - Additionally, there are advanced `pub(...)` specifiers to restrict
///   the scope of public visibility.
///
/// - Configuring pub(crate) visibility is a common pattern.
///
/// - In any case, visibility must be granted to an ancestor module (and
///   all of its descendants).
///

mod outer {
    fn private() {
        println!("outer::private");
    }

    pub(crate) fn public() {
        println!("outer::public");
    }

    mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
            super::private();
        }
    }
}

pub fn main() {
    outer::public();
}
