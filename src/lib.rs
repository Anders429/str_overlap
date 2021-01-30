//! This crate provides methods for finding the overlap between two string slices.
//!
//! An overlap is here defined as the largest substring contained both at the end of one string
//! slice and the beginning of another string slice.
//!
//! The implementation is provided through the [`Overlap`] trait, which is implemented on [`str`].
//! This allows the user to simply pull the trait into scope and use its methods:
//!
//! ```
//! use str_overlap::Overlap;
//!
//! assert_eq!("bcd".overlap_start("abc"), "bc");
//! assert_eq!("abc".overlap_end("bcd"), "bc");
//! ```
//!
//! The trait provides two methods: [`overlap_start`] and [`overlap_end`], which find the overlap
//! at the beginning and end of the first value respectively. The reason for these two methods is
//! to allow the user to specify ownership of the resulting subvalue, regardless of its overlap
//! position.
//!
//! [`overlap_end`]: Overlap::overlap_end
//! [`overlap_start`]: Overlap::overlap_start

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
#[inline]
#[must_use]
pub fn overlap<'a>(left: &'a str, right: &str) -> &'a str {
    left.overlap_end(right)
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
///
/// `Overlap` is implemented on [`str`], which means its methods are usable by `str` and any types
/// which implement [`Deref<Target = str>`], such as [`String`].
///
/// [`Deref<Target = str>`]: core::ops::Deref
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
pub trait Overlap {
    /// Returns the overlap found at the start of `self` and the end of `other`.
    ///
    /// # Example
    /// This method can be used through its implementation on [`str`], like so:
    ///
    /// ```
    /// use str_overlap::Overlap;
    ///
    /// assert_eq!("bcd".overlap_start("abc"), "bc");
    /// ```
    fn overlap_start(&self, other: &Self) -> &Self;
    /// Returns the overlap found at the end of `self` and the start of `other`.
    ///
    /// # Example
    /// This method can be used through its implementation on `str`, like so:
    ///
    /// ```
    /// use str_overlap::Overlap;
    ///
    /// assert_eq!("abc".overlap_end("bcd"), "bc");
    /// ```
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
