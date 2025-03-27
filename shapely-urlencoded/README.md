# shapely-urlencoded

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/shapely)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/shapely-urlencoded.svg)](https://crates.io/crates/shapely-urlencoded)
[![documentation](https://docs.rs/shapely-urlencoded/badge.svg)](https://docs.rs/shapely-urlencoded)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely-urlencoded.svg)](./LICENSE)

> [!IMPORTANT]
>
> There is no stable shapely API as of now (even though it's >1.0.0). The design
> is very much still being explored.
>
> Expect multiple major versions in the near future — (note left 2025-03-11)

A serialization and deserialization library for URL encoded form data using the shapely runtime reflection system.

## Features

- Simple URL encoded form data deserialization for any type implementing the `Shapely` trait
- Support for query string parsing
- Detailed error reporting with context
- Support for nested structs and scalar values

## Example

```rust
use shapely::Shapely;
use shapely_urlencoded::from_urlencoded;

#[derive(Debug, Shapely, PartialEq)]
struct SearchParams {
    query: String,
    page: u64,
}

let query_string = "query=rust+programming&page=2";

let mut partial = SearchParams::partial();
from_urlencoded(&mut partial, query_string).expect("Failed to parse URL encoded data");

let params = partial.build::<SearchParams>();
assert_eq!(params, SearchParams { query: "rust programming".to_string(), page: 2 });
```

### Funding

Thanks to Namespace for providing fast GitHub Actions workers:

<a href="https://namespace.so"><img src="./static/namespace-d.svg" height="40"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
