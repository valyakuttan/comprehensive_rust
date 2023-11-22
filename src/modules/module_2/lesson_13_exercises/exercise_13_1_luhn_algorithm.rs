/// # Luhn Algorithm
///
/// The Luhn algorithm is used to validate credit card numbers.
/// The algorithm takes a string as input and does the following
/// to validate the credit card number:
///
/// - Ignore all spaces. Reject number with less than two digits.
///
/// - Moving from right to left, double every second digit: for the
///   number 1234, we double 3 and 1. For the number 98765, we double
///   6 and 8.
///
/// - After doubling a digit, sum the digits if the result is greater
///   than 9. So doubling 7 becomes 14 which becomes 1 + 4 = 5.
///
/// - Sum all the undoubled and doubled digits.
///
/// - The credit card number is valid if the sum ends with 0.
///

pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;
    let mut digit_seen = 0;

    for c in cc_number.chars().filter(|&f| f != ' ').rev() {
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                sum += if double_digit > 9 {
                    double_digit - 9
                } else {
                    double_digit
                };
            } else {
                sum += digit;
            }
            double = !double;
            digit_seen += 1;
        } else {
            return false;
        }
    }

    if digit_seen < 2 {
        return false;
    }

    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
pub fn main() {
    luhn(" 0 0 ");
}
