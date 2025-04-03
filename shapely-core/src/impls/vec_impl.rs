use std::alloc::Layout;

use crate::{Def, ListDef, ListVTable, OpaqueConst, Shape, Shapely, ValueVTable};

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
                display: const {
                    if T::SHAPE.vtable.display.is_some() {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<Vec<T>>() };
                            write!(f, "[")?;
                            for (i, item) in value.iter().enumerate() {
                                if i > 0 {
                                    write!(f, ", ")?;
                                }
                                unsafe {
                                    (T::SHAPE.vtable.display.unwrap_unchecked())(
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
                eq: None,
                cmp: None,
                hash: None,
                drop_in_place: None,
                parse: None,
                try_from: None,
                default_in_place: None,
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

impl<T> Shapely for [T; 1]
where
    T: Shapely,
{
    const SHAPE: &'static Shape = &const {
        Shape {
            layout: Layout::new::<[T; 1]>(),
            vtable: &ValueVTable {
                type_name: |f, opts| {
                    if let Some(opts) = opts.for_children() {
                        write!(f, "[")?;
                        (T::SHAPE.vtable.type_name)(f, opts)?;
                        write!(f, "; 1]")
                    } else {
                        write!(f, "[⋯; 1]")
                    }
                },
                display: None,
                debug: const {
                    if T::SHAPE.vtable.debug.is_some() {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<[T; 1]>() };
                            write!(f, "[")?;
                            unsafe {
                                (T::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value[0]),
                                    f,
                                )?;
                            }
                            write!(f, "]")
                        })
                    } else {
                        None
                    }
                },
                eq: None,
                cmp: None,
                hash: None,
                drop_in_place: Some(|value| unsafe {
                    std::ptr::drop_in_place(value.as_mut_ptr::<[T; 1]>());
                }),
                parse: None,
                try_from: None,
                default_in_place: if T::SHAPE.vtable.default_in_place.is_some() {
                    Some(|target| unsafe {
                        let t_dip = T::SHAPE.vtable.default_in_place.unwrap_unchecked();
                        (t_dip)(target.field_uninit(0))
                    })
                } else {
                    None
                },
            },
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
