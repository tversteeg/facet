use std::fmt::Debug;

use owo_colors::{OwoColorize, Style};
use shapely::{Peek, Poke, Shapely};

fn test_peek_pair<T>(val1: T, val2: T)
where
    T: Shapely + 'static,
{
    let name = format!("{}", T::SHAPE);

    eprint!("{}", format!("== {name}: ").yellow());
    let value_vtable = T::SHAPE.vtable;
    let traits = [
        ("Debug", value_vtable.debug.is_some()),
        ("Display", value_vtable.display.is_some()),
        ("Default", value_vtable.default_in_place.is_some()),
        ("Eq", value_vtable.eq.is_some()),
        ("Ord", value_vtable.cmp.is_some()),
    ];
    let trait_str = traits
        .iter()
        .filter_map(|(name, has_impl)| {
            if *has_impl {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" + ");
    eprintln!("{} {}", trait_str, "======".yellow());

    let peek1 = Peek::new(&val1);
    let peek2 = Peek::new(&val2);

    let good = Style::new().green();

    // Format display representation
    if peek1.as_value().shape().vtable.display.is_some() {
        let display_str = format!("{} vs {}", peek1.style(good), peek2.style(good));
        eprintln!("Display:   {}", display_str);
    }

    // Format debug representation
    if peek1.as_value().shape().vtable.debug.is_some() {
        let debug_str = format!("{:?} vs {:?}", peek1.style(good), peek2.style(good));
        eprintln!("Debug:     {}", debug_str);
    }

    // Test equality
    if let Some(eq_result) = peek1.as_value().eq(&peek2.as_value()) {
        let eq_str = format!(
            "{:?} {} {:?} is {:?}",
            peek1,
            "==".style(good),
            peek2,
            eq_result
        );
        eprintln!("Equality:  {}", eq_str);
    }

    // Test ordering
    if let Some(cmp_result) = peek1.as_value().cmp(&peek2.as_value()) {
        let cmp_str = format!(
            "{:?} {} {:?} is {:?}",
            peek1,
            "cmp".style(good),
            peek2,
            cmp_result
        );
        eprintln!("Ordering:  {}", cmp_str);
    }

    // Test default_in_place
    let (poke, _guard) = Poke::alloc::<T>();
    let poke_value = poke.into_value();
    if let Ok(value) = poke_value.default_in_place() {
        let peek = unsafe { Peek::unchecked_new(value.as_const(), T::SHAPE) };
        eprintln!("Default:   {}", format!("{:?}", peek).style(good));
    }
}

#[test]
fn test_primitive_types() {
    // i32 implements Debug, PartialEq, and Ord
    test_peek_pair(42, 24);

    // Vec implements Debug and PartialEq but not Ord
    test_peek_pair(vec![1, 2, 3], vec![1, 2, 3]);

    // String implements Debug, PartialEq, and Ord
    test_peek_pair(String::from("hello"), String::from("world"));

    // bool implements Debug, PartialEq, and Ord
    test_peek_pair(true, false);

    // &str implements Debug, PartialEq, and Ord
    test_peek_pair("hello", "world");

    // Cow<'a, str> implements Debug, PartialEq, and Ord
    use std::borrow::Cow;
    test_peek_pair(Cow::Borrowed("hello"), Cow::Borrowed("world"));
    test_peek_pair(
        Cow::Owned("hello".to_string()),
        Cow::Owned("world".to_string()),
    );
    test_peek_pair(Cow::Borrowed("same"), Cow::Owned("same".to_string()));
}

#[test]
fn test_multis() {
    // &[i32] implements Debug, PartialEq, and Ord
    test_peek_pair(&[1, 2, 3][..], &[4, 5, 6][..]);

    // &[&str] implements Debug, PartialEq, and Ord
    test_peek_pair(&["hello", "world"][..], &["foo", "bar"][..]);
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
