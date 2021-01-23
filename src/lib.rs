//! This crate provides a utility function for finding the overlap between two string slices.
//!
//! The overlap is here defined as the largest substring contained both at the end of the first
//! string slice and the beginning of the second string slice.
//!
//! # Example Usage
//! ```
//! use str_overlap::overlap;
//!
//! assert_eq!(overlap("abc", "bcd"), "bc");
//! ```
//!
//! The overlap is not evaluated on both sides of the strings to reduce complexity and maintain
//! clarity on where the overlapping substring is in relation to the input strings. If evaluation
//! of overlap is desired on both sides, both can be requested by calling the function twice:
//! ```
//! use str_overlap::overlap;
//!
//! let s1 = "abcd";
//! let s2 = "cdab";
//!
//! assert_eq!(overlap(s1, s2), "cd");
//! assert_eq!(overlap(s2, s1), "ab");
//! ```

/// Finds the overlap between two string slices.
///
/// The overlap is here defined as the largest substring contained at both the end of `left` and
/// the beginning of `right`. If no overlap exists, an empty string is returned.
///
/// # Example Usage
/// ```
/// use str_overlap::overlap;
///
/// assert_eq!(overlap("abc", "bcd"), "bc");
/// ```
pub fn overlap<'a>(left: &str, right: &'a str) -> &'a str {
    let mut substring = right;
    while !substring.is_empty() {
        if left.ends_with(&substring) {
            break;
        }
        substring = &substring[0..(substring.len() - substring.chars().last().unwrap().len_utf8())];
    }
    substring
}

#[cfg(test)]
mod tests {
    use crate::overlap;

    #[test]
    fn test_partial_overlap() {
        assert_eq!(overlap("abc", "bcd"), "bc");
    }

    #[test]
    fn test_full_overlap() {
        assert_eq!(overlap("abc", "abc"), "abc");
    }

    #[test]
    fn test_no_overlap() {
        assert_eq!(overlap("abc", "def"), "");
    }

    #[test]
    fn test_right_substring_of_left() {
        assert_eq!(overlap("abcd", "bcd"), "bcd");
    }

    #[test]
    fn test_left_substring_of_right() {
        assert_eq!(overlap("abc", "abcd"), "abc");
    }

    #[test]
    fn test_only_checks_overlap_one_way() {
        assert_eq!(overlap("bcd", "abc"), "");
    }

    #[test]
    fn test_left_empty() {
        assert_eq!(overlap("", "abc"), "");
    }

    #[test]
    fn test_right_empty() {
        assert_eq!(overlap("abc", ""), "");
    }

    #[test]
    fn test_both_empty() {
        assert_eq!(overlap("", ""), "");
    }
}
