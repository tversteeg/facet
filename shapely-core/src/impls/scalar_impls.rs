use std::{alloc::Layout, hash::Hash as _};

use crate::*;

impl Shapely for () {
    fn shape() -> Shape {
        Shape {
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<Self>(),
            innards: Innards::Scalar,
            vtable: || ValueVTable {
                type_name: |f, _opts| write!(f, "()"),
                display: Some(|_value, mut f| write!(f, "()")),
                debug: Some(|_value, mut f| write!(f, "()")),
                default_in_place: Some(|target| unsafe { Some(target.write(())) }),
                eq: Some(|_left, _right| true), // () == () is always true
                cmp: Some(|_left, _right| std::cmp::Ordering::Equal), // () cmp () is always Equal
                hash: Some(|_value, _hasher_self, _hasher_write_fn| {}),
                drop_in_place: None, // unit type doesn't need dropping
                parse: Some(|s, target| {
                    if s == "()" {
                        Some(unsafe { target.write(()) })
                    } else {
                        None
                    }
                }),
                try_from: None,
            },
        }
    }
}

impl Shapely for String {
    fn shape() -> Shape {
        Shape {
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<Self>(),
            innards: Innards::Scalar,
            vtable: || ValueVTable {
                type_name: |f, _opts| write!(f, "String"),
                display: Some(|value, mut f| {
                    let val = unsafe { value.as_ref::<Self>() };
                    write!(f, "{}", val)
                }),
                debug: Some(|value, mut f| {
                    let val = unsafe { value.as_ref::<Self>() };
                    write!(f, "{:?}", val)
                }),
                default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
                eq: Some(|left, right| unsafe { left.as_ref::<Self>() == right.as_ref::<Self>() }),
                cmp: Some(|left, right| unsafe {
                    left.as_ref::<Self>().cmp(right.as_ref::<Self>())
                }),
                hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
                    value
                        .as_ref::<Self>()
                        .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
                }),
                drop_in_place: Some(|value| unsafe {
                    std::ptr::drop_in_place(value.as_mut_ptr::<Self>());
                }),
                parse: Some(|s, target| Some(unsafe { target.write(s.to_string()) })),
                try_from: None,
            },
        }
    }
}

impl Shapely for bool {
    fn shape() -> Shape {
        Shape {
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<Self>(),
            innards: Innards::Scalar,
            vtable: || ValueVTable {
                type_name: |f, _opts| write!(f, "bool"),
                display: Some(|value, mut f| {
                    let val = unsafe { value.as_ref::<Self>() };
                    write!(f, "{}", val)
                }),
                debug: Some(|value, mut f| {
                    let val = unsafe { value.as_ref::<Self>() };
                    write!(f, "{:?}", val)
                }),
                default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
                eq: Some(|left, right| unsafe { left.as_ref::<Self>() == right.as_ref::<Self>() }),
                cmp: Some(|left, right| unsafe {
                    left.as_ref::<Self>().cmp(right.as_ref::<Self>())
                }),
                hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
                    value
                        .as_ref::<Self>()
                        .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
                }),
                drop_in_place: None, // bool doesn't need dropping
                parse: Some(|s, target| {
                    s.parse::<Self>()
                        .ok()
                        .map(|value| unsafe { target.write(value) })
                }),
                try_from: None,
            },
        }
    }
}

macro_rules! impl_shapely_for_integer {
    ($type:ty) => {
        impl Shapely for $type {
            fn shape() -> Shape {
                Shape {
                    typeid: mini_typeid::of::<Self>(),
                    layout: Layout::new::<Self>(),
                    innards: Innards::Scalar,
                    vtable: || ValueVTable {
                        type_name: |f, _opts| write!(f, stringify!($type)),
                        display: Some(|value, mut f| {
                            let val = unsafe { *value.as_ptr::<Self>() };
                            write!(f, "{}", val)
                        }),
                        debug: Some(|value, mut f| {
                            let val = unsafe { *value.as_ptr::<Self>() };
                            write!(f, "{:?}", val)
                        }),
                        default_in_place: Some(|target| unsafe {
                            Some(target.write(Self::default()))
                        }),
                        eq: Some(|left, right| unsafe {
                            left.as_ref::<Self>() == right.as_ref::<Self>()
                        }),
                        cmp: Some(|left, right| unsafe {
                            left.as_ref::<Self>().cmp(right.as_ref::<Self>())
                        }),
                        hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
                            value
                                .as_ref::<Self>()
                                .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
                        }),
                        drop_in_place: None,
                        parse: Some(|s, target| {
                            s.parse::<Self>()
                                .ok()
                                .map(|value| unsafe { target.write(value) })
                        }),
                        try_from: None,
                    },
                }
            }
        }
    };
}

