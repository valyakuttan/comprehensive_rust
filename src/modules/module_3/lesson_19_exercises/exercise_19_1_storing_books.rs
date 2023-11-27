#![allow(dead_code, clippy::needless_doctest_main, clippy::boxed_local)]

/// # Storing Books
///
/// Use the code below to learn about `Vec<T>` API, and
/// model a library’s book collection.
///
/// ```
/// fn main() {
///     let mut vec = vec![10, 20];
///     vec.push(30);
///     let midpoint = vec.len() / 2;
///     println!("middle value: {}", vec[midpoint]);
///     for item in &vec {
///         println!("item: {item}");
///     }
/// }
/// ```
///
struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Notice how the `self` parameter
// changes type to indicate the method's required level of ownership
// over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("Title: {}, Year: {}", book.title, book.year);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        // Using a closure and a built-in method:
        self.books.iter().min_by_key(|book| book.year)

        // Longer hand-written solution:
        //let mut oldest: Option<&Book> = None;
        //for book in self.books.iter() {
        //    if oldest.is_none() || book.year < oldest.unwrap().year {
        //        oldest = Some(book);
        //    }
        //}
        // oldest
    }
}

#[allow(dead_code)]
pub fn main() {
    let mut library = Library::new();

    println!(
        "The library is empty: library.is_empty() -> {}",
        library.is_empty()
    );

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!(
        "The library is no longer empty: library.is_empty() -> {}",
        library.is_empty()
    );

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    library.print_books();
}

#[test]
fn test_library_len() {
    let mut library = Library::new();
    assert_eq!(library.len(), 0);
    assert!(library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    assert_eq!(library.len(), 2);
    assert!(!library.is_empty());
}

#[test]
fn test_library_is_empty() {
    let mut library = Library::new();
    assert!(library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    assert!(!library.is_empty());
}

#[test]
fn test_library_print_books() {
    let mut library = Library::new();
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    // We could try and capture stdout, but let us just call the
    // method to start with.
    library.print_books();
}

#[test]
fn test_library_oldest_book() {
    let mut library = Library::new();
    assert!(library.oldest_book().is_none());

    library.add_book(Book::new("Lord of the Rings", 1954));
    assert_eq!(
        library.oldest_book().map(|b| b.title.as_str()),
        Some("Lord of the Rings")
    );

    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    assert_eq!(
        library.oldest_book().map(|b| b.title.as_str()),
        Some("Alice's Adventures in Wonderland")
    );
}

#[test]
fn test_add_book() {
    let mut library = Library::new();
    assert!(library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    assert_eq!(library.len(), 1);

    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    assert_eq!(library.len(), 2);
}

#[test]
fn test_oldest_book() {
    let mut library = Library::new();
    assert!(library.oldest_book().is_none());

    library.add_book(Book::new("Lord of the Rings", 1954));
    match library.oldest_book() {
        Some(o1) => {
            assert_eq!(o1.title, "Lord of the Rings".to_string());
            assert_eq!(o1.year, 1954);
        }

        _ => {
            unreachable!()
        }
    }

    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    match library.oldest_book() {
        Some(o1) => {
            assert_eq!(o1.title, "Alice's Adventures in Wonderland".to_string());
            assert_eq!(o1.year, 1865);
        }

        _ => {
            unreachable!()
        }
    }
}
