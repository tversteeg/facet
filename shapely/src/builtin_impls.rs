use std::{
    collections::HashMap,
    mem::{self, MaybeUninit},
};

use crate::*;

macro_rules! impl_shapely_for_integer {
    ($type:ty, $scalar:expr) => {
        impl Shapely for $type {
            fn shape() -> Shape {
                Shape {
                    name: stringify!($type),
                    size: mem::size_of::<$type>(),
                    align: mem::align_of::<$type>(),
                    shape: ShapeKind::Scalar($scalar),
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

impl_shapely_for_integer!(u8, Scalar::U8);
impl_shapely_for_integer!(u16, Scalar::U16);
impl_shapely_for_integer!(u32, Scalar::U32);
impl_shapely_for_integer!(u64, Scalar::U64);
impl_shapely_for_integer!(i8, Scalar::I8);
impl_shapely_for_integer!(i16, Scalar::I16);
impl_shapely_for_integer!(i32, Scalar::I32);
impl_shapely_for_integer!(i64, Scalar::I64);

macro_rules! impl_schematic_for_float {
    ($type:ty, $scalar:expr) => {
        impl Shapely for $type {
            fn shape() -> Shape {
                Shape {
                    name: stringify!($type),
                    size: mem::size_of::<$type>(),
                    align: mem::align_of::<$type>(),
                    shape: ShapeKind::Scalar($scalar),
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

impl Shapely for String {
    fn shape() -> Shape {
        Shape {
            name: "String",
            size: mem::size_of::<String>(),
            align: mem::align_of::<String>(),
            shape: ShapeKind::Scalar(Scalar::String),
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

impl<V> Shapely for HashMap<String, V>
where
    V: Shapely + Send + Sync + 'static,
{
    fn shape() -> Shape {
        struct HashMapManipulator<V>(std::marker::PhantomData<V>);

        impl<V> MapManipulator for HashMapManipulator<V>
        where
            V: Shapely + Send + Sync + 'static,
        {
            unsafe fn get_field_slot<'a>(
                &self,
                map_addr: &mut ShapeUninit,
                field: MapField<'_>,
            ) -> Option<FieldSlot<'a>> {
                unsafe {
                    let map =
                        &mut *(map_addr.as_thin_ptr() as *mut HashMap<String, MaybeUninit<V>>);
                    Some(FieldSlot::new(
                        map.entry(field.name.to_string())
                            .or_insert_with(|| MaybeUninit::uninit())
                            .as_mut_ptr(),
                    ))
                }
            }
        }
        Shape {
            name: "HashMap<String, V>",
            size: mem::size_of::<HashMap<String, V>>(),
            align: mem::align_of::<HashMap<String, V>>(),
            shape: ShapeKind::Map(MapShape {
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
