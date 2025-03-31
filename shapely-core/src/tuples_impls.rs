use std::alloc::Layout;

use crate::{Field, FieldFlags, Innards, Scalar, Shape, ShapeDesc, Shapely, mini_typeid};

impl Shapely for () {
    fn shape() -> Shape {
        Shape {
            name: |f| write!(f, "()"),
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<()>(),
            innards: Innards::Scalar(Scalar::Nothing),
            set_to_default: Some(|_addr: *mut u8| {}),
            drop_in_place: None,
        }
    }
}

impl<T1: Shapely> Shapely for (T1,)
where
    T1: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<T1>,
        }

        impl<T1> FieldsMaker<T1>
        where
            T1: Shapely,
        {
            const FIELDS: [Field; 1] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: 0,
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T1::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T1,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1,));
            }),
        }
    }
}

impl<T1, T2> Shapely for (T1, T2,)
where
    T1: Shapely, T2: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2,)>,
        }

        impl<T1, T2> FieldsMaker<T1, T2>
        where
            T1: Shapely, T2: Shapely,
        {
            const FIELDS: [Field; 2] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2,), 1),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T1, T2,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2,));
            }),
        }
    }
}

impl<T1, T2, T3> Shapely for (T1, T2, T3,)
where
    T1: Shapely, T2: Shapely, T3: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2, T3> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2, T3,)>,
        }

        impl<T1, T2, T3> FieldsMaker<T1, T2, T3>
        where
            T1: Shapely, T2: Shapely, T3: Shapely,
        {
            const FIELDS: [Field; 3] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2, T3,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2, T3,), 1),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "2",
                shape: ShapeDesc(T3::shape),
                offset: std::mem::offset_of!((T1, T2, T3,), 2),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
                (T1::shape().name)(f)?;
                write!(f, ",")?;
                (T2::shape().name)(f)?;
                write!(f, ",")?;
                (T3::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T1, T2, T3,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2, T3>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2, T3,));
            }),
        }
    }
}

impl<T1, T2, T3, T4> Shapely for (T1, T2, T3, T4,)
where
    T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2, T3, T4> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2, T3, T4,)>,
        }

        impl<T1, T2, T3, T4> FieldsMaker<T1, T2, T3, T4>
        where
            T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely,
        {
            const FIELDS: [Field; 4] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4,), 1),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "2",
                shape: ShapeDesc(T3::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4,), 2),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "3",
                shape: ShapeDesc(T4::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4,), 3),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
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
            layout: Layout::new::<(T1, T2, T3, T4,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2, T3, T4>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2, T3, T4,));
            }),
        }
    }
}

impl<T1, T2, T3, T4, T5> Shapely for (T1, T2, T3, T4, T5,)
where
    T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2, T3, T4, T5> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2, T3, T4, T5,)>,
        }

        impl<T1, T2, T3, T4, T5> FieldsMaker<T1, T2, T3, T4, T5>
        where
            T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely,
        {
            const FIELDS: [Field; 5] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5,), 1),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "2",
                shape: ShapeDesc(T3::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5,), 2),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "3",
                shape: ShapeDesc(T4::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5,), 3),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "4",
                shape: ShapeDesc(T5::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5,), 4),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
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
            layout: Layout::new::<(T1, T2, T3, T4, T5,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2, T3, T4, T5>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2, T3, T4, T5,));
            }),
        }
    }
}

impl<T1, T2, T3, T4, T5, T6> Shapely for (T1, T2, T3, T4, T5, T6,)
where
    T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2, T3, T4, T5, T6> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2, T3, T4, T5, T6,)>,
        }

        impl<T1, T2, T3, T4, T5, T6> FieldsMaker<T1, T2, T3, T4, T5, T6>
        where
            T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely,
        {
            const FIELDS: [Field; 6] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6,), 1),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "2",
                shape: ShapeDesc(T3::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6,), 2),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "3",
                shape: ShapeDesc(T4::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6,), 3),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "4",
                shape: ShapeDesc(T5::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6,), 4),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "5",
                shape: ShapeDesc(T6::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6,), 5),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
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
            layout: Layout::new::<(T1, T2, T3, T4, T5, T6,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2, T3, T4, T5, T6>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2, T3, T4, T5, T6,));
            }),
        }
    }
}

