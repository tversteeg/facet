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
            innards: Innards::Struct {
                fields: &FieldsMaker::<T1>::FIELDS,
            },
            set_to_default: None,
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut (T1,));
            }),
        }
    }
}
