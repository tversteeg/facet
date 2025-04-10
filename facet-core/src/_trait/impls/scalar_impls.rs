use typeid::ConstTypeId;

use crate::value_vtable;
use crate::*;
use core::alloc::Layout;

unsafe impl Facet for ConstTypeId {
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .layout(Layout::new::<Self>())
            .def(Def::Scalar(ScalarDef::of::<Self>()))
            .vtable(value_vtable!((), |f, _opts| write!(f, "ConstTypeId")))
            .build()
    };
}

unsafe impl Facet for () {
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .layout(Layout::new::<Self>())
            .def(Def::Scalar(ScalarDef::of::<Self>()))
            .vtable(value_vtable!((), |f, _opts| write!(f, "()")))
            .build()
    };
}

unsafe impl Facet for String {
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .layout(Layout::new::<Self>())
            .def(Def::Scalar(ScalarDef::of::<Self>()))
            .vtable(value_vtable!(String, |f, _opts| write!(f, "String")))
            .build()
    };
}

unsafe impl Facet for &str {
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .layout(Layout::new::<Self>())
            .def(Def::Scalar(ScalarDef::of::<Self>()))
            .vtable(value_vtable!(&str, |f, _opts| write!(f, "&str")))
            .build()
    };
}

#[cfg(feature = "std")]
unsafe impl Facet for std::borrow::Cow<'_, str> {
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .layout(Layout::new::<Self>())
            .def(Def::Scalar(ScalarDef::of::<Self>()))
            .vtable(value_vtable!(std::borrow::Cow<'_, str>, |f, _opts| write!(
                f,
                "Cow<'_, str>"
            )))
            .build()
    };
}

unsafe impl Facet for bool {
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .layout(Layout::new::<Self>())
            .def(Def::Scalar(ScalarDef::of::<Self>()))
            .vtable(value_vtable!(bool, |f, _opts| write!(f, "bool")))
            .build()
    };
}

macro_rules! impl_facet_for_integer {
    ($type:ty) => {
        unsafe impl Facet for $type {
            const SHAPE: &'static Shape = &const {
                Shape::builder()
                    .layout(Layout::new::<Self>())
                    .def(Def::Scalar(ScalarDef::of::<Self>()))
                    .vtable(value_vtable!($type, |f, _opts| write!(
                        f,
                        stringify!($type)
                    )))
                    .build()
            };
        }
    };
}

impl_facet_for_integer!(u8);
impl_facet_for_integer!(i8);
impl_facet_for_integer!(u16);
impl_facet_for_integer!(i16);
impl_facet_for_integer!(u32);
impl_facet_for_integer!(i32);
impl_facet_for_integer!(u64);
impl_facet_for_integer!(i64);
impl_facet_for_integer!(u128);
impl_facet_for_integer!(i128);
impl_facet_for_integer!(usize);
impl_facet_for_integer!(isize);

macro_rules! impl_facet_for_float {
    ($type:ty) => {
        unsafe impl Facet for $type {
            const SHAPE: &'static Shape = &const {
                Shape::builder()
                    .layout(Layout::new::<Self>())
                    .def(Def::Scalar(ScalarDef::of::<Self>()))
                    .vtable(value_vtable!($type, |f, _opts| write!(
                        f,
                        stringify!($type)
                    )))
                    .build()
            };
        }
    };
}

impl_facet_for_float!(f32);
impl_facet_for_float!(f64);

#[cfg(feature = "std")]
unsafe impl Facet for std::net::SocketAddr {
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .layout(Layout::new::<Self>())
            .def(Def::Scalar(ScalarDef::of::<Self>()))
            .vtable(value_vtable!(std::net::SocketAddr, |f, _opts| write!(
                f,
                "SocketAddr"
            )))
            .build()
    };
}
