use crate::value_vtable;
use crate::*;
use std::alloc::Layout;

impl Shapely for () {
    const SHAPE: &'static Shape = &const {
        Shape {
            layout: Layout::new::<Self>(),
            def: Def::Scalar(ScalarDef::of::<Self>()),
            vtable: value_vtable!((), |f, _opts| write!(f, "()")),
        }
    };
}

impl Shapely for String {
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: value_vtable!(String, |f, _opts| write!(f, "String")),
    };
}

impl Shapely for &str {
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: value_vtable!(&str, |f, _opts| write!(f, "&str")),
    };
}

impl Shapely for bool {
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: value_vtable!(bool, |f, _opts| write!(f, "bool")),
    };
}

macro_rules! impl_shapely_for_integer {
    ($type:ty) => {
        impl Shapely for $type {
            const SHAPE: &'static Shape = &Shape {
                layout: Layout::new::<Self>(),
                def: Def::Scalar(ScalarDef::of::<Self>()),
                vtable: value_vtable!($type, |f, _opts| write!(f, stringify!($type))),
            };
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
            const SHAPE: &'static Shape = &Shape {
                layout: Layout::new::<Self>(),
                def: Def::Scalar(ScalarDef::of::<Self>()),
                vtable: value_vtable!($type, |f, _opts| write!(f, stringify!($type))),
            };
        }
    };
}

impl_shapely_for_float!(f32);
impl_shapely_for_float!(f64);
