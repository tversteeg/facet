# facet-yaml

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/facet)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/facet-yaml.svg)](https://crates.io/crates/facet-yaml)
[![documentation](https://docs.rs/facet-yaml/badge.svg)](https://docs.rs/facet-yaml)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/facet-yaml.svg)](./LICENSE)

[YAML](https://yaml.org/) serialization and deserialization for Shapely types.

## Example

```rust
use facet::Shapely;
use facet_yaml::from_yaml;

#[derive(Debug, Shapely, PartialEq)]
struct Config {
    name: String,
    version: u64,
}

let yaml = r#"
name: MyApp
version: 1
"#;

let mut partial = Config::partial();
from_yaml(&mut partial, yaml).expect("Failed to parse YAML");

let config = partial.build::<Config>();
assert_eq!(config, Config { name: "MyApp".to_string(), version: 1 });
```

### Funding

Thanks to Namespace for providing fast GitHub Actions workers:

<a href="https://namespace.so"><img src="./static/namespace-d.svg" height="40"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
