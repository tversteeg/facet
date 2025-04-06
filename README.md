# facet

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/facet)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/facet.svg)](https://crates.io/crates/facet)
[![documentation](https://docs.rs/facet/badge.svg)](https://docs.rs/facet)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/facet.svg)](./LICENSE)

A Rust reflection, introspection, serialization and deserialization framework with support for multiple formats including JSON, YAML, MessagePack, URL-encoded data, and more.

A single, lightweight derive macro (thanks to [unsynn](https://crates.io/crates/unsynn))

```rust
#[derive(Facet)]
struct Blah {
    foo: u32,
    bar: String,
}
```

...gives you the equivalent of `Debug`, `Serialize`, `Deserialize`, and more.

facet's approach is halfway between [serde](https://crates.io/crates/serde) and [bevy_reflect](https://crates.io/crates/bevy_reflect)

See the [facet README](./facet/README.md) for more info.

### Funding

Thanks to Namespace for providing fast GitHub Actions workers:

<a href="https://namespace.so"><img src="./static/namespace-d.svg" height="40"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