impl_shapely_for_integer!(u8);
impl_shapely_for_integer!(i8);
impl_shapely_for_integer!(u16);
impl_shapely_for_integer!(i16);
impl_shapely_for_integer!(u32);
impl_shapely_for_integer!(i32);
impl_shapely_for_integer!(u64);
impl_shapely_for_integer!(i64);
impl_shapely_for_integer!(u128);
impl_shapely_for_integer!(i128);

macro_rules! impl_shapely_for_float {
    ($type:ty) => {
        impl Shapely for $type {
            fn shape() -> Shape {
                Shape {
                    typeid: mini_typeid::of::<Self>(),
                    layout: Layout::new::<Self>(),
                    innards: Innards::Scalar,
                    vtable: || ValueVTable {
                        type_name: |f, _opts| write!(f, stringify!($type)),
                        display: Some(|value, mut f| {
                            let val = unsafe { *value.as_ptr::<Self>() };
                            write!(f, "{}", val)
                        }),
                        debug: Some(|value, mut f| {
                            let val = unsafe { *value.as_ptr::<Self>() };
                            write!(f, "{:?}", val)
                        }),
                        default_in_place: Some(|target| unsafe {
                            Some(target.write(Self::default()))
                        }),
                        eq: Some(|left, right| unsafe {
                            left.as_ref::<Self>() == right.as_ref::<Self>()
                        }),
                        cmp: Some(|left, right| unsafe {
                            left.as_ref::<Self>()
                                .partial_cmp(right.as_ref::<Self>())
                                .unwrap_or(std::cmp::Ordering::Equal)
                        }),
                        hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
                            value
                                .as_ref::<Self>()
                                .to_bits()
                                .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
                        }),
                        drop_in_place: None,
                        parse: Some(|s, target| {
                            s.parse::<Self>()
                                .ok()
                                .map(|value| unsafe { target.write(value) })
                        }),
                        try_from: None,
                    },
                }
            }
        }
    };
}

impl_shapely_for_float!(f32);
impl_shapely_for_float!(f64);

// impl Shapely for String {
//     fn shape() -> Shape {
//         Shape {
//             name: |f, _nameopts| write!(f, "String"),
//             typeid: mini_typeid::of::<Self>(),
//             layout: Layout::new::<String>(),
//             innards: Innards::Scalar(Scalar::String),
//             set_to_default: Some(|addr: *mut u8| unsafe {
//                 *(addr as *mut String) = String::new();
//             }),
//             drop_in_place: Some(|addr: *mut u8| unsafe {
//                 std::ptr::drop_in_place(addr as *mut String);
//             }),
//         }
//     }
// }

// impl Shapely for bool {
//     fn shape() -> Shape {
//         Shape {
//             name: |f, _nameopts| write!(f, "bool"),
//             typeid: mini_typeid::of::<Self>(),
//             layout: Layout::new::<bool>(),
//             innards: Innards::Scalar(Scalar::Boolean),
//             set_to_default: Some(|addr: *mut u8| unsafe {
//                 *(addr as *mut bool) = false;
//             }),
//             // bool doesn't need to drop
//             drop_in_place: None,
//         }
//     }
// }

// impl Shapely for () {
//     fn shape() -> Shape {
//         Shape {
//             name: |f, _nameopts| write!(f, "()"),
//             typeid: mini_typeid::of::<Self>(),
//             layout: Layout::new::<()>(),
//             innards: Innards::Scalar(Scalar::Nothing),
//             set_to_default: None,
//             drop_in_place: None,
//         }
//     }
// }
