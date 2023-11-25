#![allow(unused_variables, dead_code)]

/// # Strings and Iterators
///
/// In this exercise, you are implementing a routing component of
/// a web server. The server is configured with a number of path
/// prefixes which are matched against request paths. The path
/// prefixes can contain a wildcard character which matches a full
/// segment.
///
use std::iter::repeat;

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let mut result = true;

    request_path
        .split('/')
        .map(|s| Some(s))
        .chain(repeat(None))
        .zip(prefix.split('/').map(|t| Some(t)))
        .for_each(
            |(request_segment, prefix_segment)| match (request_segment, prefix_segment) {
                (Some(rs), Some(ps)) => {
                    if rs != ps && ps != "*" {
                        result = false;
                    }
                }

                (None, Some(_)) => result = false,

                _ => {}
            },
        );

    result
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
