# shapely

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/shapely)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/shapely.svg)](https://crates.io/crates/shapely)
[![documentation](https://docs.rs/shapely/badge.svg)](https://docs.rs/shapely)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely.svg)](./LICENSE)

shapely provides runtime reflection for Rust.

Any type that implements `Shapely` trait returns a `Shape`, which describes:

  * The memory layout of the type
  * Its definition: struct fields, underlying type for newtypes, etc.
  * How to drop it in place

The `Poke` type is able to allocate (or work from a `&mut MaybeUninit<T>`)
any Shapely type, and gradually initialize its fields — until the fully-built
value is moved out of the partial.

The `Peek` type helps perform read operations on any Shapely type.

It comes with a derive macro that uses [unsynn](https://crates.io/crates/unsynn)
for speed of compilation.

## Ecosystem

The main `shapely` crate re-exports symbols from:

- [shapely-trait](../shapely-trait), which defines the main `Shapely` trait and implements it for foreign types (mostly `libstd`)
- [shapely-types](../shapely-types), which defines the `Shape` struct, along with various vtables and the whole `Def` tree
- [shapely-opaque](../shapely-opaque), which provides helpers around type-erased pointers like
  `OpaqueUninit`, `OpaqueConst`, `Opaque`
- [shapely-derive](../shapely-derive), which implements the `Shapely` derive attribute as a fast/light proc macro powered by [unsynn](https://docs.rs/unsynn)
- [shapely-spez](../shapely-spez), which implements an autoderef specialization trick needed for `shapely-derive`
- [shapely-peek](../shapely-peek), which allows reading arbitrary `Shapely` types
- [shapely-poke](../shapely-poke), which allows building/altering arbitrary `Shapely` types

shapely supports deserialization from multiple data formats through dedicated crates:

- [shapely-json](../shapely-json): JSON deserialization
- [shapely-yaml](../shapely-yaml): YAML deserialization
- [shapely-msgpack](../shapely-msgpack): MessagePack deserialization
- [shapely-urlencoded](../shapely-urlencoded): URL-encoded form data deserialization

Additionally:

- [shapely-pretty](../shapely-pretty) is able to pretty-print Shapely types.
- [shapely-codegen](../shapely-codegen) is internal and generates some of the code of `shapely-core`

### Example usage

[shapely-json](../shapely-json/src/lib.rs) is the one that gets updated first — look at it.

### Funding

Thanks to Namespace for providing fast GitHub Actions workers:

<a href="https://namespace.so"><img src="https://github.com/shapely-rs/shapely/raw/main/static/namespace-d.svg" height="40"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/shapely-rs/shapely/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://github.com/shapely-rs/shapely/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.