use std::alloc::Layout;

use crate::*;

macro_rules! impl_shapely_for_integer {
    ($type:ty, $scalar:expr) => {
        impl Shapely for $type {
            fn shape() -> Shape {
                Shape {
                    name: |_, f| write!(f, stringify!($type)),
                    typeid: mini_typeid::of::<Self>(),
                    layout: Layout::new::<$type>(),
                    innards: Innards::Scalar($scalar),
                    set_to_default: Some(|addr: *mut u8| unsafe {
                        *(addr as *mut $type) = 0;
                    }),
                    // integers don't need to drop
                    drop_in_place: None,
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
                    name: |_, f| write!(f, stringify!($type)),
                    typeid: mini_typeid::of::<Self>(),
                    layout: Layout::new::<$type>(),
                    innards: Innards::Scalar($scalar),
                    set_to_default: Some(|addr: *mut u8| unsafe {
                        *(addr as *mut $type) = 0.0;
                    }),
                    // floats don't need to drop
                    drop_in_place: None,
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
            name: |_, f| write!(f, "String"),
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<String>(),
            innards: Innards::Scalar(Scalar::String),
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut String) = String::new();
            }),
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut String);
            }),
        }
    }
}

impl Shapely for bool {
    fn shape() -> Shape {
        Shape {
            name: |_, f| write!(f, "bool"),
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<bool>(),
            innards: Innards::Scalar(Scalar::Boolean),
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut bool) = false;
            }),
            // bool doesn't need to drop
            drop_in_place: None,
        }
    }
}

impl Shapely for () {
    fn shape() -> Shape {
        Shape {
            name: |_, f| write!(f, "()"),
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<()>(),
            innards: Innards::Scalar(Scalar::Nothing),
            set_to_default: Some(|_addr: *mut u8| {}),
            drop_in_place: None,
        }
    }
}
