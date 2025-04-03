use std::hash::Hash as _;

use crate::{Bytes, Def, HasherProxy, ScalarDef, Shape, Shapely, ValueVTable};

impl Shapely for Bytes {
    const SHAPE: &'static Shape = &Shape {
        layout: std::alloc::Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: &ValueVTable {
            type_name: |f, _opts| write!(f, "Bytes"),
            display: None,
            debug: None,
            default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
            eq: Some(|left, right| unsafe { left.as_ref::<Self>() == right.as_ref::<Self>() }),
            cmp: Some(|left, right| unsafe { left.as_ref::<Self>().cmp(right.as_ref::<Self>()) }),
            hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
                value
                    .as_ref::<Self>()
                    .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
            }),
            drop_in_place: Some(|value| unsafe {
                std::ptr::drop_in_place(value.as_mut_ptr::<Self>());
            }),
            parse: None,
            try_from: None,
        },
    };
}
