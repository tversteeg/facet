use std::{
    alloc::Layout,
    fmt::{self},
};

use crate::{Def, ListDef, ListVTable, OpaqueConst, Shape, Shapely, ValueVTable};

impl<T> Shapely for Vec<T>
where
    T: Shapely,
{
    const SHAPE: &'static Shape = &const {
        /// proxies, using the <https://docs.rs/spez> trick, kind of:
        pub struct Wrap<T>(T);

        pub trait ViaDebug {
            fn debug_fn(&self, f: &mut fmt::Formatter<'_>) -> Option<Result<(), fmt::Error>>;
        }

        pub trait ViaNone {
            fn debug_fn(&self, f: &mut fmt::Formatter<'_>) -> Option<Result<(), fmt::Error>>;
        }

        impl<T: fmt::Debug> ViaDebug for &Wrap<T> {
            fn debug_fn(&self, f: &mut fmt::Formatter<'_>) -> Option<Result<(), fmt::Error>> {
                eprintln!("yes Debug");
                Some(fmt::Debug::fmt(&self.0, f))
            }
        }

        impl<T> ViaNone for Wrap<T> {
            fn debug_fn(&self, _f: &mut fmt::Formatter<'_>) -> Option<Result<(), fmt::Error>> {
                eprintln!("no Debug");
                None
            }
        }

        Shape {
            layout: Layout::new::<Vec<T>>(),
            vtable: &ValueVTable {
                type_name: |f, opts| {
                    if let Some(opts) = opts.for_children() {
                        write!(f, "Vec<")?;
                        (T::SHAPE.vtable.type_name)(f, opts)?;
                        write!(f, ">")
                    } else {
                        write!(f, "Vec<…>")
                    }
                },
                // TODO: specialize these
                display: None,
                debug: |value, f| {
                    let val = Wrap(value);
                    val.debug_fn(f)
                },
                // TODO: specialize these
                eq: None,
                // TODO: specialize these
                cmp: None,
                // TODO: specialize these
                hash: None,
                drop_in_place: Some(|value| unsafe {
                    std::ptr::drop_in_place(value.as_mut_ptr::<Vec<T>>());
                }),
                parse: None,
                // TODO: specialize these
                try_from: None,
                default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
            },
            def: Def::List(ListDef {
                vtable: &ListVTable {
                    init_in_place_with_capacity: |data, capacity| unsafe {
                        Ok(data.write(Self::with_capacity(capacity)))
                    },
                    push: |ptr, item| unsafe {
                        let vec = ptr.as_mut_ptr::<Vec<T>>();
                        let item = item.read::<T>();
                        (*vec).push(item);
                    },
                    len: |ptr| unsafe {
                        let vec = ptr.as_ref::<Vec<T>>();
                        vec.len()
                    },
                    get_item_ptr: |ptr, index| unsafe {
                        let vec = ptr.as_ref::<Vec<T>>();
                        let len = vec.len();
                        if index >= len {
                            panic!(
                                "Index out of bounds: the len is {len} but the index is {index}"
                            );
                        }
                        OpaqueConst::new_unchecked(vec.as_ptr().add(index))
                    },
                },
                t: T::SHAPE,
            }),
        }
    };
}
