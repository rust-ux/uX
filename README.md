# uX [![Build Status](https://travis-ci.org/kjetilkjeka/uX.svg?branch=master)](https://travis-ci.org/kjetilkjeka/uX) [![Crates.io](https://img.shields.io/crates/v/ux.svg)](https://crates.io/crates/ux)

> Non standard integer types like `u7`, `u9`, `u10`, `u63`, `i7`, `i9` etc

When non-standard-width integers is required in an applications, the norm is to use a larger container and make sure the value is within range after manipulation. uX aims to take care of this once and for all by:
 - Providing `u2`-`u63` and `i2`-`i63` types that should behave as similar as possible to the built in rust types
     - The methods of the defined types are the same as for the built in types (far from all is implemented at this point but fill out an issue or create a PR if something essential for you is missing)
     - Overflow will panic in debug and wrap in release.
 - When `i128` and `u128` is stabilized this crate will also support `u65-u127` and `i65-i127`
 - All possible lossless conversions is possible by using `From`.
 - When `TryFrom` is stabilized fallible conversions will also be supported.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
