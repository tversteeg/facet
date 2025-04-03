use std::alloc::Layout;

use crate::{DebugFn, Def, ListDef, ListVTable, OpaqueConst, Shape, Shapely, ValueVTable};

impl<T> Shapely for Vec<T>
where
    T: Shapely,
{
    const SHAPE: &'static Shape = &const {
        struct Wrap<T>(T);

        trait HasDebugFn {
            const DEBUG: Option<DebugFn>;
        }

        impl<T> HasDebugFn for &Wrap<Vec<T>> {
            const DEBUG: Option<DebugFn> = None;
        }

        impl<T: std::fmt::Debug> HasDebugFn for Wrap<Vec<T>> {
            const DEBUG: Option<DebugFn> =
                Some(|data, mut f| write!(f, "{:?}", unsafe { data.as_ref::<Vec<T>>() }));
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
                debug: <&Wrap<Vec<T>> as HasDebugFn>::DEBUG,
                default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
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
