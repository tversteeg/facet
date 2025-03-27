# shapely-core

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/shapely)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/shapely-core.svg)](https://crates.io/crates/shapely-core)
[![documentation](https://docs.rs/shapely-core/badge.svg)](https://docs.rs/shapely-core)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely-core.svg)](./LICENSE)

> [!IMPORTANT]
>
> There is no stable shapely API as of now (even though it's >1.0.0). The design
> is very much still being explored.
>
> Expect multiple major versions in the near future — (note left 2025-03-11)

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

### Funding

Thanks to Namespace for providing fast GitHub Actions workers:

<a href="https://namespace.so"><img src="./static/namespace-d.svg" height="40"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
