//! GENERATED: DO NOT EDIT — this file is generated from `tuples_impls.rs.j2`
//! file in the `facet-codegen` crate.

use std::{alloc::Layout, fmt};

use crate::{
    Characteristic, Def, Facet, Field, FieldFlags, MarkerTraits, OpaqueConst, Shape, StructDef,
    TypeNameOpts, ValueVTable,
};

#[inline(always)]
pub fn write_type_name_list(
    f: &mut fmt::Formatter<'_>,
    opts: TypeNameOpts,
    open: &'static str,
    delimiter: &'static str,
    close: &'static str,
    shapes: &'static [&'static Shape],
) -> fmt::Result {
    f.pad(open)?;
    if let Some(opts) = opts.for_children() {
        for (index, shape) in shapes.iter().enumerate() {
            if index > 0 {
                f.pad(delimiter)?;
            }
            shape.write_type_name(f, opts)?;
        }
    } else {
        write!(f, "⋯")?;
    }
    f.pad(close)?;
    Ok(())
}

macro_rules! field {
    ($idx:tt, $ty:ty,) => {
        Field {
            name: stringify!($idx),
            shape: $crate::shape_of(&|t: $ty| t.$idx),
            offset: std::mem::offset_of!($ty, $idx),
            flags: FieldFlags::EMPTY,
        }
    };
}

