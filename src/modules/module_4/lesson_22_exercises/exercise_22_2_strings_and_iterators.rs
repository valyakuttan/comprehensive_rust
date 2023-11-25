#![allow(unused_variables, dead_code)]

/// # Strings and Iterators
///
/// In this exercise, you are implementing a routing component of
/// a web server. The server is configured with a number of path
/// prefixes which are matched against request paths. The path
/// prefixes can contain a wildcard character which matches a full
/// segment.
///

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let mut request_segments = request_path.split('/');

    for prefix_segment in prefix.split('/') {
        let Some(request_segment) = request_segments.next() else {
            return false;
        };
        if request_segment != prefix_segment && prefix_segment != "*" {
            return false;
        }
    }
    true

    // Alternatively, Iterator::zip() lets us iterate simultaneously over prefix
    // and request segments. The zip() iterator is finished as soon as one of
    // the source iterators is finished, but we need to iterate over all request
    // segments. A neat trick that makes zip() work is to use map() and chain()
    // to produce an iterator that returns Some(str) for each pattern segments,
    // and then returns None indefinitely.
    //
    // let mut result = true;
    //
    // request_path
    //     .split('/')
    //     .map(|s| Some(s))
    //     .chain(std::iter::repeat(None))
    //     .zip(prefix.split('/').map(|t| Some(t)))
    //     .for_each(
    //         |(request_segment, prefix_segment)| match (request_segment, prefix_segment) {
    //             (Some(rs), Some(ps)) => {
    //                 if rs != ps && ps != "*" {
    //                     result = false;
    //                 }
    //             }
    //
    //             (None, Some(_)) => result = false,
    //
    //             _ => {}
    //         },
    //     );
    //
    // result
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

pub fn main() {}
