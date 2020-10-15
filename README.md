This crate provides a utility function for finding the overlap between two string slices.

The overlap is here defined as the largest substring contained both at the end of the first
string slice and the beginning of the second string slice.

# Usage
To use this crate, call the provided `overlap` function with two string slices in the left and
right positions.

```rust
use string_overlap::overlap;

assert_eq!(overlap("abc", "bcd"), "bc");
```

Note that the positions of the string slices matter. The overlap found is the largest substring at
both the end of the left string slice and the beginning of the right string slice.