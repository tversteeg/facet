use std::{
    collections::HashMap,
    mem::{self, MaybeUninit},
};

use crate::{
    MapField, MapManipulator, MapShape, Scalar, Schema, Schematic, SetFieldOutcome, Shape,
};

macro_rules! impl_schematic_for_integer {
    ($type:ty, $scalar:expr) => {
        impl Schematic for $type {
            fn schema() -> Schema {
                Schema {
                    name: stringify!($type),
                    size: mem::size_of::<$type>(),
                    align: mem::align_of::<$type>(),
                    shape: Shape::Scalar($scalar),
                    display: Some(|addr: *const u8, f: &mut std::fmt::Formatter| unsafe {
                        write!(f, "{}", *(addr as *const $type))
                    }),
                    debug: Some(|addr: *const u8, f: &mut std::fmt::Formatter| unsafe {
                        write!(f, "{:?}", *(addr as *const $type))
                    }),
                    set_to_default: Some(|addr: *mut u8| unsafe {
                        *(addr as *mut $type) = 0;
                    }),
                }
            }
        }
    };
}

impl_schematic_for_integer!(u8, Scalar::U8);
impl_schematic_for_integer!(u16, Scalar::U16);
impl_schematic_for_integer!(u32, Scalar::U32);
impl_schematic_for_integer!(u64, Scalar::U64);
impl_schematic_for_integer!(i8, Scalar::I8);
impl_schematic_for_integer!(i16, Scalar::I16);
impl_schematic_for_integer!(i32, Scalar::I32);
impl_schematic_for_integer!(i64, Scalar::I64);

macro_rules! impl_schematic_for_float {
    ($type:ty, $scalar:expr) => {
        impl Schematic for $type {
            fn schema() -> Schema {
                Schema {
                    name: stringify!($type),
                    size: mem::size_of::<$type>(),
                    align: mem::align_of::<$type>(),
                    shape: Shape::Scalar($scalar),
                    display: Some(|addr: *const u8, f: &mut std::fmt::Formatter| unsafe {
                        write!(f, "{}", *(addr as *const $type))
                    }),
                    debug: Some(|addr: *const u8, f: &mut std::fmt::Formatter| unsafe {
                        write!(f, "{:?}", *(addr as *const $type))
                    }),
                    set_to_default: Some(|addr: *mut u8| unsafe {
                        *(addr as *mut $type) = 0.0;
                    }),
                }
            }
        }
    };
}

impl_schematic_for_float!(f32, Scalar::F32);
impl_schematic_for_float!(f64, Scalar::F64);

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
