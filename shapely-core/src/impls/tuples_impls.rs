use std::alloc::Layout;

use crate::{Def, Field, FieldFlags, Shape, Shapely, StructDef, TypeNameOpts, value_vtable};

impl<T0> Shapely for (T0,)
where
    T0: Shapely,
{
    const DUMMY: Self = (T0::DUMMY,);
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Shapely,
        {
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0,), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0,)>(),
            vtable: value_vtable!((T0,), type_name::<T0> as _),
            def: Def::Tuple(StructDef {
                fields: &const { [field!(0, T0)] },
            }),
        }
    };
}

impl<T0, T1> Shapely for (T0, T1)
where
    T0: Shapely,
    T1: Shapely,
{
    const DUMMY: Self = (T0::DUMMY, T1::DUMMY);
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Shapely,
            T1: Shapely,
        {
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0, T1), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1)>(),
            vtable: value_vtable!((T0, T1), type_name::<T0, T1> as _),
            def: Def::Tuple(StructDef {
                fields: &const { [field!(0, T0), field!(1, T1)] },
            }),
        }
    };
}

impl<T0, T1, T2> Shapely for (T0, T1, T2)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
{
    const DUMMY: Self = (T0::DUMMY, T1::DUMMY, T2::DUMMY);
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1, T2>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Shapely,
            T1: Shapely,
            T2: Shapely,
        {
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T2::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0, T1, T2), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2)>(),
            vtable: value_vtable!((T0, T1, T2), type_name::<T0, T1, T2> as _),
            def: Def::Tuple(StructDef {
                fields: &const { [field!(0, T0), field!(1, T1), field!(2, T2)] },
            }),
        }
    };
}

impl<T0, T1, T2, T3> Shapely for (T0, T1, T2, T3)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
{
    const DUMMY: Self = (T0::DUMMY, T1::DUMMY, T2::DUMMY, T3::DUMMY);
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1, T2, T3>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Shapely,
            T1: Shapely,
            T2: Shapely,
            T3: Shapely,
        {
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T2::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T3::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0, T1, T2, T3), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3)>(),
            vtable: value_vtable!((T0, T1, T2, T3), type_name::<T0, T1, T2, T3> as _),
            def: Def::Tuple(StructDef {
                fields: &const { [field!(0, T0), field!(1, T1), field!(2, T2), field!(3, T3)] },
            }),
        }
    };
}

