use ctor::ctor;

#[ctor]
fn init_backtrace() {
    color_backtrace::install();
}

use std::{cmp::Ordering, collections::HashSet, fmt::Debug};

use owo_colors::{OwoColorize, Style};
use shapely::{Peek, Poke, Shapely};

fn test_peek_pair<T>(val1: T, val2: T, expected_facts: HashSet<Fact>)
where
    T: Shapely + 'static,
{
    let mut facts: HashSet<Fact> = HashSet::new();
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

    let l = Peek::new(&val1);
    let r = Peek::new(&val2);

    let remarkable = Style::new().blue();

    // Format display representation
    if l.as_value().shape().vtable.display.is_some() {
        facts.insert(Fact::HasDisplay);
        let display_str = format!("{} vs {}", l.style(remarkable), r.style(remarkable));
        eprintln!("Display:   {}", display_str);
    }

    // Format debug representation
    if l.as_value().shape().vtable.debug.is_some() {
        facts.insert(Fact::HasDebug);
        let debug_str = format!("{:?} vs {:?}", l.style(remarkable), r.style(remarkable));
        eprintln!("Debug:     {}", debug_str);
    }

    // Test equality
    if let Some(eq_result) = l.as_value().eq(&r.as_value()) {
        facts.insert(Fact::HasEqualAnd { l_eq_r: eq_result });
        let eq_str = format!(
            "{:?} {} {:?}",
            l.style(remarkable),
            if eq_result { "==" } else { "!=" }.yellow(),
            r.style(remarkable),
        );
        eprintln!("Equality:  {}", eq_str);
    }

    // Test ordering
    if let Some(cmp_result) = l.as_value().cmp(&r.as_value()) {
        facts.insert(Fact::HasOrdAnd {
            l_ord_r: cmp_result,
        });
        let cmp_symbol = match cmp_result {
            Ordering::Less => "<",
            Ordering::Equal => "==",
            Ordering::Greater => ">",
        };
        let cmp_str = format!(
            "{:?} {} {:?}",
            l.style(remarkable),
            cmp_symbol.yellow(),
            r.style(remarkable),
        );
        eprintln!("Ordering:  {}", cmp_str);
    }

    // Test default_in_place
    let (poke, _guard) = Poke::alloc::<T>();
    let poke_value = poke.into_value();
    if let Ok(value) = poke_value.default_in_place() {
        facts.insert(Fact::HasDefault);
        let peek = unsafe { Peek::unchecked_new(value.as_const(), T::SHAPE) };
        eprintln!("Default:   {}", format!("{:?}", peek).style(remarkable));
    }

    let expected_minus_actual: HashSet<_> = expected_facts.difference(&facts).collect();
    let actual_minus_expected: HashSet<_> = facts.difference(&expected_facts).collect();

    assert!(
        expected_facts == facts,
        "{}\nExpected but not found: {:?}\n{}\nFound but not expected: {:?}",
        "Facts mismatch:".red().bold(),
        expected_minus_actual.yellow(),
        "Found but not expected:".red().bold(),
        actual_minus_expected.yellow()
    );
}

#[test]
fn test_number_traits() {
    // i32 implements Debug, PartialEq, and Ord
    test_peek_pair(
        42,
        24,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .default()
            .build(),
    );

    // bool implements Debug, PartialEq, and Ord
    test_peek_pair(
        true,
        false,
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .default()
            .build(),
    );
}

#[test]
fn test_string_traits() {
    // String implements Debug, PartialEq, and Ord
    test_peek_pair(
        String::from("hello"),
        String::from("world"),
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .default()
            .build(),
    );

    // &str implements Debug, PartialEq, and Ord
    test_peek_pair(
        "hello",
        "world",
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .build(),
    );

    // Cow<'a, str> implements Debug, PartialEq, and Ord
    use std::borrow::Cow;
    test_peek_pair(
        Cow::Borrowed("hello"),
        Cow::Borrowed("world"),
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .build(),
    );
    test_peek_pair(
        Cow::Owned("hello".to_string()),
        Cow::Owned("world".to_string()),
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .build(),
    );
    test_peek_pair(
        Cow::Borrowed("same"),
        Cow::Owned("same".to_string()),
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(true)
            .ord_and(Ordering::Equal)
            .build(),
    );
}

#[test]
fn test_other_types() {
    // Vec implements Debug and PartialEq but not Ord
    test_peek_pair(
        vec![1, 2, 3],
        vec![1, 2, 3],
        FactBuilder::new().debug().equal_and(true).build(),
    );
}

#[test]
fn test_slice_traits() {
    // &[i32] implements Debug, PartialEq, and Ord
    test_peek_pair(
        &[1, 2, 3][..],
        &[4, 5, 6][..],
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .build(),
    );

    // &[&str] implements Debug, PartialEq, and Ord
    test_peek_pair(
        &["hello", "world"][..],
        &["foo", "bar"][..],
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .build(),
    );
}

#[test]
fn test_array_traits() {
    // [i32; 1] implements Debug, PartialEq, and Ord
    test_peek_pair(
        [42],
        [24],
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .default()
            .build(),
    );

    // [&str; 1] implements Debug, PartialEq, and Ord
    test_peek_pair(
        ["hello"],
        ["world"],
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .build(),
    );
}

