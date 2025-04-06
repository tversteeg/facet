use crate::value_vtable;
use crate::*;
use std::alloc::Layout;
use std::borrow::Cow;

unsafe impl Facet for () {
    const DUMMY: Self = ();
    const SHAPE: &'static Shape = &const {
        Shape {
            layout: Layout::new::<Self>(),
            def: Def::Scalar(ScalarDef::of::<Self>()),
            vtable: value_vtable!((), |f, _opts| write!(f, "()")),
        }
    };
}

unsafe impl Facet for String {
    const DUMMY: Self = String::new();
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: value_vtable!(String, |f, _opts| write!(f, "String")),
    };
}

unsafe impl Facet for &str {
    const DUMMY: Self = "";
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: value_vtable!(&str, |f, _opts| write!(f, "&str")),
    };
}

unsafe impl Facet for Cow<'_, str> {
    const DUMMY: Self = Cow::Borrowed("");
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: value_vtable!(Cow<'_, str>, |f, _opts| write!(f, "Cow<'_, str>")),
    };
}

unsafe impl Facet for bool {
    const DUMMY: Self = false;
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: value_vtable!(bool, |f, _opts| write!(f, "bool")),
    };
}

macro_rules! impl_facet_for_integer {
    ($type:ty) => {
        unsafe impl Facet for $type {
            const DUMMY: Self = 0;
            const SHAPE: &'static Shape = &Shape {
                layout: Layout::new::<Self>(),
                def: Def::Scalar(ScalarDef::of::<Self>()),
                vtable: value_vtable!($type, |f, _opts| write!(f, stringify!($type))),
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

macro_rules! impl_facet_for_float {
    ($type:ty) => {
        unsafe impl Facet for $type {
            const DUMMY: Self = 0.0;
            const SHAPE: &'static Shape = &Shape {
                layout: Layout::new::<Self>(),
                def: Def::Scalar(ScalarDef::of::<Self>()),
                vtable: value_vtable!($type, |f, _opts| write!(f, stringify!($type))),
            };
        }
    };
}

impl_facet_for_float!(f32);
impl_facet_for_float!(f64);
