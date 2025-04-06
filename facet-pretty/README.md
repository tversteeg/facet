# facet-pretty

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/facet-rs/facet)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/bearcove/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/facet-pretty.svg)](https://crates.io/crates/facet-pretty)
[![documentation](https://docs.rs/facet-pretty/badge.svg)](https://docs.rs/facet-pretty)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/facet-pretty.svg)](./LICENSE)

A pretty-printing library for types implementing the `Facet` trait, providing colorful and well-formatted output.

## Features

- Pretty-print any type implementing the `Facet` trait
- Colorful output with ANSI escape codes
- Unique colors for different scalar types based on their Shape hash
- Customizable indentation and formatting
- Dimmed colors for punctuation
- Simple abstraction over ANSI escape codes

## Example

```rust
use facet::Facet;
use facet_pretty::PrettyPrinter;

#[derive(Debug, Facet)]
struct Person {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Debug, Facet)]
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
