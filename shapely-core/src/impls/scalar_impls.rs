use std::alloc::Layout;

use crate::*;

impl Shapely for u8 {
    fn shape() -> Shape {
        Shape {
            name: |f, _nameopts| write!(f, stringify!($type)),
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<Self>(),
            innards: Innards::Scalar {
                vtable: ScalarVTable {
                    display: Some(|this, _| {
                        let value = unsafe { *this.as_ptr::<u8>() };
                        format!("{}", value)
                    }),
                    debug: Some(|this, _| format!("{:?}", unsafe { this.as_ref::<Self>() })),
                    default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
                    from_str: Some(|target, s| match s.parse::<u8>() {
                        Ok(value) => {
                            unsafe { std::ptr::write(target.0 as *mut u8, value) };
                            Ok(())
                        }
                        Err(e) => Err(e.to_string()),
                    }),
                    eq: Some(|left, right| {
                        let left_val = unsafe { *left.as_ptr::<u8>() };
                        let right_val = unsafe { *right.as_ptr::<u8>() };
                        left_val == right_val
                    }),
                    cmp: Some(|left, right| {
                        let left_val = unsafe { *left.as_ptr::<u8>() };
                        let right_val = unsafe { *right.as_ptr::<u8>() };
                        left_val.cmp(&right_val)
                    }),
                    hash: Some(|value, hasher| {
                        let val = unsafe { *value.as_ptr::<u8>() };
                        unsafe { &mut *hasher }.write_u8(val);
                    }),
                },
            },
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut Self) = 0;
            }),
            // integers don't need to drop
            drop_in_place: None,
        }
    }
}

macro_rules! impl_shapely_for_integer {
    ($type:ty, $scalar:expr) => {
        impl Shapely for $type {
            fn shape() -> Shape {
                Shape {
                    name: |f, _nameopts| write!(f, stringify!($type)),
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
impl_shapely_for_integer!(u128, Scalar::U128);
impl_shapely_for_integer!(i8, Scalar::I8);
impl_shapely_for_integer!(i16, Scalar::I16);
impl_shapely_for_integer!(i32, Scalar::I32);
impl_shapely_for_integer!(i64, Scalar::I64);
impl_shapely_for_integer!(i128, Scalar::I128);

macro_rules! impl_shapely_for_float {
    ($type:ty, $scalar:expr) => {
        impl Shapely for $type {
            fn shape() -> Shape {
                Shape {
                    name: |f, _nameopts| write!(f, stringify!($type)),
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

impl_shapely_for_float!(f32, Scalar::F32);
impl_shapely_for_float!(f64, Scalar::F64);

impl Shapely for String {
    fn shape() -> Shape {
        Shape {
            name: |f, _nameopts| write!(f, "String"),
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
            name: |f, _nameopts| write!(f, "bool"),
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
            name: |f, _nameopts| write!(f, "()"),
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<()>(),
            innards: Innards::Scalar(Scalar::Nothing),
            set_to_default: None,
            drop_in_place: None,
        }
    }
}
