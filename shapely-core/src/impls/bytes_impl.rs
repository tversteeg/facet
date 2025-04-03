use crate::{Bytes, Def, ScalarDef, Shape, Shapely, value_vtable};

impl Shapely for Bytes {
    const DUMMY: Self = Bytes(Shapely::DUMMY);
    const SHAPE: &'static Shape = &Shape {
        layout: std::alloc::Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: value_vtable!((), |f, _opts| write!(f, "Bytes")),
    };
}