unsafe impl<T0> Facet for (T0,)
where
    T0: Facet,
{
    const DUMMY: Self = (T0::DUMMY,);
    const SHAPE: &'static Shape = &const {
        fn type_name<T0>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Facet,
        {
            write_type_name_list(f, opts, "(", ", ", ")", &[T0::SHAPE])
        }

        Shape {
            layout: Layout::new::<(T0,)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[T0::SHAPE]) {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<(T0,)>() };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[T0::SHAPE]) {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0,)>() };
                        let b = unsafe { b.as_ref::<(T0,)>() };

                        // Compare last element
                        unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const { [field!(0, (T0,),)] },
            }),
        }
    };
}
unsafe impl<T0, T1> Facet for (T0, T1)
where
    T0: Facet,
    T1: Facet,
{
    const DUMMY: Self = (T0::DUMMY, T1::DUMMY);
    const SHAPE: &'static Shape = &const {
        fn type_name<T0, T1>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
        {
            write_type_name_list(f, opts, "(", ", ", ")", &[T0::SHAPE, T1::SHAPE])
        }

        Shape {
            layout: Layout::new::<(T0, T1)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[T0::SHAPE, T1::SHAPE]) {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<(T0, T1)>() };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[T0::SHAPE, T1::SHAPE]) {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1)>() };
                        let b = unsafe { b.as_ref::<(T0, T1)>() };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const { [field!(0, (T0, T1,),), field!(1, (T0, T1,),)] },
            }),
        }
    };
}
unsafe impl<T0, T1, T2> Facet for (T0, T1, T2)
where
    T0: Facet,
    T1: Facet,
    T2: Facet,
{
    const DUMMY: Self = (T0::DUMMY, T1::DUMMY, T2::DUMMY);
    const SHAPE: &'static Shape = &const {
        fn type_name<T0, T1, T2>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
            T2: Facet,
        {
            write_type_name_list(f, opts, "(", ", ", ")", &[T0::SHAPE, T1::SHAPE, T2::SHAPE])
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1, T2>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[T0::SHAPE, T1::SHAPE, T2::SHAPE]) {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<(T0, T1, T2)>() };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T2::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.2),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[T0::SHAPE, T1::SHAPE, T2::SHAPE]) {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1, T2)>() };
                        let b = unsafe { b.as_ref::<(T0, T1, T2)>() };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare element 1
                        if !unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T2::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.2),
                                OpaqueConst::from_ref(&b.2),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, (T0, T1, T2,),),
                        field!(1, (T0, T1, T2,),),
                        field!(2, (T0, T1, T2,),),
                    ]
                },
            }),
        }
    };
}
unsafe impl<T0, T1, T2, T3> Facet for (T0, T1, T2, T3)
where
    T0: Facet,
    T1: Facet,
    T2: Facet,
    T3: Facet,
{
    const DUMMY: Self = (T0::DUMMY, T1::DUMMY, T2::DUMMY, T3::DUMMY);
    const SHAPE: &'static Shape = &const {
        fn type_name<T0, T1, T2, T3>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
            T2: Facet,
            T3: Facet,
        {
            write_type_name_list(
                f,
                opts,
                "(",
                ", ",
                ")",
                &[T0::SHAPE, T1::SHAPE, T2::SHAPE, T3::SHAPE],
            )
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1, T2, T3>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[T0::SHAPE, T1::SHAPE, T2::SHAPE, T3::SHAPE]) {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<(T0, T1, T2, T3)>() };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T2::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.2),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T3::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.3),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[T0::SHAPE, T1::SHAPE, T2::SHAPE, T3::SHAPE]) {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1, T2, T3)>() };
                        let b = unsafe { b.as_ref::<(T0, T1, T2, T3)>() };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare element 1
                        if !unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        } {
                            return false;
                        }

                        // Compare element 2
                        if !unsafe {
                            (T2::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.2),
                                OpaqueConst::from_ref(&b.2),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T3::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.3),
                                OpaqueConst::from_ref(&b.3),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, (T0, T1, T2, T3,),),
                        field!(1, (T0, T1, T2, T3,),),
                        field!(2, (T0, T1, T2, T3,),),
                        field!(3, (T0, T1, T2, T3,),),
                    ]
                },
            }),
        }
    };
}
unsafe impl<T0, T1, T2, T3, T4> Facet for (T0, T1, T2, T3, T4)
where
    T0: Facet,
    T1: Facet,
    T2: Facet,
    T3: Facet,
    T4: Facet,
{
    const DUMMY: Self = (T0::DUMMY, T1::DUMMY, T2::DUMMY, T3::DUMMY, T4::DUMMY);
    const SHAPE: &'static Shape = &const {
        fn type_name<T0, T1, T2, T3, T4>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
            T2: Facet,
            T3: Facet,
            T4: Facet,
        {
            write_type_name_list(
                f,
                opts,
                "(",
                ", ",
                ")",
                &[T0::SHAPE, T1::SHAPE, T2::SHAPE, T3::SHAPE, T4::SHAPE],
            )
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1, T2, T3, T4>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[
                        T0::SHAPE,
                        T1::SHAPE,
                        T2::SHAPE,
                        T3::SHAPE,
                        T4::SHAPE,
                    ]) {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<(T0, T1, T2, T3, T4)>() };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T2::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.2),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T3::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.3),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T4::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.4),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                ]) {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4)>() };
                        let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4)>() };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare element 1
                        if !unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        } {
                            return false;
                        }

                        // Compare element 2
                        if !unsafe {
                            (T2::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.2),
                                OpaqueConst::from_ref(&b.2),
                            )
                        } {
                            return false;
                        }

                        // Compare element 3
                        if !unsafe {
                            (T3::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.3),
                                OpaqueConst::from_ref(&b.3),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T4::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.4),
                                OpaqueConst::from_ref(&b.4),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, (T0, T1, T2, T3, T4,),),
                        field!(1, (T0, T1, T2, T3, T4,),),
                        field!(2, (T0, T1, T2, T3, T4,),),
                        field!(3, (T0, T1, T2, T3, T4,),),
                        field!(4, (T0, T1, T2, T3, T4,),),
                    ]
                },
            }),
        }
    };
}
unsafe impl<T0, T1, T2, T3, T4, T5> Facet for (T0, T1, T2, T3, T4, T5)
where
    T0: Facet,
    T1: Facet,
    T2: Facet,
    T3: Facet,
    T4: Facet,
    T5: Facet,
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
        fn type_name<T0, T1, T2, T3, T4, T5>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
            T2: Facet,
            T3: Facet,
            T4: Facet,
            T5: Facet,
        {
            write_type_name_list(
                f,
                opts,
                "(",
                ", ",
                ")",
                &[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                ],
            )
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1, T2, T3, T4, T5>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[
                        T0::SHAPE,
                        T1::SHAPE,
                        T2::SHAPE,
                        T3::SHAPE,
                        T4::SHAPE,
                        T5::SHAPE,
                    ]) {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<(T0, T1, T2, T3, T4, T5)>() };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T2::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.2),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T3::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.3),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T4::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.4),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T5::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.5),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                ]) {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5)>() };
                        let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5)>() };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare element 1
                        if !unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        } {
                            return false;
                        }

                        // Compare element 2
                        if !unsafe {
                            (T2::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.2),
                                OpaqueConst::from_ref(&b.2),
                            )
                        } {
                            return false;
                        }

                        // Compare element 3
                        if !unsafe {
                            (T3::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.3),
                                OpaqueConst::from_ref(&b.3),
                            )
                        } {
                            return false;
                        }

                        // Compare element 4
                        if !unsafe {
                            (T4::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.4),
                                OpaqueConst::from_ref(&b.4),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T5::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.5),
                                OpaqueConst::from_ref(&b.5),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, (T0, T1, T2, T3, T4, T5,),),
                        field!(1, (T0, T1, T2, T3, T4, T5,),),
                        field!(2, (T0, T1, T2, T3, T4, T5,),),
                        field!(3, (T0, T1, T2, T3, T4, T5,),),
                        field!(4, (T0, T1, T2, T3, T4, T5,),),
                        field!(5, (T0, T1, T2, T3, T4, T5,),),
                    ]
                },
            }),
        }
    };
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6> Facet for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Facet,
    T1: Facet,
    T2: Facet,
    T3: Facet,
    T4: Facet,
    T5: Facet,
    T6: Facet,
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
        fn type_name<T0, T1, T2, T3, T4, T5, T6>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
            T2: Facet,
            T3: Facet,
            T4: Facet,
            T5: Facet,
            T6: Facet,
        {
            write_type_name_list(
                f,
                opts,
                "(",
                ", ",
                ")",
                &[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                ],
            )
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1, T2, T3, T4, T5, T6>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[
                        T0::SHAPE,
                        T1::SHAPE,
                        T2::SHAPE,
                        T3::SHAPE,
                        T4::SHAPE,
                        T5::SHAPE,
                        T6::SHAPE,
                    ]) {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<(T0, T1, T2, T3, T4, T5, T6)>() };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T2::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.2),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T3::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.3),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T4::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.4),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T5::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.5),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T6::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.6),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                ]) {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5, T6)>() };
                        let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5, T6)>() };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare element 1
                        if !unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        } {
                            return false;
                        }

                        // Compare element 2
                        if !unsafe {
                            (T2::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.2),
                                OpaqueConst::from_ref(&b.2),
                            )
                        } {
                            return false;
                        }

                        // Compare element 3
                        if !unsafe {
                            (T3::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.3),
                                OpaqueConst::from_ref(&b.3),
                            )
                        } {
                            return false;
                        }

                        // Compare element 4
                        if !unsafe {
                            (T4::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.4),
                                OpaqueConst::from_ref(&b.4),
                            )
                        } {
                            return false;
                        }

                        // Compare element 5
                        if !unsafe {
                            (T5::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.5),
                                OpaqueConst::from_ref(&b.5),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T6::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.6),
                                OpaqueConst::from_ref(&b.6),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, (T0, T1, T2, T3, T4, T5, T6,),),
                        field!(1, (T0, T1, T2, T3, T4, T5, T6,),),
                        field!(2, (T0, T1, T2, T3, T4, T5, T6,),),
                        field!(3, (T0, T1, T2, T3, T4, T5, T6,),),
                        field!(4, (T0, T1, T2, T3, T4, T5, T6,),),
                        field!(5, (T0, T1, T2, T3, T4, T5, T6,),),
                        field!(6, (T0, T1, T2, T3, T4, T5, T6,),),
                    ]
                },
            }),
        }
    };
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7> Facet for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Facet,
    T1: Facet,
    T2: Facet,
    T3: Facet,
    T4: Facet,
    T5: Facet,
    T6: Facet,
    T7: Facet,
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
        fn type_name<T0, T1, T2, T3, T4, T5, T6, T7>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
            T2: Facet,
            T3: Facet,
            T4: Facet,
            T5: Facet,
            T6: Facet,
            T7: Facet,
        {
            write_type_name_list(
                f,
                opts,
                "(",
                ", ",
                ")",
                &[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                    T7::SHAPE,
                ],
            )
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1, T2, T3, T4, T5, T6, T7>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[
                        T0::SHAPE,
                        T1::SHAPE,
                        T2::SHAPE,
                        T3::SHAPE,
                        T4::SHAPE,
                        T5::SHAPE,
                        T6::SHAPE,
                        T7::SHAPE,
                    ]) {
                        Some(|value, f| {
                            let value =
                                unsafe { value.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7)>() };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T2::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.2),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T3::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.3),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T4::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.4),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T5::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.5),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T6::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.6),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T7::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.7),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                    T7::SHAPE,
                ]) {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7)>() };
                        let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7)>() };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare element 1
                        if !unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        } {
                            return false;
                        }

                        // Compare element 2
                        if !unsafe {
                            (T2::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.2),
                                OpaqueConst::from_ref(&b.2),
                            )
                        } {
                            return false;
                        }

                        // Compare element 3
                        if !unsafe {
                            (T3::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.3),
                                OpaqueConst::from_ref(&b.3),
                            )
                        } {
                            return false;
                        }

                        // Compare element 4
                        if !unsafe {
                            (T4::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.4),
                                OpaqueConst::from_ref(&b.4),
                            )
                        } {
                            return false;
                        }

                        // Compare element 5
                        if !unsafe {
                            (T5::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.5),
                                OpaqueConst::from_ref(&b.5),
                            )
                        } {
                            return false;
                        }

                        // Compare element 6
                        if !unsafe {
                            (T6::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.6),
                                OpaqueConst::from_ref(&b.6),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T7::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.7),
                                OpaqueConst::from_ref(&b.7),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, (T0, T1, T2, T3, T4, T5, T6, T7,),),
                        field!(1, (T0, T1, T2, T3, T4, T5, T6, T7,),),
                        field!(2, (T0, T1, T2, T3, T4, T5, T6, T7,),),
                        field!(3, (T0, T1, T2, T3, T4, T5, T6, T7,),),
                        field!(4, (T0, T1, T2, T3, T4, T5, T6, T7,),),
                        field!(5, (T0, T1, T2, T3, T4, T5, T6, T7,),),
                        field!(6, (T0, T1, T2, T3, T4, T5, T6, T7,),),
                        field!(7, (T0, T1, T2, T3, T4, T5, T6, T7,),),
                    ]
                },
            }),
        }
    };
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Facet for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: Facet,
    T1: Facet,
    T2: Facet,
    T3: Facet,
    T4: Facet,
    T5: Facet,
    T6: Facet,
    T7: Facet,
    T8: Facet,
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
        fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
            T2: Facet,
            T3: Facet,
            T4: Facet,
            T5: Facet,
            T6: Facet,
            T7: Facet,
            T8: Facet,
        {
            write_type_name_list(
                f,
                opts,
                "(",
                ", ",
                ")",
                &[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                    T7::SHAPE,
                    T8::SHAPE,
                ],
            )
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[
                        T0::SHAPE,
                        T1::SHAPE,
                        T2::SHAPE,
                        T3::SHAPE,
                        T4::SHAPE,
                        T5::SHAPE,
                        T6::SHAPE,
                        T7::SHAPE,
                        T8::SHAPE,
                    ]) {
                        Some(|value, f| {
                            let value =
                                unsafe { value.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>() };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T2::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.2),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T3::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.3),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T4::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.4),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T5::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.5),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T6::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.6),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T7::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.7),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T8::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.8),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                    T7::SHAPE,
                    T8::SHAPE,
                ]) {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>() };
                        let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>() };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare element 1
                        if !unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        } {
                            return false;
                        }

                        // Compare element 2
                        if !unsafe {
                            (T2::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.2),
                                OpaqueConst::from_ref(&b.2),
                            )
                        } {
                            return false;
                        }

                        // Compare element 3
                        if !unsafe {
                            (T3::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.3),
                                OpaqueConst::from_ref(&b.3),
                            )
                        } {
                            return false;
                        }

                        // Compare element 4
                        if !unsafe {
                            (T4::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.4),
                                OpaqueConst::from_ref(&b.4),
                            )
                        } {
                            return false;
                        }

                        // Compare element 5
                        if !unsafe {
                            (T5::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.5),
                                OpaqueConst::from_ref(&b.5),
                            )
                        } {
                            return false;
                        }

                        // Compare element 6
                        if !unsafe {
                            (T6::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.6),
                                OpaqueConst::from_ref(&b.6),
                            )
                        } {
                            return false;
                        }

                        // Compare element 7
                        if !unsafe {
                            (T7::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.7),
                                OpaqueConst::from_ref(&b.7),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T8::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.8),
                                OpaqueConst::from_ref(&b.8),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
                        field!(1, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
                        field!(2, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
                        field!(3, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
                        field!(4, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
                        field!(5, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
                        field!(6, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
                        field!(7, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
                        field!(8, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
                    ]
                },
            }),
        }
    };
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Facet
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: Facet,
    T1: Facet,
    T2: Facet,
    T3: Facet,
    T4: Facet,
    T5: Facet,
    T6: Facet,
    T7: Facet,
    T8: Facet,
    T9: Facet,
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
        fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
            T2: Facet,
            T3: Facet,
            T4: Facet,
            T5: Facet,
            T6: Facet,
            T7: Facet,
            T8: Facet,
            T9: Facet,
        {
            write_type_name_list(
                f,
                opts,
                "(",
                ", ",
                ")",
                &[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                    T7::SHAPE,
                    T8::SHAPE,
                    T9::SHAPE,
                ],
            )
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[
                        T0::SHAPE,
                        T1::SHAPE,
                        T2::SHAPE,
                        T3::SHAPE,
                        T4::SHAPE,
                        T5::SHAPE,
                        T6::SHAPE,
                        T7::SHAPE,
                        T8::SHAPE,
                        T9::SHAPE,
                    ]) {
                        Some(|value, f| {
                            let value = unsafe {
                                value.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>()
                            };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T2::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.2),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T3::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.3),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T4::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.4),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T5::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.5),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T6::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.6),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T7::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.7),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T8::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.8),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T9::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.9),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                    T7::SHAPE,
                    T8::SHAPE,
                    T9::SHAPE,
                ]) {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>() };
                        let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>() };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare element 1
                        if !unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        } {
                            return false;
                        }

                        // Compare element 2
                        if !unsafe {
                            (T2::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.2),
                                OpaqueConst::from_ref(&b.2),
                            )
                        } {
                            return false;
                        }

                        // Compare element 3
                        if !unsafe {
                            (T3::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.3),
                                OpaqueConst::from_ref(&b.3),
                            )
                        } {
                            return false;
                        }

                        // Compare element 4
                        if !unsafe {
                            (T4::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.4),
                                OpaqueConst::from_ref(&b.4),
                            )
                        } {
                            return false;
                        }

                        // Compare element 5
                        if !unsafe {
                            (T5::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.5),
                                OpaqueConst::from_ref(&b.5),
                            )
                        } {
                            return false;
                        }

                        // Compare element 6
                        if !unsafe {
                            (T6::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.6),
                                OpaqueConst::from_ref(&b.6),
                            )
                        } {
                            return false;
                        }

                        // Compare element 7
                        if !unsafe {
                            (T7::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.7),
                                OpaqueConst::from_ref(&b.7),
                            )
                        } {
                            return false;
                        }

                        // Compare element 8
                        if !unsafe {
                            (T8::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.8),
                                OpaqueConst::from_ref(&b.8),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T9::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.9),
                                OpaqueConst::from_ref(&b.9),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
                        field!(1, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
                        field!(2, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
                        field!(3, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
                        field!(4, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
                        field!(5, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
                        field!(6, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
                        field!(7, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
                        field!(8, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
                        field!(9, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
                    ]
                },
            }),
        }
    };
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Facet
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: Facet,
    T1: Facet,
    T2: Facet,
    T3: Facet,
    T4: Facet,
    T5: Facet,
    T6: Facet,
    T7: Facet,
    T8: Facet,
    T9: Facet,
    T10: Facet,
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
        fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
            T2: Facet,
            T3: Facet,
            T4: Facet,
            T5: Facet,
            T6: Facet,
            T7: Facet,
            T8: Facet,
            T9: Facet,
            T10: Facet,
        {
            write_type_name_list(
                f,
                opts,
                "(",
                ", ",
                ")",
                &[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                    T7::SHAPE,
                    T8::SHAPE,
                    T9::SHAPE,
                    T10::SHAPE,
                ],
            )
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[
                        T0::SHAPE,
                        T1::SHAPE,
                        T2::SHAPE,
                        T3::SHAPE,
                        T4::SHAPE,
                        T5::SHAPE,
                        T6::SHAPE,
                        T7::SHAPE,
                        T8::SHAPE,
                        T9::SHAPE,
                        T10::SHAPE,
                    ]) {
                        Some(|value, f| {
                            let value = unsafe {
                                value.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>()
                            };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T2::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.2),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T3::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.3),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T4::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.4),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T5::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.5),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T6::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.6),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T7::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.7),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T8::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.8),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T9::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.9),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T10::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.10),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                    T7::SHAPE,
                    T8::SHAPE,
                    T9::SHAPE,
                    T10::SHAPE,
                ]) {
                    Some(|a, b| {
                        let a =
                            unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>() };
                        let b =
                            unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>() };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare element 1
                        if !unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        } {
                            return false;
                        }

                        // Compare element 2
                        if !unsafe {
                            (T2::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.2),
                                OpaqueConst::from_ref(&b.2),
                            )
                        } {
                            return false;
                        }

                        // Compare element 3
                        if !unsafe {
                            (T3::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.3),
                                OpaqueConst::from_ref(&b.3),
                            )
                        } {
                            return false;
                        }

                        // Compare element 4
                        if !unsafe {
                            (T4::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.4),
                                OpaqueConst::from_ref(&b.4),
                            )
                        } {
                            return false;
                        }

                        // Compare element 5
                        if !unsafe {
                            (T5::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.5),
                                OpaqueConst::from_ref(&b.5),
                            )
                        } {
                            return false;
                        }

                        // Compare element 6
                        if !unsafe {
                            (T6::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.6),
                                OpaqueConst::from_ref(&b.6),
                            )
                        } {
                            return false;
                        }

                        // Compare element 7
                        if !unsafe {
                            (T7::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.7),
                                OpaqueConst::from_ref(&b.7),
                            )
                        } {
                            return false;
                        }

                        // Compare element 8
                        if !unsafe {
                            (T8::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.8),
                                OpaqueConst::from_ref(&b.8),
                            )
                        } {
                            return false;
                        }

                        // Compare element 9
                        if !unsafe {
                            (T9::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.9),
                                OpaqueConst::from_ref(&b.9),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T10::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.10),
                                OpaqueConst::from_ref(&b.10),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                        field!(1, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                        field!(2, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                        field!(3, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                        field!(4, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                        field!(5, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                        field!(6, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                        field!(7, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                        field!(8, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                        field!(9, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                        field!(10, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
                    ]
                },
            }),
        }
    };
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Facet
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: Facet,
    T1: Facet,
    T2: Facet,
    T3: Facet,
    T4: Facet,
    T5: Facet,
    T6: Facet,
    T7: Facet,
    T8: Facet,
    T9: Facet,
    T10: Facet,
    T11: Facet,
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
        fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result
        where
            T0: Facet,
            T1: Facet,
            T2: Facet,
            T3: Facet,
            T4: Facet,
            T5: Facet,
            T6: Facet,
            T7: Facet,
            T8: Facet,
            T9: Facet,
            T10: Facet,
            T11: Facet,
        {
            write_type_name_list(
                f,
                opts,
                "(",
                ", ",
                ")",
                &[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                    T7::SHAPE,
                    T8::SHAPE,
                    T9::SHAPE,
                    T10::SHAPE,
                    T11::SHAPE,
                ],
            )
        }

        Shape {
            layout: Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>,
                display: None,
                debug: const {
                    if Characteristic::Eq.all(&[
                        T0::SHAPE,
                        T1::SHAPE,
                        T2::SHAPE,
                        T3::SHAPE,
                        T4::SHAPE,
                        T5::SHAPE,
                        T6::SHAPE,
                        T7::SHAPE,
                        T8::SHAPE,
                        T9::SHAPE,
                        T10::SHAPE,
                        T11::SHAPE,
                    ]) {
                        Some(|value, f| {
                            let value = unsafe {
                                value.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>()
                            };
                            write!(f, "(")?;
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T2::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.2),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T3::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.3),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T4::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.4),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T5::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.5),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T6::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.6),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T7::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.7),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T8::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.8),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T9::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.9),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T10::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.10),
                                    f,
                                )
                            }?;
                            write!(f, ", ")?;
                            unsafe {
                                (T11::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.11),
                                    f,
                                )
                            }?;
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                default_in_place: None,
                clone_into: None,
                marker_traits: MarkerTraits::empty(),
                eq: if Characteristic::Eq.all(&[
                    T0::SHAPE,
                    T1::SHAPE,
                    T2::SHAPE,
                    T3::SHAPE,
                    T4::SHAPE,
                    T5::SHAPE,
                    T6::SHAPE,
                    T7::SHAPE,
                    T8::SHAPE,
                    T9::SHAPE,
                    T10::SHAPE,
                    T11::SHAPE,
                ]) {
                    Some(|a, b| {
                        let a = unsafe {
                            a.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>()
                        };
                        let b = unsafe {
                            b.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>()
                        };

                        // Compare element 0
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare element 1
                        if !unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        } {
                            return false;
                        }

                        // Compare element 2
                        if !unsafe {
                            (T2::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.2),
                                OpaqueConst::from_ref(&b.2),
                            )
                        } {
                            return false;
                        }

                        // Compare element 3
                        if !unsafe {
                            (T3::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.3),
                                OpaqueConst::from_ref(&b.3),
                            )
                        } {
                            return false;
                        }

                        // Compare element 4
                        if !unsafe {
                            (T4::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.4),
                                OpaqueConst::from_ref(&b.4),
                            )
                        } {
                            return false;
                        }

                        // Compare element 5
                        if !unsafe {
                            (T5::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.5),
                                OpaqueConst::from_ref(&b.5),
                            )
                        } {
                            return false;
                        }

                        // Compare element 6
                        if !unsafe {
                            (T6::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.6),
                                OpaqueConst::from_ref(&b.6),
                            )
                        } {
                            return false;
                        }

                        // Compare element 7
                        if !unsafe {
                            (T7::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.7),
                                OpaqueConst::from_ref(&b.7),
                            )
                        } {
                            return false;
                        }

                        // Compare element 8
                        if !unsafe {
                            (T8::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.8),
                                OpaqueConst::from_ref(&b.8),
                            )
                        } {
                            return false;
                        }

                        // Compare element 9
                        if !unsafe {
                            (T9::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.9),
                                OpaqueConst::from_ref(&b.9),
                            )
                        } {
                            return false;
                        }

                        // Compare element 10
                        if !unsafe {
                            (T10::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.10),
                                OpaqueConst::from_ref(&b.10),
                            )
                        } {
                            return false;
                        }

                        // Compare last element
                        unsafe {
                            (T11::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.11),
                                OpaqueConst::from_ref(&b.11),
                            )
                        }
                    })
                } else {
                    None
                },
                partial_ord: None,
                ord: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
            },
            def: Def::Tuple(StructDef {
                fields: &const {
                    [
                        field!(0, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(1, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(2, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(3, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(4, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(5, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(6, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(7, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(8, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(9, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(10, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                        field!(11, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
                    ]
                },
            }),
        }
    };
}
