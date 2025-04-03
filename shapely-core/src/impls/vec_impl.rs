use crate::*;
use std::alloc::Layout;

impl<T> Shapely for Vec<T>
where
    T: Shapely,
{
    const SHAPE: &'static Shape = &const {
        Shape {
            layout: Layout::new::<Vec<T>>(),
            vtable: &ValueVTable {
                type_name: |f, opts| {
                    if let Some(opts) = opts.for_children() {
                        write!(f, "Vec<")?;
                        (T::SHAPE.vtable.type_name)(f, opts)?;
                        write!(f, ">")
                    } else {
                        write!(f, "Vec<⋯>")
                    }
                },
                // vecs don't have display in the shapeless cinematic universe
                display: None,
                debug: const {
                    if T::SHAPE.vtable.debug.is_some() {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<Vec<T>>() };
                            write!(f, "[")?;
                            for (i, item) in value.iter().enumerate() {
                                if i > 0 {
                                    write!(f, ", ")?;
                                }
                                unsafe {
                                    (T::SHAPE.vtable.debug.unwrap_unchecked())(
                                        OpaqueConst::from_ref(item),
                                        f,
                                    )?;
                                }
                            }
                            write!(f, "]")
                        })
                    } else {
                        None
                    }
                },
                eq: const {
                    if T::SHAPE.vtable.eq.is_some() {
                        Some(|a, b| unsafe {
                            let a = a.as_ref::<Vec<T>>();
                            let b = b.as_ref::<Vec<T>>();
                            if a.len() != b.len() {
                                return false;
                            }
                            for (item_a, item_b) in a.iter().zip(b.iter()) {
                                if !(T::SHAPE.vtable.eq.unwrap_unchecked())(
                                    OpaqueConst::from_ref(item_a),
                                    OpaqueConst::from_ref(item_b),
                                ) {
                                    return false;
                                }
                            }
                            true
                        })
                    } else {
                        None
                    }
                },
                // TODO: specialize these
                ord: None,
                // TODO: specialize these
                hash: None,
                drop_in_place: Some(|value| unsafe {
                    std::ptr::drop_in_place(value.as_mut_ptr::<Vec<T>>());
                }),
                parse: None,
                // TODO: specialize these
                try_from: None,
                default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
                clone_into: Some(|src, dst| unsafe { Some(dst.write(src.as_ref::<Vec<T>>())) }),
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
