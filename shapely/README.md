# shapely

[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/shapely.svg)](https://crates.io/crates/shapely)
[![documentation](https://docs.rs/shapely/badge.svg)](https://docs.rs/shapely)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely.svg)](./LICENSE)

shapely provides runtime reflection for Rust.

Any type that implements `Shapely` trait returns a `Shape`, which describes:

  * The memory layout of the type
  * Its innards: struct fields, underlying type for newtypes, etc.
  * How to invoke its Display/Debug implementation
  * How to drop it in place

The `Partial` type is able to allocate (or work from a `&mut MaybeUninit<T>`)
any Shapely type, and gradually initialize its fields — until the fully-built
value is moved out of the partial.

It comes with a derive macro that uses [unsynn](https://crates.io/crates/unsynn)
for speed of compilation.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