impl<T1, T2, T3, T4, T5, T6, T7> Shapely for (T1, T2, T3, T4, T5, T6, T7,)
where
    T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2, T3, T4, T5, T6, T7> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2, T3, T4, T5, T6, T7,)>,
        }

        impl<T1, T2, T3, T4, T5, T6, T7> FieldsMaker<T1, T2, T3, T4, T5, T6, T7>
        where
            T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely,
        {
            const FIELDS: [Field; 7] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7,), 1),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "2",
                shape: ShapeDesc(T3::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7,), 2),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "3",
                shape: ShapeDesc(T4::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7,), 3),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "4",
                shape: ShapeDesc(T5::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7,), 4),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "5",
                shape: ShapeDesc(T6::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7,), 5),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "6",
                shape: ShapeDesc(T7::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7,), 6),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
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
            layout: Layout::new::<(T1, T2, T3, T4, T5, T6, T7,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2, T3, T4, T5, T6, T7>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2, T3, T4, T5, T6, T7,));
            }),
        }
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> Shapely for (T1, T2, T3, T4, T5, T6, T7, T8,)
where
    T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely, T8: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2, T3, T4, T5, T6, T7, T8> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2, T3, T4, T5, T6, T7, T8,)>,
        }

        impl<T1, T2, T3, T4, T5, T6, T7, T8> FieldsMaker<T1, T2, T3, T4, T5, T6, T7, T8>
        where
            T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely, T8: Shapely,
        {
            const FIELDS: [Field; 8] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8,), 1),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "2",
                shape: ShapeDesc(T3::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8,), 2),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "3",
                shape: ShapeDesc(T4::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8,), 3),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "4",
                shape: ShapeDesc(T5::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8,), 4),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "5",
                shape: ShapeDesc(T6::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8,), 5),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "6",
                shape: ShapeDesc(T7::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8,), 6),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "7",
                shape: ShapeDesc(T8::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8,), 7),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
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
            layout: Layout::new::<(T1, T2, T3, T4, T5, T6, T7, T8,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2, T3, T4, T5, T6, T7, T8>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2, T3, T4, T5, T6, T7, T8,));
            }),
        }
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> Shapely for (T1, T2, T3, T4, T5, T6, T7, T8, T9,)
where
    T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely, T8: Shapely, T9: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2, T3, T4, T5, T6, T7, T8, T9> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2, T3, T4, T5, T6, T7, T8, T9,)>,
        }

        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> FieldsMaker<T1, T2, T3, T4, T5, T6, T7, T8, T9>
        where
            T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely, T8: Shapely, T9: Shapely,
        {
            const FIELDS: [Field; 9] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9,), 1),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "2",
                shape: ShapeDesc(T3::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9,), 2),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "3",
                shape: ShapeDesc(T4::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9,), 3),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "4",
                shape: ShapeDesc(T5::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9,), 4),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "5",
                shape: ShapeDesc(T6::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9,), 5),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "6",
                shape: ShapeDesc(T7::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9,), 6),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "7",
                shape: ShapeDesc(T8::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9,), 7),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "8",
                shape: ShapeDesc(T9::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9,), 8),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
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
            layout: Layout::new::<(T1, T2, T3, T4, T5, T6, T7, T8, T9,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2, T3, T4, T5, T6, T7, T8, T9>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2, T3, T4, T5, T6, T7, T8, T9,));
            }),
        }
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Shapely for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,)
where
    T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely, T8: Shapely, T9: Shapely, T10: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,)>,
        }

        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> FieldsMaker<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
        where
            T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely, T8: Shapely, T9: Shapely, T10: Shapely,
        {
            const FIELDS: [Field; 10] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,), 1),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "2",
                shape: ShapeDesc(T3::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,), 2),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "3",
                shape: ShapeDesc(T4::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,), 3),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "4",
                shape: ShapeDesc(T5::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,), 4),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "5",
                shape: ShapeDesc(T6::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,), 5),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "6",
                shape: ShapeDesc(T7::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,), 6),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "7",
                shape: ShapeDesc(T8::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,), 7),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "8",
                shape: ShapeDesc(T9::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,), 8),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "9",
                shape: ShapeDesc(T10::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,), 9),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
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
            layout: Layout::new::<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,));
            }),
        }
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Shapely for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,)
where
    T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely, T8: Shapely, T9: Shapely, T10: Shapely, T11: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,)>,
        }

        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> FieldsMaker<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
        where
            T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely, T8: Shapely, T9: Shapely, T10: Shapely, T11: Shapely,
        {
            const FIELDS: [Field; 11] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 1),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "2",
                shape: ShapeDesc(T3::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 2),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "3",
                shape: ShapeDesc(T4::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 3),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "4",
                shape: ShapeDesc(T5::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 4),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "5",
                shape: ShapeDesc(T6::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 5),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "6",
                shape: ShapeDesc(T7::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 6),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "7",
                shape: ShapeDesc(T8::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 7),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "8",
                shape: ShapeDesc(T9::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 8),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "9",
                shape: ShapeDesc(T10::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 9),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "10",
                shape: ShapeDesc(T11::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,), 10),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
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
            layout: Layout::new::<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,));
            }),
        }
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Shapely for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,)
where
    T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely, T8: Shapely, T9: Shapely, T10: Shapely, T11: Shapely, T12: Shapely,
{
    fn shape() -> Shape {
        struct FieldsMaker<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
            #[allow(clippy::type_complexity)]
            _phantom: std::marker::PhantomData<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,)>,
        }

        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> FieldsMaker<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
        where
            T1: Shapely, T2: Shapely, T3: Shapely, T4: Shapely, T5: Shapely, T6: Shapely, T7: Shapely, T8: Shapely, T9: Shapely, T10: Shapely, T11: Shapely, T12: Shapely,
        {
            const FIELDS: [Field; 12] = [Field {
                name: "0",
                shape: ShapeDesc(T1::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 0),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "1",
                shape: ShapeDesc(T2::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 1),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "2",
                shape: ShapeDesc(T3::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 2),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "3",
                shape: ShapeDesc(T4::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 3),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "4",
                shape: ShapeDesc(T5::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 4),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "5",
                shape: ShapeDesc(T6::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 5),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "6",
                shape: ShapeDesc(T7::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 6),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "7",
                shape: ShapeDesc(T8::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 7),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "8",
                shape: ShapeDesc(T9::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 8),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "9",
                shape: ShapeDesc(T10::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 9),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "10",
                shape: ShapeDesc(T11::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 10),
                flags: FieldFlags::EMPTY,
            }, Field {
                name: "11",
                shape: ShapeDesc(T12::shape),
                offset: std::mem::offset_of!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,), 11),
                flags: FieldFlags::EMPTY,
            }];
        }

        Shape {
            name: |f| {
                write!(f, "(")?;
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
                write!(f, ",")?;
                (T12::shape().name)(f)?;
                write!(f, ",)")
            },
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,)>(),
            innards: Innards::Tuple {
                fields: &FieldsMaker::<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,));
            }),
        }
    }
}
