# facet-msgpack

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/facet)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/facet-msgpack.svg)](https://crates.io/crates/facet-msgpack)
[![documentation](https://docs.rs/facet-msgpack/badge.svg)](https://docs.rs/facet-msgpack)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/facet-msgpack.svg)](./LICENSE)

A crate for deserializing [MessagePack](https://msgpack.org/) data into Facet structures.

## Example

```rust
use facet::Facet;
use facet_msgpack::from_msgpack;

#[derive(Debug, Facet, PartialEq)]
struct User {
    id: u64,
    username: String,
}

// MessagePack binary data (equivalent to {"id": 42, "username": "user123"})
let msgpack_data = [
    0x82, 0xa2, 0x69, 0x64, 0x2a, 0xa8, 0x75, 0x73,
    0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0xa7, 0x75,
    0x73, 0x65, 0x72, 0x31, 0x32, 0x33
];

let mut partial = User::partial();
from_msgpack(&mut partial, &msgpack_data).expect("Failed to parse MessagePack data");

let user = partial.build::<User>();
assert_eq!(user, User { id: 42, username: "user123".to_string() });
```

### Funding

Thanks to Namespace for providing fast GitHub Actions workers:

<a href="https://namespace.so"><img src="./static/namespace-d.svg" height="40"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
