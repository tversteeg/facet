# facet

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/facet)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/facet.svg)](https://crates.io/crates/facet)
[![documentation](https://docs.rs/facet/badge.svg)](https://docs.rs/facet)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/facet.svg)](./LICENSE)

facet provides runtime reflection for Rust.

Any type that implements `Facet` trait returns a `Shape`, which describes:

  * The memory layout of the type
  * Its definition: struct fields, underlying type for newtypes, etc.
  * How to drop it in place

The `Poke` type is able to allocate (or work from a `&mut MaybeUninit<T>`)
any Facet type, and gradually initialize its fields — until the fully-built
value is moved out of the partial.

The `Peek` type helps perform read operations on any Facet type.

It comes with a derive macro that uses [unsynn](https://crates.io/crates/unsynn)
for speed of compilation.

## Ecosystem

The main `facet` crate re-exports symbols from:

- [facet-trait](../facet-trait), which defines the main `Facet` trait and implements it for foreign types (mostly `libstd`)
- [facet-types](../facet-types), which defines the `Shape` struct, along with various vtables and the whole `Def` tree
- [facet-opaque](../facet-opaque), which provides helpers around type-erased pointers like
  `OpaqueUninit`, `OpaqueConst`, `Opaque`
- [facet-derive](../facet-derive), which implements the `Facet` derive attribute as a fast/light proc macro powered by [unsynn](https://docs.rs/unsynn)
- [facet-spez](../facet-spez), which implements an autoderef specialization trick needed for `facet-derive`
- [facet-peek](../facet-peek), which allows reading arbitrary `Facet` types
- [facet-poke](../facet-poke), which allows building/altering arbitrary `Facet` types

facet supports deserialization from multiple data formats through dedicated crates:

- [facet-json](../facet-json): JSON deserialization
- [facet-yaml](../facet-yaml): YAML deserialization
- [facet-msgpack](../facet-msgpack): MessagePack deserialization
- [facet-urlencoded](../facet-urlencoded): URL-encoded form data deserialization

Additionally:

- [facet-pretty](../facet-pretty) is able to pretty-print Facet types.
- [facet-codegen](../facet-codegen) is internal and generates some of the code of `facet-core`

### Example usage

[facet-json](../facet-json/src/lib.rs) is the one that gets updated first — look at it.

### Funding

Thanks to Zed for sponsoring this project:

<a href="https://zed.dev"><svg width="96" height="96" viewBox="0 0 96 96" fill="none" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M9 6C7.34315 6 6 7.34315 6 9V75H0V9C0 4.02944 4.02944 0 9 0H89.3787C93.3878 0 95.3955 4.84715 92.5607 7.68198L43.0551 57.1875H57V51H63V58.6875C63 61.1728 60.9853 63.1875 58.5 63.1875H37.0551L26.7426 73.5H73.5V36H79.5V73.5C79.5 76.8137 76.8137 79.5 73.5 79.5H20.7426L10.2426 90H87C88.6569 90 90 88.6569 90 87V21H96V87C96 91.9706 91.9706 96 87 96H6.62132C2.61224 96 0.604504 91.1529 3.43934 88.318L52.7574 39H39V45H33V37.5C33 35.0147 35.0147 33 37.5 33H58.7574L69.2574 22.5H22.5V60H16.5V22.5C16.5 19.1863 19.1863 16.5 22.5 16.5H75.2574L85.7574 6H9Z" fill="white"/></svg></a>

Thanks to Namespace for providing fast GitHub Actions workers:

<a href=\"https://namespace.so\"><img src=\"https://github.com/facet-rs/facet/raw/main/static/namespace-d.svg\" height=\"40\"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/facet-rs/facet/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://github.com/facet-rs/facet/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.