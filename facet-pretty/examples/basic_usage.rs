use facet_derive::Facet;
use facet_trait as facet;
use facet_trait::Facet;

use facet_pretty::{FacetPretty, PrettyPrinter};

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

fn main() {
    let address = Address {
        street: "123 Main St".to_string(),
        city: "Wonderland".to_string(),
        country: "Imagination".to_string(),
    };

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        address,
    };

    println!("Default pretty-printing:");
    println!("{}", person.pretty());

    println!("\nCustomized pretty-printing:");
    let custom_printer = PrettyPrinter::new()
        .with_indent_size(4)
        .with_max_depth(3)
        .with_colors(true);

    println!("{}", person.pretty_with(custom_printer));

    println!("\nWithout colors:");
    let no_colors_printer = PrettyPrinter::new().with_colors(false);

    println!("{}", person.pretty_with(no_colors_printer));
}
