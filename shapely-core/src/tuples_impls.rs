use std::alloc::Layout;

use crate::{mini_typeid, Field, FieldFlags, Innards, Shape, ShapeDesc, Shapely};

impl<T0> Shapely for (T0,)
where
    T0: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!((T0,), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0,)>(),
            innards: Innards::Tuple {
                fields: &const { [field!(0, T0)] },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0,));
            }),
        }
    }
}

impl<T0, T1> Shapely for (T0, T1)
where
    T0: Shapely,
    T1: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!((T0, T1), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1)>(),
            innards: Innards::Tuple {
                fields: &const { [field!(0, T0), field!(1, T1)] },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0, T1));
            }),
        }
    }
}

impl<T0, T1, T2> Shapely for (T0, T1, T2)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!((T0, T1, T2), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1, T2)>(),
            innards: Innards::Tuple {
                fields: &const { [field!(0, T0), field!(1, T1), field!(2, T2)] },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0, T1, T2));
            }),
        }
    }
}

impl<T0, T1, T2, T3> Shapely for (T0, T1, T2, T3)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!((T0, T1, T2, T3), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",")?;
                (T3::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1, T2, T3)>(),
            innards: Innards::Tuple {
                fields: &const { [field!(0, T0), field!(1, T1), field!(2, T2), field!(3, T3)] },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0, T1, T2, T3));
            }),
        }
    }
}

impl<T0, T1, T2, T3, T4> Shapely for (T0, T1, T2, T3, T4)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
    T4: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",")?;
                (T3::shape().name)(f)?;
                write!(f, ",")?;
                (T4::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1, T2, T3, T4)>(),
            innards: Innards::Tuple {
                fields: &const {
                    [
                        field!(0, T0),
                        field!(1, T1),
                        field!(2, T2),
                        field!(3, T3),
                        field!(4, T4),
                    ]
                },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0, T1, T2, T3, T4));
            }),
        }
    }
}

