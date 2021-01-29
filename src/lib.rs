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

#![cfg_attr(rustc_1_6, no_std)]

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
#[must_use]
pub fn overlap<'a>(left: &'a str, right: &str) -> &'a str {
    left.char_indices()
        .map(|(index, _)| index)
        .find(|index| {
            left.len() - index <= right.len()
                && left.as_bytes()[*index..] == right.as_bytes()[..(left.len() - index)]
        })
        .map_or("", |index| &left[index..])
}

/// Provides methods for finding overlaps between values.
///
/// This trait provides methods for finding overlaps at both the start and end of `self`. This
/// allows for returning overlapping values that are owned by `self`, regardless of which side of
/// `self` the overlap is occurring.
///
/// This trait is made available by pulling it into scope:
///
/// ```
/// use str_overlap::Overlap;
/// ```
pub trait Overlap {
    /// Returns the overlap found at the start of `self` and the end of `other`.
    fn overlap_start(&self, other: &Self) -> &Self;
    /// Returns the overlap found at the end of `self` and the start of `other`.
    fn overlap_end(&self, other: &Self) -> &Self;
}

/// Overlap methods for string slices.
///
/// This allows for the returned string slice to be a subset of either string slice from which an
/// overlap is obtained.
impl Overlap for str {
    /// Returns the substring which is both the prefix to `self` and the suffix to `other`.
    ///
    /// The returned string slice is a reference to the substring contained in `self`.
    ///
    /// # Example
    /// ```
    /// use str_overlap::Overlap;
    ///
    /// assert_eq!("bcd".overlap_start("abc"), "bc");
    /// ```
    fn overlap_start(&self, other: &Self) -> &Self {
        other.char_indices()
            .map(|(index, _)| index)
            .find(|index| {
                other.len() - index <= self.len()
                    && other.as_bytes()[*index..] == self.as_bytes()[..(other.len() - index)]
            })
            .map_or("", |index| &self[..(other.len() - index)])
    }

    /// Returns the substring which is both the suffix to `self` and the prefix to `other`.
    ///
    /// The returned string slice is a reference to the substring contained in `self`.
    ///
    /// # Example
    /// ```
    /// use str_overlap::Overlap;
    ///
    /// assert_eq!("abc".overlap_end("bcd"), "bc");
    /// ```
    fn overlap_end(&self, other: &Self) -> &Self {
        self.char_indices()
            .map(|(index, _)| index)
            .find(|index| {
                self.len() - index <= other.len()
                    && self.as_bytes()[*index..] == other.as_bytes()[..(self.len() - index)]
            })
            .map_or("", |index| &self[index..])
    }
}

#[cfg(test)]
mod tests {
    use overlap;
    use Overlap;

    // overlap function.

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

    #[test]
    fn multi_byte() {
        assert_eq!(overlap("b日本語a", "語a日bc本"), "語a");
    }

    // Overlap trait.

    #[test]
    fn partial_overlap_start() {
        assert_eq!("bcd".overlap_start("abc"), "bc");
    }

    #[test]
    fn partial_overlap_end() {
        assert_eq!("abc".overlap_end("bcd"), "bc");
    }

    #[test]
    fn full_overlap_start() {
        assert_eq!("abc".overlap_start("abc"), "abc");
    }

    #[test]
    fn full_overlap_end() {
        assert_eq!("abc".overlap_end("abc"), "abc");
    }

    #[test]
    fn no_overlap_start() {
        assert_eq!("abc".overlap_start("def"), "");
    }

    #[test]
    fn no_overlap_end() {
        assert_eq!("abc".overlap_end("def"), "");
    }

    #[test]
    fn other_substring_of_self_start() {
        assert_eq!("abcd".overlap_start("abc"), "abc");
    }

    #[test]
    fn other_substring_of_self_end() {
        assert_eq!("abcd".overlap_end("bcd"), "bcd");
    }

    #[test]
    fn self_substring_of_other_start() {        
        assert_eq!("bcd".overlap_start("abcd"), "bcd");
    }

    #[test]
    fn self_substring_other_end() {
        assert_eq!("abc".overlap_end("abcd"), "abc");
    }

    #[test]
    fn only_checks_overlap_one_way_start() {
        assert_eq!("abc".overlap_start("bcd"), "");
    }

    #[test]
    fn only_checks_overlap_one_way_end() {
        assert_eq!("bcd".overlap_end("abc"), "");
    }

    #[test]
    fn self_empty_start() {
        assert_eq!("".overlap_start("abc"), "");
    }

    #[test]
    fn self_empty_end() {
        assert_eq!("".overlap_end("abc"), "");
    }

    #[test]
    fn other_empty_start() {
        assert_eq!("abc".overlap_start(""), "");
    }

    #[test]
    fn other_empty_end () {
        assert_eq!("abc".overlap_end(""), "");
    }

    #[test]
    fn all_empty_start() {
        assert_eq!("".overlap_end(""), "");        
    }

    #[test]
    fn all_empty_end() {
        assert_eq!("".overlap_start(""), "");
    }

    #[test]
    fn multi_byte_start() {
        assert_eq!("語a日bc本".overlap_start("b日本語a"), "語a");
    }

    #[test]
    fn multi_byte_end() {
        assert_eq!("b日本語a".overlap_end("語a日bc本"), "語a");
    }
}
