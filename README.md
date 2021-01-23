[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Anders429/str_overlap/Tests)](https://github.com/Anders429/str_overlap/actions)
[![codecov.io](https://img.shields.io/codecov/c/gh/Anders429/str_overlap)](https://codecov.io/gh/Anders429/str_overlap)
[![Crates.io](https://img.shields.io/crates/v/str_overlap)](https://crates.io/crates/str_overlap)
[![Docs.rs](https://docs.rs/str_overlap/badge.svg)](https://docs.rs/str_overlap)
[![MSRV](https://img.shields.io/badge/rustc-1.0.0+-yellow.svg)](#minimum-supported-rust-version)
[![License](https://img.shields.io/crates/l/str_overlap)](#license)

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

# Minimum Supported Rust Version
This crate is guaranteed to compile on stable `rustc 1.0.0` and up.

# License
This project is licensed under either of

* Apache License, Version 2.0
([LICENSE-APACHE](https://github.com/Anders429/nested_containment_list/blob/HEAD/LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
([LICENSE-MIT](https://github.com/Anders429/nested_containment_list/blob/HEAD/LICENSE-MIT) or
http://opensource.org/licenses/MIT)

at your option.
