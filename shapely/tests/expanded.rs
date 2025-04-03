#[automatically_derived]
impl shapely::Shapely for TupleNoTraits {
    const SHAPE: &'static shapely::Shape = &const {
        shapely::Shape {
            layout: std::alloc::Layout::new::<Self>(),
            vtable: &shapely::ValueVTable {
                type_name: |f, _opts| std::fmt::Write::write_str(f, "TupleNoTraits"),
                display: if shapely_core::impls!(Self:std::fmt::Display) {
                    Some(|data, f| {
                        use shapely::spez::*;
                        (&&Spez(unsafe { data.as_ref::<Self>() })).spez_display(f)
                    })
                } else {
                    None
                },
                debug: if shapely_core::impls!(Self:std::fmt::Debug) {
                    Some(|data, f| {
                        use shapely::spez::*;
                        (&&Spez(unsafe { data.as_ref::<Self>() })).spez_debug(f)
                    })
                } else {
                    None
                },
                default_in_place: if shapely_core::impls!(Self:std::default::Default) {
                    Some(|data| unsafe {
                        data.write(<Self as std::default::Default>::default());
                    })
                } else {
                    None
                },
                clone_in_place: if shapely_core::impls!(Self:std::clone::Clone) {
                    Some(|src, dst| unsafe {
                        Some(dst.write(<Self as std::clone::Clone>::clone(src.as_ref::<Self>())))
                    })
                } else {
                    None
                },
                eq: if shapely_core::impls!(Self:std::cmp::PartialEq) {
                    Some(|left, right| {
                        use shapely::spez::*;
                        (&&Spez(unsafe { left.as_ref::<Self>() }))
                            .spez_eq(&&Spez(unsafe { right.as_ref::<Self>() }))
                    })
                } else {
                    None
                },
                cmp: if shapely_core::impls!(Self:std::cmp::Ord) {
                    Some(|left, right| {
                        use shapely::spez::*;
                        (&&Spez(unsafe { left.as_ref::<Self>() }))
                            .spez_cmp(&&Spez(unsafe { right.as_ref::<Self>() }))
                    })
                } else {
                    None
                },
                hash: if shapely_core::impls!(Self:std::hash::Hash) {
                    Some(|value, hasher_this, hasher_write_fn| {
                        use shapely::spez::*;
                        use shapely::vtable::HasherProxy;
                        (&&Spez(unsafe { value.as_ref::<Self>() })).spez_hash(&mut unsafe {
                            HasherProxy::new(hasher_this, hasher_write_fn)
                        })
                    })
                } else {
                    None
                },
                drop_in_place: Some(|data| unsafe { data.drop_in_place::<Self>() }),
                parse: None,
                try_from: None,
            },
            def: shapely::Def::TupleStruct(shapely::StructDef {
                fields: shapely::struct_fields!(TupleNoTraits, (0, 1)),
            }),
        }
    };
}