impl<T0, T1, T2, T3, T4, T5> Shapely for (T0, T1, T2, T3, T4, T5)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
    T4: Shapely,
    T5: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4, T5), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",")?;
                (T3::shape().name)(f)?;
                write!(f, ",")?;
                (T4::shape().name)(f)?;
                write!(f, ",")?;
                (T5::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5)>(),
            innards: Innards::Tuple {
                fields: &const {
                    [
                        field!(0, T0),
                        field!(1, T1),
                        field!(2, T2),
                        field!(3, T3),
                        field!(4, T4),
                        field!(5, T5),
                    ]
                },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0, T1, T2, T3, T4, T5));
            }),
        }
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> Shapely for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
    T4: Shapely,
    T5: Shapely,
    T6: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4, T5, T6), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",")?;
                (T3::shape().name)(f)?;
                write!(f, ",")?;
                (T4::shape().name)(f)?;
                write!(f, ",")?;
                (T5::shape().name)(f)?;
                write!(f, ",")?;
                (T6::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6)>(),
            innards: Innards::Tuple {
                fields: &const {
                    [
                        field!(0, T0),
                        field!(1, T1),
                        field!(2, T2),
                        field!(3, T3),
                        field!(4, T4),
                        field!(5, T5),
                        field!(6, T6),
                    ]
                },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0, T1, T2, T3, T4, T5, T6));
            }),
        }
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Shapely for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
    T4: Shapely,
    T5: Shapely,
    T6: Shapely,
    T7: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4, T5, T6, T7), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",")?;
                (T3::shape().name)(f)?;
                write!(f, ",")?;
                (T4::shape().name)(f)?;
                write!(f, ",")?;
                (T5::shape().name)(f)?;
                write!(f, ",")?;
                (T6::shape().name)(f)?;
                write!(f, ",")?;
                (T7::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7)>(),
            innards: Innards::Tuple {
                fields: &const {
                    [
                        field!(0, T0),
                        field!(1, T1),
                        field!(2, T2),
                        field!(3, T3),
                        field!(4, T4),
                        field!(5, T5),
                        field!(6, T6),
                        field!(7, T7),
                    ]
                },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0, T1, T2, T3, T4, T5, T6, T7));
            }),
        }
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Shapely for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
    T4: Shapely,
    T5: Shapely,
    T6: Shapely,
    T7: Shapely,
    T8: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4, T5, T6, T7, T8), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",")?;
                (T3::shape().name)(f)?;
                write!(f, ",")?;
                (T4::shape().name)(f)?;
                write!(f, ",")?;
                (T5::shape().name)(f)?;
                write!(f, ",")?;
                (T6::shape().name)(f)?;
                write!(f, ",")?;
                (T7::shape().name)(f)?;
                write!(f, ",")?;
                (T8::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>(),
            innards: Innards::Tuple {
                fields: &const {
                    [
                        field!(0, T0),
                        field!(1, T1),
                        field!(2, T2),
                        field!(3, T3),
                        field!(4, T4),
                        field!(5, T5),
                        field!(6, T6),
                        field!(7, T7),
                        field!(8, T8),
                    ]
                },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0, T1, T2, T3, T4, T5, T6, T7, T8));
            }),
        }
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Shapely for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
    T4: Shapely,
    T5: Shapely,
    T6: Shapely,
    T7: Shapely,
    T8: Shapely,
    T9: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",")?;
                (T3::shape().name)(f)?;
                write!(f, ",")?;
                (T4::shape().name)(f)?;
                write!(f, ",")?;
                (T5::shape().name)(f)?;
                write!(f, ",")?;
                (T6::shape().name)(f)?;
                write!(f, ",")?;
                (T7::shape().name)(f)?;
                write!(f, ",")?;
                (T8::shape().name)(f)?;
                write!(f, ",")?;
                (T9::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>(),
            innards: Innards::Tuple {
                fields: &const {
                    [
                        field!(0, T0),
                        field!(1, T1),
                        field!(2, T2),
                        field!(3, T3),
                        field!(4, T4),
                        field!(5, T5),
                        field!(6, T6),
                        field!(7, T7),
                        field!(8, T8),
                        field!(9, T9),
                    ]
                },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9));
            }),
        }
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Shapely
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
    T4: Shapely,
    T5: Shapely,
    T6: Shapely,
    T7: Shapely,
    T8: Shapely,
    T9: Shapely,
    T10: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!(
                        (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
                        $idx
                    ),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",")?;
                (T3::shape().name)(f)?;
                write!(f, ",")?;
                (T4::shape().name)(f)?;
                write!(f, ",")?;
                (T5::shape().name)(f)?;
                write!(f, ",")?;
                (T6::shape().name)(f)?;
                write!(f, ",")?;
                (T7::shape().name)(f)?;
                write!(f, ",")?;
                (T8::shape().name)(f)?;
                write!(f, ",")?;
                (T9::shape().name)(f)?;
                write!(f, ",")?;
                (T10::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>(),
            innards: Innards::Tuple {
                fields: &const {
                    [
                        field!(0, T0),
                        field!(1, T1),
                        field!(2, T2),
                        field!(3, T3),
                        field!(4, T4),
                        field!(5, T5),
                        field!(6, T6),
                        field!(7, T7),
                        field!(8, T8),
                        field!(9, T9),
                        field!(10, T10),
                    ]
                },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10));
            }),
        }
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Shapely
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
    T4: Shapely,
    T5: Shapely,
    T6: Shapely,
    T7: Shapely,
    T8: Shapely,
    T9: Shapely,
    T10: Shapely,
    T11: Shapely,
{
    fn shape() -> Shape {
        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: ShapeDesc(<$ty>::shape),
                    offset: std::mem::offset_of!(
                        (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
                        $idx
                    ),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T0::shape().name)(f)?;
                write!(f, ",")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",")?;
                (T3::shape().name)(f)?;
                write!(f, ",")?;
                (T4::shape().name)(f)?;
                write!(f, ",")?;
                (T5::shape().name)(f)?;
                write!(f, ",")?;
                (T6::shape().name)(f)?;
                write!(f, ",")?;
                (T7::shape().name)(f)?;
                write!(f, ",")?;
                (T8::shape().name)(f)?;
                write!(f, ",")?;
                (T9::shape().name)(f)?;
                write!(f, ",")?;
                (T10::shape().name)(f)?;
                write!(f, ",")?;
                (T11::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>(),
            innards: Innards::Tuple {
                fields: &const {
                    [
                        field!(0, T0),
                        field!(1, T1),
                        field!(2, T2),
                        field!(3, T3),
                        field!(4, T4),
                        field!(5, T5),
                        field!(6, T6),
                        field!(7, T7),
                        field!(8, T8),
                        field!(9, T9),
                        field!(10, T10),
                        field!(11, T11),
                    ]
                },
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(
                    addr as *mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
                );
            }),
        }
    }
}
