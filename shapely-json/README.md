# shapely-json

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/shapely)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/shapely-json.svg)](https://crates.io/crates/shapely-json)
[![documentation](https://docs.rs/shapely-json/badge.svg)](https://docs.rs/shapely-json)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely-json.svg)](./LICENSE)

> [!IMPORTANT]
>
> There is no stable shapely API as of now (even though it's >1.0.0). The design
> is very much still being explored.
>
> Expect multiple major versions in the near future — (note left 2025-03-11)

A serialization and deserialization library for JSON using the shapely runtime reflection system.

## Features

- Simple JSON deserialization for any type implementing the `Shapely` trait
- Detailed error reporting with context
- Support for nested structs and scalar values

## Example

```rust
use shapely::Shapely;
use shapely_json::from_json;

#[derive(Debug, Shapely, PartialEq)]
struct Person {
    name: String,
    age: u64,
}

let json = r#"{"name": "Alice", "age": 30}"#;

let mut partial = Person::partial();
from_json(&mut partial, json).expect("Failed to parse JSON");

let person = partial.build::<Person>();
assert_eq!(person, Person { name: "Alice".to_string(), age: 30 });
```

### Funding

Thanks to Namespace for providing fast GitHub Actions workers:

<a href="https://namespace.so"><img src="./static/namespace-d.svg" height="40"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
