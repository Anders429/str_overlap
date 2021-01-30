# str_overlap

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Anders429/str_overlap/Tests)](https://github.com/Anders429/str_overlap/actions)
[![codecov.io](https://img.shields.io/codecov/c/gh/Anders429/str_overlap)](https://codecov.io/gh/Anders429/str_overlap)
[![Crates.io](https://img.shields.io/crates/v/str_overlap)](https://crates.io/crates/str_overlap)
[![Docs.rs](https://docs.rs/str_overlap/badge.svg)](https://docs.rs/str_overlap)
[![MSRV](https://img.shields.io/badge/rustc-1.0.0+-yellow.svg)](#minimum-supported-rust-version)
[![License](https://img.shields.io/crates/l/str_overlap)](#license)

This crate provides methods for finding the overlap between two string slices.

An overlap is here defined as the largest substring contained both at the end of one string slice
and the beginning of another string slice.

## Usage
To use this crate, bring the
[`Overlap`](https://docs.rs/str_overlap/*/str_overlap//trait.Overlap.html) trait into scope. This
will provide [`str`](https://doc.rust-lang.org/std/primitive.str.html)s with two methods:
- [`overlap_start`](https://docs.rs/str_overlap/*/str_overlap//trait.Overlap.html#tymethod.overlap_start) - Finds the overlap at the **start** of the string slice and the **end** of another.
- [`overlap_end`](https://docs.rs/str_overlap/*/str_overlap//trait.Overlap.html#tymethod.overlap_end) - Finds the overlap at the **end** of the string slice and the **start** of another.

```rust
use str_overlap::Overlap;

assert_eq!("bcd".overlap_start("abc"), "bc");
assert_eq!("abc".overlap_end("bcd"), "bc");
```

The return value of these methods is a string slice, borrowed from the string the method is called
on. The two methods allows the caller to choose who owns the resulting string slice.

To use this crate, call the provided `overlap` function with two string slices in the left and
right positions.

```rust
use str_overlap::overlap;

assert_eq!(overlap("abc", "bcd"), "bc");
```

Note that the positions of the string slices matter. The overlap found is the largest substring at
both the end of the left string slice and the beginning of the right string slice.

## Performance
The `overlap` function has temporal complexity *O(n)* in the worst case (where no overlap is found),
where *n* is the length of the first string parameter.

## Minimum Supported Rust Version
This crate is guaranteed to compile on stable `rustc 1.0.0` and up.

## License
This project is licensed under either of

* Apache License, Version 2.0
([LICENSE-APACHE](https://github.com/Anders429/nested_containment_list/blob/HEAD/LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
([LICENSE-MIT](https://github.com/Anders429/nested_containment_list/blob/HEAD/LICENSE-MIT) or
http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
