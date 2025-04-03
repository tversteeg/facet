use crate::*;
use std::alloc::Layout;

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
