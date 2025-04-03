use std::alloc::Layout;

use crate::{Def, ListDef, ListVTable, OpaqueConst, Shape, Shapely, value_vtable};

impl<T> Shapely for Vec<T>
where
    T: Shapely,
{
    const SHAPE: &'static Shape = &const {
        Shape {
            layout: Layout::new::<Vec<T>>(),
            vtable: value_vtable!(Self, |f, opts| {
                if let Some(opts) = opts.for_children() {
                    write!(f, "Vec<")?;
                    (T::SHAPE.vtable.type_name)(f, opts)?;
                    write!(f, ">")
                } else {
                    write!(f, "Vec<⋯>")
                }
            }),
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

impl<T> Shapely for &[T]
where
    T: Shapely,
{
    const SHAPE: &'static Shape = &const {
        Shape {
            layout: Layout::new::<&[T]>(),
            vtable: value_vtable!(Self, |f, opts| {
                if let Some(opts) = opts.for_children() {
                    write!(f, "&[")?;
                    (T::SHAPE.vtable.type_name)(f, opts)?;
                    write!(f, "]")
                } else {
                    write!(f, "&[⋯]")
                }
            }),
            def: Def::List(ListDef {
                vtable: &ListVTable {
                    init_in_place_with_capacity: |_, _| Err(()),
                    push: |_, _| {
                        panic!("Cannot push to &[T]");
                    },
                    len: |ptr| unsafe {
                        let slice = ptr.as_ref::<&[T]>();
                        slice.len()
                    },
                    get_item_ptr: |ptr, index| unsafe {
                        let slice = ptr.as_ref::<&[T]>();
                        let len = slice.len();
                        if index >= len {
                            panic!(
                                "Index out of bounds: the len is {len} but the index is {index}"
                            );
                        }
                        OpaqueConst::new_unchecked(slice.as_ptr().add(index))
                    },
                },
                t: T::SHAPE,
            }),
        }
    };
}

impl<T> Shapely for [T; 1]
where
    T: Shapely,
{
    const SHAPE: &'static Shape = &const {
        Shape {
            layout: Layout::new::<Self>(),
            vtable: value_vtable!(Self, |f, opts| {
                write!(f, "[")?;
                if let Some(opts) = opts.for_children() {
                    (T::SHAPE.vtable.type_name)(f, opts)?;
                } else {
                    write!(f, "⋯")?;
                }
                write!(f, "; 1]")
            }),
            def: Def::List(ListDef {
                vtable: &ListVTable {
                    init_in_place_with_capacity: |_, _| Err(()),
                    push: |_, _| {
                        panic!("Cannot push to [T; 1]");
                    },
                    len: |_| 1,
                    get_item_ptr: |ptr, index| unsafe {
                        if index >= 1 {
                            panic!("Index out of bounds: the len is 1 but the index is {index}");
                        }
                        OpaqueConst::new_unchecked(ptr.as_ptr::<[T; 1]>())
                    },
                },
                t: T::SHAPE,
            }),
        }
    };
}
