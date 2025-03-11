# shapely-msgpack

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/shapely)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/shapely-msgpack.svg)](https://crates.io/crates/shapely-msgpack)
[![documentation](https://docs.rs/shapely-msgpack/badge.svg)](https://docs.rs/shapely-msgpack)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely-msgpack.svg)](./LICENSE)

> [!IMPORTANT]
>
> There is no stable shapely API as of now (even though it's >1.0.0). The design
> is very much still being explored.
>
> Expect multiple major versions in the near future — (note left 2025-03-11)

A crate for deserializing [MessagePack](https://msgpack.org/) data into Shapely structures.

## Example

```rust
use shapely::Shapely;
use shapely_msgpack::from_msgpack;

#[derive(Debug, Shapely, PartialEq)]
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

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.