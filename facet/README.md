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

Thanks to Zed for sponsoring this project, and Namespace for providing fast GitHub Actions workers.

<a href="https://zed.dev">
    <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/facet-rs/facet/raw/main/static/sponsors-v1/zed-dark.svg">
    <img src="https://github.com/facet-rs/facet/raw/main/static/sponsors-v1/zed-light.svg" height="40" alt="Zed">
    </picture>
</a> <a href="https://namespace.so">
    <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/facet-rs/facet/raw/main/static/sponsors-v1/namespace-dark.svg">
    <img src="https://github.com/facet-rs/facet/raw/main/static/sponsors-v1/namespace-light.svg" height="40" alt="Namespace">
    </picture>
</a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/facet-rs/facet/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://github.com/facet-rs/facet/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.