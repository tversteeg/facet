use std::fmt::Debug;

use owo_colors::{OwoColorize, Style};
use shapely::{Peek, Poke, Shapely};

fn test_peek_pair<T>(val1: T, val2: T)
where
    T: Shapely + 'static,
{
    let name = format!("{}", T::SHAPE);

    eprintln!(
        "{}",
        format!("== {name} ==========================").yellow()
    );
    let peek1 = Peek::new(&val1);
    let peek2 = Peek::new(&val2);

    let good = Style::new().green();
    let bad = Style::new().red();

    // Format debug representation
    let style1 = if peek1.as_value().shape().vtable.debug.is_some() {
        good
    } else {
        bad
    };
    let style2 = if peek2.as_value().shape().vtable.debug.is_some() {
        good
    } else {
        bad
    };
    let debug_str = format!("{:?} vs {:?}", peek1.style(style1), peek2.style(style2));
    eprintln!("Debug:     {}", debug_str);

    // Test equality
    let eq_result = peek1.as_value().eq(&peek2.as_value());
    let style = if eq_result.is_some() { good } else { bad };
    let eq_str = match eq_result {
        Some(result) => format!(
            "{:?} {} {:?} is {:?}",
            peek1,
            "==".style(style),
            peek2,
            result
        ),
        None => "unsupported!".style(bad).to_string(),
    };
    eprintln!("Equality:  {}", eq_str);

    // Test ordering
    let cmp_result = peek1.as_value().cmp(&peek2.as_value());
    let style = if cmp_result.is_some() { good } else { bad };
    let cmp_str = match cmp_result {
        Some(result) => format!(
            "{:?} {} {:?} is {:?}",
            peek1,
            "cmp".style(style),
            peek2,
            result
        ),
        None => "unsupported!".style(bad).to_string(),
    };
    eprintln!("Ordering:  {}", cmp_str);

    // Test default_in_place
    let (poke, guard) = Poke::alloc::<T>();
    match poke {
        Poke::Scalar(scalar) => {
            if let Ok(value) = scalar.default_in_place() {
                let peek = unsafe { Peek::unchecked_new(value.as_const(), T::SHAPE) };
                eprintln!("Default in place gave us: {:?}", peek.blue());
                drop(guard);
            }
        }
        _ => {
            // meh
        }
    }
}

#[test]
fn test_primitive_types() {
    // i32 implements Debug, PartialEq, and Ord
    test_peek_pair(42, 24);

    // Vec implements Debug and PartialEq but not Ord
    test_peek_pair(vec![1, 2, 3], vec![1, 2, 3]);
}

#[test]
fn test_custom_structs() {
    // Struct with no trait implementations
    #[derive(Shapely)]
    struct StructNoTraits {
        value: i32,
    }
    test_peek_pair(StructNoTraits { value: 42 }, StructNoTraits { value: 24 });

    // Struct with Debug only
    #[derive(Shapely, Debug)]
    struct StructDebug {
        value: i32,
    }
    test_peek_pair(StructDebug { value: 42 }, StructDebug { value: 24 });

    // Struct with Debug and PartialEq
    #[derive(Shapely, Debug, PartialEq)]
    struct StructDebugEq {
        value: i32,
    }
    test_peek_pair(StructDebugEq { value: 42 }, StructDebugEq { value: 24 });

    // Struct with all traits
    #[derive(Shapely, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct StructAll {
        value: i32,
    }
    test_peek_pair(StructAll { value: 42 }, StructAll { value: 24 });
    test_peek_pair(StructAll { value: 10 }, StructAll { value: 90 });
    test_peek_pair(StructAll { value: 69 }, StructAll { value: 69 });
}

#[test]
fn test_tuple_structs() {
    // Tuple struct with no trait implementations
    #[derive(Shapely)]
    struct TupleNoTraits(i32, String);
    test_peek_pair(
        TupleNoTraits(42, "Hello".to_string()),
        TupleNoTraits(24, "World".to_string()),
    );

    // Tuple struct with Debug only
    #[derive(Shapely, Debug)]
    struct TupleDebug(i32, String);
    test_peek_pair(
        TupleDebug(42, "Hello".to_string()),
        TupleDebug(24, "World".to_string()),
    );

    // Tuple struct with EQ only
    #[derive(Shapely, PartialEq)]
    struct TupleEq(i32, String);
    test_peek_pair(
        TupleEq(42, "Hello".to_string()),
        TupleEq(24, "World".to_string()),
    );

    // Tuple struct with all traits
    #[derive(Shapely, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TupleAll(i32, String);
    test_peek_pair(
        TupleAll(42, "Hello".to_string()),
        TupleAll(24, "World".to_string()),
    );
}

// Commented out enum tests for now as they may need special handling
/*
#[test]
fn test_enums() {
    #[derive(Shapely, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum TestEnum {
        Variant1,
        Variant2(i32),
        Variant3 { field: String },
    }

    test_peek_pair(
        "Enum-Unit",
        TestEnum::Variant1,
        TestEnum::Variant1,
        true,
        true,
        true,
    );

    test_peek_pair(
        "Enum-Tuple",
        TestEnum::Variant2(42),
        TestEnum::Variant2(24),
        true,
        true,
        true,
    );

    test_peek_pair(
        "Enum-Struct",
        TestEnum::Variant3 {
            field: "Hello".to_string(),
        },
        TestEnum::Variant3 {
            field: "World".to_string(),
        },
        true,
        true,
        true,
    );
}
*/
