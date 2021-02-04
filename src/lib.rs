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

// Since the MSRV is 1.0.0, allowing usage of deprecated items is ok, as the replacements are likely
// not available in early versions.
#![allow(deprecated)]
#![cfg_attr(rustc_1_6, no_std)]

#[cfg(not(rustc_1_6))]
extern crate std as core;

/// Shared logic for finding the index at which two strings overlap.
///
/// The `left` and `right` parameters are, conceptually, defined as follows:
/// - `left` is the parameter whose suffix will be overlapping
/// - `right` is the parameter whose prefix will be overlapping
///
/// If no overlap exists, the returned index will be the length of `left`. This allows the result to
/// be used to create an empty slice.
#[inline]
#[must_use]
fn string_overlap_index(left: &str, right: &str) -> usize {
    left.char_indices()
        .map(|(index, _)| index)
        .find(|index| {
            let slice_len = left.len() - index;
            slice_len <= right.len()
                && unsafe {
                    // SAFETY: `index` is obtained from `left`'s `CharIndices`, so it will always be
                    // within the bounds of `left`. Additionally, `index` will also always be on
                    // UTF-8 character bounds of `left`.
                    left.slice_unchecked(*index, left.len()).as_bytes()
                    // SAFETY: Since `slice_len - index` is less than or equal to `right.len()`,
                    // `slice_len` will always be within the bounds of `right`. Additionally, since
                    // the string slice is cast to bytes, we don't need to worry about whether the
                    // slice occurs on a valid UTF-8 character bound.
                        == right.slice_unchecked(0, slice_len).as_bytes()
                }
        })
        .unwrap_or_else(|| left.len())
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
    #[inline]
    #[must_use]
    fn overlap_start(&self, other: &Self) -> &Self {
        unsafe {
            // SAFETY: The result of `string_overlap_index()` subtracted from `other.len()` will
            // always be on a character bound of `self`, since it is found by comparing directly the
            // bytes of the start of `self` and the end of `other`. Therefore, the range will be
            // within `self`'s bounds and also will uphold `str` invariants.
            self.slice_unchecked(0, other.len() - string_overlap_index(other, self))
        }
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
    #[inline]
    #[must_use]
    fn overlap_end(&self, other: &Self) -> &Self {
        unsafe {
            // SAFETY: The result of `string_overlap_index()` will always be on a character bound of
            // `self`, since it is found from running over the CharIndices of `self`. Therefore, the
            // range will be within `self`'s bounds and also will uphold `str` invariants.
            self.slice_unchecked(string_overlap_index(self, other), self.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use Overlap;

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
    fn other_empty_end() {
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
