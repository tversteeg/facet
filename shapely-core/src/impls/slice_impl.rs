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
