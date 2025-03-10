use std::{
    collections::HashMap,
    mem::{self, MaybeUninit},
};

use crate::{
    MapField, MapManipulator, MapShape, Scalar, Schema, Schematic, SetFieldOutcome, Shape,
};

impl Schematic for u64 {
    fn schema() -> Schema {
        Schema {
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
        }
    }
}

impl Schematic for String {
    fn schema() -> Schema {
        Schema {
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
        }
    }
}

impl<V> Schematic for HashMap<String, V>
where
    V: Schematic + 'static,
{
    fn schema() -> Schema {
        struct HashMapManipulator<V>(std::marker::PhantomData<V>);

        impl<V> MapManipulator for HashMapManipulator<V> {
            unsafe fn set_field_raw(
                &self,
                map_addr: *mut u8,
                field: MapField,
                write_field: &mut dyn FnMut(*mut u8),
            ) -> SetFieldOutcome {
                unsafe {
                    let map = &mut *(map_addr as *mut HashMap<String, MaybeUninit<V>>);
                    let name = field.name;
                    let entry = map
                        // FIXME: we should avoid this `to_string` call, if the entry is already there.
                        .entry(name.to_string())
                        .or_insert_with(|| MaybeUninit::uninit());
                    // # Safety: write_field is supposed to fully initialize the field.
                    write_field(entry.as_mut_ptr() as *mut u8);
                    SetFieldOutcome::Accepted
                }
            }
        }
        Schema {
            name: "HashMap<String, V>",
            size: mem::size_of::<HashMap<String, V>>(),
            align: mem::align_of::<HashMap<String, V>>(),
            shape: Shape::Map(MapShape {
                fields: &[],
                open_ended: true,
                manipulator: &HashMapManipulator(std::marker::PhantomData::<V>),
            }),
            display: None,
            debug: None,
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut HashMap<String, V>) = HashMap::new();
            }),
        }
    }
}