impl<T0, T1, T2, T3, T4> Shapely for (T0, T1, T2, T3, T4)
where
    T0: Shapely,
    T1: Shapely,
    T2: Shapely,
    T3: Shapely,
    T4: Shapely,
{
    const DUMMY: Self = (T0::DUMMY, T1::DUMMY, T2::DUMMY, T3::DUMMY, T4::DUMMY);
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1, T2, T3, T4>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Shapely,
            T1: Shapely,
            T2: Shapely,
            T3: Shapely,
            T4: Shapely,
        {
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T2::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T3::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T4::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4)>(),
            vtable: value_vtable!((T0, T1, T2, T3, T4), type_name::<T0, T1, T2, T3, T4> as _),
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, T0),
                        field!(1, T1),
                        field!(2, T2),
                        field!(3, T3),
                        field!(4, T4),
                    ]
                },
            }),
        }
    };
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
    const DUMMY: Self = (
        T0::DUMMY,
        T1::DUMMY,
        T2::DUMMY,
        T3::DUMMY,
        T4::DUMMY,
        T5::DUMMY,
    );
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1, T2, T3, T4, T5>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
        where
            T0: Shapely,
            T1: Shapely,
            T2: Shapely,
            T3: Shapely,
            T4: Shapely,
            T5: Shapely,
        {
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T2::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T3::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T4::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T5::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4, T5), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5)>(),
            vtable: value_vtable!(
                (T0, T1, T2, T3, T4, T5),
                type_name::<T0, T1, T2, T3, T4, T5> as _
            ),
            def: Def::Tuple(StructDef {
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
            }),
        }
    };
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
    const DUMMY: Self = (
        T0::DUMMY,
        T1::DUMMY,
        T2::DUMMY,
        T3::DUMMY,
        T4::DUMMY,
        T5::DUMMY,
        T6::DUMMY,
    );
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1, T2, T3, T4, T5, T6>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
        where
            T0: Shapely,
            T1: Shapely,
            T2: Shapely,
            T3: Shapely,
            T4: Shapely,
            T5: Shapely,
            T6: Shapely,
        {
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T2::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T3::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T4::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T5::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T6::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4, T5, T6), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6)>(),
            vtable: value_vtable!(
                (T0, T1, T2, T3, T4, T5, T6),
                type_name::<T0, T1, T2, T3, T4, T5, T6> as _
            ),
            def: Def::Tuple(StructDef {
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
            }),
        }
    };
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
    const DUMMY: Self = (
        T0::DUMMY,
        T1::DUMMY,
        T2::DUMMY,
        T3::DUMMY,
        T4::DUMMY,
        T5::DUMMY,
        T6::DUMMY,
        T7::DUMMY,
    );
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1, T2, T3, T4, T5, T6, T7>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
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
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T2::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T3::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T4::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T5::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T6::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T7::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4, T5, T6, T7), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7)>(),
            vtable: value_vtable!(
                (T0, T1, T2, T3, T4, T5, T6, T7),
                type_name::<T0, T1, T2, T3, T4, T5, T6, T7> as _
            ),
            def: Def::Tuple(StructDef {
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
            }),
        }
    };
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
    const DUMMY: Self = (
        T0::DUMMY,
        T1::DUMMY,
        T2::DUMMY,
        T3::DUMMY,
        T4::DUMMY,
        T5::DUMMY,
        T6::DUMMY,
        T7::DUMMY,
        T8::DUMMY,
    );
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
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
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T2::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T3::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T4::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T5::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T6::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T7::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T8::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4, T5, T6, T7, T8), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>(),
            vtable: value_vtable!(
                (T0, T1, T2, T3, T4, T5, T6, T7, T8),
                type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8> as _
            ),
            def: Def::Tuple(StructDef {
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
            }),
        }
    };
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
    const DUMMY: Self = (
        T0::DUMMY,
        T1::DUMMY,
        T2::DUMMY,
        T3::DUMMY,
        T4::DUMMY,
        T5::DUMMY,
        T6::DUMMY,
        T7::DUMMY,
        T8::DUMMY,
        T9::DUMMY,
    );
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
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
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T2::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T3::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T4::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T5::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T6::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T7::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T8::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T9::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>(),
            vtable: value_vtable!(
                (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9),
                type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> as _
            ),
            def: Def::Tuple(StructDef {
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
            }),
        }
    };
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
    const DUMMY: Self = (
        T0::DUMMY,
        T1::DUMMY,
        T2::DUMMY,
        T3::DUMMY,
        T4::DUMMY,
        T5::DUMMY,
        T6::DUMMY,
        T7::DUMMY,
        T8::DUMMY,
        T9::DUMMY,
        T10::DUMMY,
    );
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
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
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T2::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T3::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T4::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T5::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T6::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T7::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T8::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T9::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T10::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!(
                        (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
                        $idx
                    ),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>(),
            vtable: value_vtable!(
                (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
                type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> as _
            ),
            def: Def::Tuple(StructDef {
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
            }),
        }
    };
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
    const DUMMY: Self = (
        T0::DUMMY,
        T1::DUMMY,
        T2::DUMMY,
        T3::DUMMY,
        T4::DUMMY,
        T5::DUMMY,
        T6::DUMMY,
        T7::DUMMY,
        T8::DUMMY,
        T9::DUMMY,
        T10::DUMMY,
        T11::DUMMY,
    );
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
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
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T2::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T3::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T4::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T5::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T6::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T7::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T8::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T9::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T10::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T11::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!(
                        (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
                        $idx
                    ),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>(),
            vtable: value_vtable!(
                (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
                type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> as _
            ),
            def: Def::Tuple(StructDef {
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
            }),
        }
    };
}
