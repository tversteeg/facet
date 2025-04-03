use crate::*;
use std::alloc::Layout;

impl<T> Shapely for &[T]
where
    T: Shapely,
{
    const SHAPE: &'static Shape = &const {
        Shape {
            layout: Layout::new::<&[T]>(),
            vtable: &ValueVTable {
                type_name: |f, opts| {
                    if let Some(opts) = opts.for_children() {
                        write!(f, "&[")?;
                        (T::SHAPE.vtable.type_name)(f, opts)?;
                        write!(f, "]")
                    } else {
                        write!(f, "&[⋯]")
                    }
                },
                display: None,
                debug: const {
                    if T::SHAPE.vtable.debug.is_some() {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<&[T]>() };
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
                eq: if T::SHAPE.vtable.eq.is_some() {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<&[T]>() };
                        let b = unsafe { b.as_ref::<&[T]>() };
                        if a.len() != b.len() {
                            return false;
                        }
                        for (x, y) in a.iter().zip(b.iter()) {
                            if !unsafe {
                                (T::SHAPE.vtable.eq.unwrap_unchecked())(
                                    OpaqueConst::from_ref(x),
                                    OpaqueConst::from_ref(y),
                                )
                            } {
                                return false;
                            }
                        }
                        true
                    })
                } else {
                    None
                },
                cmp: if T::SHAPE.vtable.cmp.is_some() {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<&[T]>() };
                        let b = unsafe { b.as_ref::<&[T]>() };
                        for (x, y) in a.iter().zip(b.iter()) {
                            let cmp = unsafe {
                                (T::SHAPE.vtable.cmp.unwrap_unchecked())(
                                    OpaqueConst::from_ref(x),
                                    OpaqueConst::from_ref(y),
                                )
                            };
                            if cmp != std::cmp::Ordering::Equal {
                                return cmp;
                            }
                        }
                        a.len().cmp(&b.len())
                    })
                } else {
                    None
                },
                hash: if T::SHAPE.vtable.hash.is_some() {
                    Some(|value, state, hasher| {
                        let value = unsafe { value.as_ref::<&[T]>() };
                        for item in value.iter() {
                            unsafe {
                                (T::SHAPE.vtable.hash.unwrap_unchecked())(
                                    OpaqueConst::from_ref(item),
                                    state,
                                    hasher,
                                )
                            };
                        }
                    })
                } else {
                    None
                },
                drop_in_place: Some(|ptr| {
                    // No need to drop a &[T] as it's just a reference
                    // The actual slice data is not owned by this reference
                    let _ = unsafe { ptr.as_ref::<&[T]>() };
                }),
                parse: None,
                try_from: None,
                default_in_place: None,
                clone_in_place: Some(|src, dst| unsafe { Some(dst.write(src.as_ref::<&[T]>())) }),
            },
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
