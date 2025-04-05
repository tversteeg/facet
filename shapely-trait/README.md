# shapely-trait

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/shapely)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/shapely-trait.svg)](https://crates.io/crates/shapely-trait)
[![documentation](https://docs.rs/shapely-trait/badge.svg)](https://docs.rs/shapely-trait)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely-trait.svg)](./LICENSE)

This exports the `Shapely` trait, which exposes an associated const `SHAPE` of type
`Shape`, a struct defined in the `shapely-types` crate.

This crate also provides implementations of `Shapely` for most standard types, like:

  * integers
  * booleans
  * strings
  * collections like `Vec<T>`, and `HashMap<K, V>`
  * `[T; 1]` (for now)
  * `&[T]`
  * `Bytes` (a wrapper around `Vec<u8>`)
  * `()`, `(T0,)`

For more detailed information and usage examples, please refer to the [shapely crate documentation](https://docs.rs/shapely).

### Funding

Thanks to Namespace for providing fast GitHub Actions workers:

<a href="https://namespace.so"><img src="./static/namespace-d.svg" height="40"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
