use shapely::Shapely;
use shapely_core::{Field, FieldFlags, Shape, ShapeDesc, Innards};
use shapely_pretty::{PrettyPrinter, ShapelyPretty};
use std::{alloc::Layout, fmt::Write};

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

#[test]
fn test_pretty_print() {
    // This is a simplified test that just ensures the code runs without panicking
    // In a real test, we would verify the output format
    
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
    
    // Test the PrettyPrinter directly
    let printer = PrettyPrinter::new();
    let output = printer.format(&person);
    
    // Ensure the output contains the field names
    assert!(output.contains("name"));
    assert!(output.contains("age"));
    assert!(output.contains("address"));
    assert!(output.contains("street"));
    assert!(output.contains("city"));
    assert!(output.contains("country"));
    
    // Test the ShapelyPretty trait
    let mut buffer = String::new();
    write!(buffer, "{}", person.pretty()).unwrap();
    
    // Ensure the output contains the field names
    assert!(buffer.contains("name"));
    assert!(buffer.contains("age"));
    assert!(buffer.contains("address"));
    assert!(buffer.contains("street"));
    assert!(buffer.contains("city"));
    assert!(buffer.contains("country"));
    
    // Test with custom printer settings
    let custom_printer = PrettyPrinter::new()
        .with_indent_size(4)
        .with_max_depth(3)
        .with_colors(false);
        
    let custom_output = custom_printer.format(&person);
    
    // Ensure the output contains the field names
    assert!(custom_output.contains("name"));
    assert!(custom_output.contains("age"));
    assert!(custom_output.contains("address"));
    
    // Test the ShapelyPretty trait with custom printer
    let mut custom_buffer = String::new();
    write!(custom_buffer, "{}", person.pretty_with(custom_printer)).unwrap();
    
    // Ensure the output contains the field names
    assert!(custom_buffer.contains("name"));
    assert!(custom_buffer.contains("age"));
    assert!(custom_buffer.contains("address"));
}

#[test]
fn test_sensitive_fields() {
    // Create a shape with a sensitive field manually
    let shape = Shape {
        name: |f| write!(f, "TestStruct"),
        typeid: std::any::TypeId::of::<String>(),
        layout: Layout::new::<String>(),
        innards: Innards::Struct {
            fields: &[
                Field {
                    name: "normal_field",
                    shape: ShapeDesc(String::shape),
                    offset: 0,
                    flags: FieldFlags::EMPTY,
                },
                Field {
                    name: "sensitive_field",
                    shape: ShapeDesc(String::shape),
                    offset: std::mem::size_of::<String>(),
                    flags: FieldFlags::SENSITIVE,
                },
            ],
        },
        set_to_default: None,
        drop_in_place: None,
    };

    // Test debug formatting
    let mut output = String::new();
    write!(output, "{:?}", shape).unwrap();

    // Verify output
    assert!(output.contains("normal_field"));
    assert!(output.contains("sensitive_field"));
    assert!(output.contains("[REDACTED]"));

    // Test debug formatting with alternate flag
    let mut output = String::new();
    write!(output, "{:#?}", shape).unwrap();

    // Verify output
    assert!(output.contains("normal_field"));
    assert!(output.contains("sensitive_field"));
    assert!(output.contains("[REDACTED]"));
}
