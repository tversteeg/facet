# shapely-core

[![crates.io](https://img.shields.io/crates/v/shapely-core.svg)](https://crates.io/crates/shapely-core)
[![documentation](https://docs.rs/shapely-core/badge.svg)](https://docs.rs/shapely-core)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely-core.svg)](./LICENSE)

This is the core crate for the shapely ecosystem. It provides the fundamental
types and traits used by other crates in the shapely family.

Note that the main documentation for the shapely project can be found in the
[shapely crate](https://crates.io/crates/shapely). This core crate serves as a
common dependency for:

  * The derived proc macro crate [`shapely-derive`]  (https://crates.io/crates/shapely-derive)
  * Serializers and deserializers (e.g. [`shapely-json`]  (https://crates.io/crates/shapely-json))
  * Any other crates in the shapely ecosystem

If you're building tools or libraries that interact with shapely's core
functionality, you should depend directly on `shapely-core` rather than the main
`shapely` crate.

For more detailed information and usage examples, please refer to the [shapely crate documentation](https://docs.rs/shapely).
