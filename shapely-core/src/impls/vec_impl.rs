use std::{alloc::Layout, fmt};

use crate::{
    Innards, ListInnards, ListVTable, OpaqueConst, Shape, Shapely, TypeNameOpts, ValueVTable,
    mini_typeid,
};

impl<T> Shapely for Vec<T>
where
    T: Shapely,
{
    fn shape() -> Shape {
        fn type_name<T: Shapely>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result {
            if let Some(opts) = opts.for_children() {
                write!(f, "Vec<")?;
                let shape = T::shape();
                (shape.vtable().type_name)(f, opts)?;
                write!(f, ">")
            } else {
                write!(f, "Vec<…>")
            }
        }

        Shape {
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<Vec<T>>(),
            vtable: || ValueVTable {
                type_name: type_name::<T> as _,
                // TODO: specialize these
                display: None,
                // TODO: specialize these
                debug: None,
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
            innards: Innards::List(ListInnards {
                vtable: ListVTable {
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
                        if index >= vec.len() {
                            panic!(
                                "Index out of bounds: the len is {} but the index is {}",
                                vec.len(),
                                index
                            );
                        }
                        OpaqueConst::new_unchecked(vec.as_ptr().add(index))
                    },
                },
                t: T::shape_desc(),
            }),
        }
    }
}