#[test]
fn test_vecs() {
    // Vec<i32> implements Debug, PartialEq, but not Ord
    test_peek_pair(
        vec![1, 2, 3],
        vec![4, 5, 6],
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .default()
            .build(),
    );

    // Vec<String> implements Debug, PartialEq, but not Ord
    test_peek_pair(
        vec!["hello".to_string(), "world".to_string()],
        vec!["foo".to_string(), "bar".to_string()],
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .default()
            .build(),
    );

    // Two pairs of equal Vecs
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];
    test_peek_pair(
        vec1,
        vec2,
        FactBuilder::new().debug().equal_and(true).default().build(),
    );

    let vec3 = vec!["hello".to_string(), "world".to_string()];
    let vec4 = vec!["hello".to_string(), "world".to_string()];
    test_peek_pair(
        vec3,
        vec4,
        FactBuilder::new().debug().equal_and(true).default().build(),
    );
}

#[test]
fn test_hashmaps() {
    use std::collections::HashMap;

    // HashMap<String, i32> implements Debug, PartialEq, but not Ord
    let mut map1 = HashMap::new();
    map1.insert("key1".to_string(), 42);
    map1.insert("key2".to_string(), 24);

    let mut map2 = HashMap::new();
    map2.insert("key3".to_string(), 100);
    map2.insert("key4".to_string(), 200);

    test_peek_pair(
        map1,
        map2,
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .default()
            .build(),
    );

    // Two pairs of equal HashMaps
    let mut map3 = HashMap::new();
    map3.insert("key1".to_string(), 10);
    map3.insert("key2".to_string(), 20);

    let mut map4 = HashMap::new();
    map4.insert("key1".to_string(), 10);
    map4.insert("key2".to_string(), 20);

    test_peek_pair(
        map3,
        map4,
        FactBuilder::new().debug().equal_and(true).default().build(),
    );
}

#[test]
fn test_custom_structs() {
    // Struct with no trait implementations
    #[derive(Shapely)]
    struct StructNoTraits {
        value: i32,
    }
    test_peek_pair(
        StructNoTraits { value: 42 },
        StructNoTraits { value: 24 },
        FactBuilder::new().build(),
    );

    // Struct with Debug only
    #[derive(Shapely, Debug)]
    struct StructDebug {
        value: i32,
    }
    test_peek_pair(
        StructDebug { value: 42 },
        StructDebug { value: 24 },
        FactBuilder::new().debug().build(),
    );

    // Struct with Debug and PartialEq
    #[derive(Shapely, Debug, PartialEq)]
    struct StructDebugEq {
        value: i32,
    }
    test_peek_pair(
        StructDebugEq { value: 42 },
        StructDebugEq { value: 24 },
        FactBuilder::new().debug().equal_and(false).build(),
    );

    // Struct with all traits
    #[derive(Shapely, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct StructAll {
        value: i32,
    }
    test_peek_pair(
        StructAll { value: 42 },
        StructAll { value: 24 },
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .build(),
    );
    test_peek_pair(
        StructAll { value: 10 },
        StructAll { value: 90 },
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .build(),
    );
    test_peek_pair(
        StructAll { value: 69 },
        StructAll { value: 69 },
        FactBuilder::new()
            .debug()
            .equal_and(true)
            .ord_and(Ordering::Equal)
            .build(),
    );
}

#[test]
fn test_tuple_structs() {
    // Tuple struct with no trait implementations
    #[derive(Shapely)]
    struct TupleNoTraits(i32, String);
    test_peek_pair(
        TupleNoTraits(42, "Hello".to_string()),
        TupleNoTraits(24, "World".to_string()),
        FactBuilder::new().build(),
    );

    // Tuple struct with Debug only
    #[derive(Shapely, Debug)]
    struct TupleDebug(i32, String);
    test_peek_pair(
        TupleDebug(42, "Hello".to_string()),
        TupleDebug(24, "World".to_string()),
        FactBuilder::new().debug().build(),
    );

    // Tuple struct with EQ only
    #[derive(Shapely, PartialEq)]
    struct TupleEq(i32, String);
    test_peek_pair(
        TupleEq(42, "Hello".to_string()),
        TupleEq(24, "World".to_string()),
        FactBuilder::new().equal_and(false).build(),
    );

    // Tuple struct with all traits
    #[derive(Shapely, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TupleAll(i32, String);
    test_peek_pair(
        TupleAll(42, "Hello".to_string()),
        TupleAll(24, "World".to_string()),
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .build(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fact {
    HasDebug,
    HasDisplay,
    HasEqualAnd { l_eq_r: bool },
    HasOrdAnd { l_ord_r: Ordering },
    HasDefault,
}

impl Fact {
    pub fn builder() -> FactBuilder {
        FactBuilder::new()
    }
}

#[derive(Default)]
pub struct FactBuilder {
    has_debug: bool,
    has_display: bool,
    has_equal_and: Option<bool>,
    has_ord_and: Option<Ordering>,
    has_default: bool,
}

impl FactBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn debug(mut self) -> Self {
        self.has_debug = true;
        self
    }

    pub fn display(mut self) -> Self {
        self.has_display = true;
        self
    }

    pub fn equal_and(mut self, l_eq_r: bool) -> Self {
        self.has_equal_and = Some(l_eq_r);
        self
    }

    pub fn ord_and(mut self, l_ord_r: Ordering) -> Self {
        self.has_ord_and = Some(l_ord_r);
        self
    }

    pub fn default(mut self) -> Self {
        self.has_default = true;
        self
    }

    pub fn build(self) -> HashSet<Fact> {
        let mut facts = HashSet::new();
        if self.has_debug {
            facts.insert(Fact::HasDebug);
        }
        if self.has_display {
            facts.insert(Fact::HasDisplay);
        }
        if let Some(l_eq_r) = self.has_equal_and {
            facts.insert(Fact::HasEqualAnd { l_eq_r });
        }
        if let Some(l_ord_r) = self.has_ord_and {
            facts.insert(Fact::HasOrdAnd { l_ord_r });
        }
        if self.has_default {
            facts.insert(Fact::HasDefault);
        }
        facts
    }
}
