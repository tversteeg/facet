use std::alloc::Layout;
use std::mem;

use crate::*;

macro_rules! impl_shapely_for_integer {
    ($type:ty, $scalar:expr) => {
        impl Shapely for $type {
            fn shape() -> Shape {
                Shape {
                    name: stringify!($type),
                    layout: Layout::new::<$type>(),
                    innards: Innards::Scalar($scalar),
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
                    layout: Layout::new::<$type>(),
                    innards: Innards::Scalar($scalar),
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
            layout: Layout::new::<String>(),
            innards: Innards::Scalar(Scalar::String),
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
