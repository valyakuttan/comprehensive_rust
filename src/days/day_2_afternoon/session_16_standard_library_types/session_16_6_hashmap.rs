/// # 16.6 HashMap
///
/// Standard hash map with protection against **HashDoS** attacks:
///
/// - `HashMap` is not defined in the prelude and needs to be brought into scope.
///
/// - Unlike `vec!`, there is unfortunately no standard hashmap! macro.
///
/// - `HashMap` implements `From<[(K, V); N]>`, which allows us to easily initialize
///   a hash map from a literal array:
///
/// ```
/// let page_counts = HashMap::from([
///     ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
///     ("The Hunger Games".to_string(), 374),
/// ]);
/// ```
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
        println!(
            "We know about {} books, but not Les Misérables.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown."),
        }
    }

    // Use the .entry() method to insert a value if nothing is found.
    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:#?}");

    let pc1 = page_counts
        .get("Harry Potter and the Sorcerer's Stone")
        .unwrap_or(&336);

    assert_eq!(pc1, &336);
}
