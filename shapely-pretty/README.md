# shapely-pretty

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/bearcove/shapely)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/bearcove/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/shapely-pretty.svg)](https://crates.io/crates/shapely-pretty)
[![documentation](https://docs.rs/shapely-pretty/badge.svg)](https://docs.rs/shapely-pretty)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely-pretty.svg)](./LICENSE)

> [!IMPORTANT]
>
> There is no stable shapely API as of now (even though it's >1.0.0). The design
> is very much still being explored.
>
> Expect multiple major versions in the near future — (note left 2025-03-11)

A pretty-printing library for types implementing the `Shapely` trait, providing colorful and well-formatted output.

## Features

- Pretty-print any type implementing the `Shapely` trait
- Colorful output with ANSI escape codes
- Unique colors for different scalar types based on their Shape hash
- Customizable indentation and formatting
- Dimmed colors for punctuation
- Simple abstraction over ANSI escape codes

## Example

```rust
use shapely::Shapely;
use shapely_pretty::PrettyPrinter;

#[derive(Debug, Shapely)]
struct Person {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Debug, Shapely)]
struct Address {
    street: String,
    city: String,
    country: String,
}

let person = Person {
    name: "Alice".to_string(),
    age: 30,
    address: Address {
        street: "123 Main St".to_string(),
        city: "Wonderland".to_string(),
        country: "Imagination".to_string(),
    },
};

// Print with default settings
PrettyPrinter::new().print(&person);

// Customize the printer
PrettyPrinter::new()
    .with_indent_size(4)
    .with_max_depth(3)
    .print(&person);
```

### Funding

Thanks to Namespace for providing fast GitHub Actions workers:

<a href="https://namespace.so"><img src="./static/namespace-d.svg" height="40"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
