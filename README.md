[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Anders429/str_overlap/Tests)](https://github.com/Anders429/str_overlap/actions)
[![codecov.io](https://img.shields.io/codecov/c/gh/Anders429/str_overlap)](https://codecov.io/gh/Anders429/str_overlap)
[![Crates.io](https://img.shields.io/crates/v/str_overlap)](https://crates.io/crates/str_overlap)
![Crates.io](https://img.shields.io/crates/l/str_overlap)
[![Docs.rs](https://docs.rs/str_overlap/badge.svg)](https://docs.rs/str_overlap)

This crate provides a utility function for finding the overlap between two string slices.

The overlap is here defined as the largest substring contained both at the end of the first
string slice and the beginning of the second string slice.

# Usage
To use this crate, call the provided `overlap` function with two string slices in the left and
right positions.

```rust
use str_overlap::overlap;

assert_eq!(overlap("abc", "bcd"), "bc");
```

Note that the positions of the string slices matter. The overlap found is the largest substring at
both the end of the left string slice and the beginning of the right string slice.
