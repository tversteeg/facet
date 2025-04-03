use owo_colors::OwoColorize;
use shapely::{Peek, PeekValue, Shapely};
use std::cmp::Ordering;
use std::fmt::Debug;

fn test_peek_values(name: &str, v1: &PeekValue, v2: &PeekValue) {
    eprintln!("{}", format!("=================== {}", name).yellow());

    // Debug
    eprintln!("Debug v1: {}", format!("{:?}", v1).green());
    eprintln!("Debug v2: {}", format!("{:?}", v2).green());

    // Equality
    match v1.eq(v2) {
        Some(result) => eprintln!("Equality: {}", format!("{:?}", result).green()),
        None => eprintln!("Equality: {}", "Not implemented".red()),
    }

    // Comparison
    match v1.cmp(v2) {
        Some(result) => eprintln!("Comparison: {}", format!("{:?}", result).green()),
        None => eprintln!("Comparison: {}", "Not implemented".red()),
    }

    // Default (just checking if it's implemented)
    let default_implemented = v1.shape.vtable.default_in_place.is_some();
    eprintln!(
        "Default implemented: {}",
        format!("{:?}", default_implemented).green()
    );

    eprintln!();
}

#[test]
fn test_primitive_types() {
    let i32_1 = 42;
    let i32_2 = 24;
    let peek1 = Peek::new(&i32_1);
    let peek2 = Peek::new(&i32_2);
    test_peek_values("i32", &peek1.as_value(), &peek2.as_value());

    let f64_1 = 3.14;
    let f64_2 = 2.71;
    let peek1 = Peek::new(&f64_1);
    let peek2 = Peek::new(&f64_2);
    test_peek_values("f64", &peek1.as_value(), &peek2.as_value());
}

#[test]
fn test_vec() {
    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2: Vec<i32> = vec![4, 5, 6];
    let peek1 = Peek::new(&vec1);
    let peek2 = Peek::new(&vec2);
    test_peek_values("Vec<i32>", &peek1.as_value(), &peek2.as_value());
}

#[test]
fn test_struct_without_traits() {
    #[derive(Shapely)]
    struct StructNoTraits {
        blah: i32,
    }
    let s1 = StructNoTraits { blah: 42 };
    let s2 = StructNoTraits { blah: 24 };
    let peek1 = Peek::new(&s1);
    let peek2 = Peek::new(&s2);
    test_peek_values("StructNoTraits", &peek1.as_value(), &peek2.as_value());
}

#[test]
fn test_struct_with_traits() {
    #[derive(Shapely, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct StructWithTraits {
        blah: i32,
    }
    let s1 = StructWithTraits { blah: 42 };
    let s2 = StructWithTraits { blah: 24 };
    let peek1 = Peek::new(&s1);
    let peek2 = Peek::new(&s2);
    test_peek_values("StructWithTraits", &peek1.as_value(), &peek2.as_value());
}

#[test]
fn test_tuple_struct() {
    #[derive(Shapely, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TupleStruct(i32, String);
    let t1 = TupleStruct(42, "Hello".to_string());
    let t2 = TupleStruct(24, "World".to_string());
    let peek1 = Peek::new(&t1);
    let peek2 = Peek::new(&t2);
    test_peek_values("TupleStruct", &peek1.as_value(), &peek2.as_value());
}

// Uncomment and adjust this test when enum support is added
// #[test]
// fn test_enum() {
//     #[derive(Shapely, Debug, PartialEq, Eq, PartialOrd, Ord)]
//     enum MyEnum {
//         Variant1,
//         Variant2(i32),
//         Variant3 { field: String },
//     }
//     let e1 = MyEnum::Variant2(42);
//     let e2 = MyEnum::Variant3 { field: "Hello".to_string() };
//     let peek1 = Peek::new(&e1);
//     let peek2 = Peek::new(&e2);
//     test_peek_values("MyEnum", &peek1.as_value(), &peek2.as_value());
// }
