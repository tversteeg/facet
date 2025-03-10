use std::{collections::HashMap, mem};

use crate::{MapManipulator, Scalar, Schema, Schematic, Shape};

impl Schematic for u64 {
    fn schema() -> &'static Schema {
        static SCHEMA: Schema = Schema {
            name: "u64",
            size: mem::size_of::<u64>(),
            align: mem::align_of::<u64>(),
            shape: Shape::Scalar(Scalar::U64),
            display: Some(|addr: *const u8, f: &mut std::fmt::Formatter| unsafe {
                write!(f, "{}", *(addr as *const u64))
            }),
            debug: Some(|addr: *const u8, f: &mut std::fmt::Formatter| unsafe {
                write!(f, "{:?}", *(addr as *const u64))
            }),
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut u64) = 0;
            }),
        };
        &SCHEMA
    }
}

impl Schematic for String {
    fn schema() -> &'static Schema {
        static SCHEMA: Schema = Schema {
            name: "String",
            size: mem::size_of::<String>(),
            align: mem::align_of::<String>(),
            shape: Shape::Scalar(Scalar::String),
            display: Some(|addr: *const u8, f: &mut std::fmt::Formatter| unsafe {
                write!(f, "{}", *(addr as *const String))
            }),
            debug: Some(|addr: *const u8, f: &mut std::fmt::Formatter| unsafe {
                write!(f, "{:?}", *(addr as *const String))
            }),
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut String) = String::new();
            }),
        };
        &SCHEMA
    }
}

impl<V> Schematic for HashMap<String, V>
where
    V: Schematic,
{
    fn schema() -> &'static Schema {
        static SCHEMA: Schema = Schema {
            name: "HashMap<String, V>",
            // all good, values are heap-allocated
            size: mem::size_of::<HashMap<String, ()>>(),
            align: mem::align_of::<HashMap<String, ()>>(),
            shape: Shape::Map(MapShape {
                fields: &[],
                open_ended: true,
                manipulator: &HashMapManipulator,
            }),
            display: Some(|addr: *const u8, f: &mut std::fmt::Formatter| unsafe {
                write!(f, "{}", *(addr as *const HashMap<String, V>))
            }),
            debug: Some(|addr: *const u8, f: &mut std::fmt::Formatter| unsafe {
                write!(f, "{:?}", *(addr as *const HashMap<String, V>))
            }),
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut HashMap<String, V>) = HashMap::new();
            }),
        };

        struct HashMapManipulator;

        impl MapManipulator for HashMapManipulator {
            unsafe fn set_field_raw(
                &self,
                _map_addr: *mut u8,
                _field: &MapField,
                _on_addr: &mut dyn FnMut(*mut u8),
            ) {
                // Implementation left empty as HashMap doesn't have fixed fields
            }
        }
        &SCHEMA
    }
}
