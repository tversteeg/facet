use std::hash::Hash as _;

use crate::{Bytes, HasherProxy, Innards, Shape, Shapely, ValueVTable, mini_typeid};

impl Shapely for Bytes {
    fn shape() -> Shape {
        Shape {
            typeid: mini_typeid::of::<Self>(),
            layout: std::alloc::Layout::new::<Self>(),
            innards: Innards::Scalar,
            vtable: || ValueVTable {
                type_name: |f, _opts| write!(f, "Bytes"),
                display: None,
                // now would be a good time to be opinionated I guess.
                debug: None,
                default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
                eq: Some(|left, right| unsafe { left.as_ref::<Self>() == right.as_ref::<Self>() }),
                cmp: Some(|left, right| unsafe {
                    left.as_ref::<Self>().cmp(right.as_ref::<Self>())
                }),
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
        }
    }
}
