use std::{alloc::Layout, fmt};

use crate::{mini_typeid, ArrayVtable, Innards, Shape, Shapely};

impl<T> Shapely for Vec<T>
where
    T: Shapely,
{
    fn shape() -> Shape {
        fn name<T: Shapely>(f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Vec<")?;
            let shape = T::shape();
            (shape.name)(f)?;
            write!(f, ">")
        }

        Shape {
            name: name::<T> as _,
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<Vec<T>>(),
            innards: Innards::Array {
                vtable: ArrayVtable {
                    init: |ptr, size_hint| unsafe {
                        let vec = if let Some(capacity) = size_hint {
                            Vec::<T>::with_capacity(capacity)
                        } else {
                            Vec::<T>::new()
                        };
                        *(ptr as *mut Vec<T>) = vec;
                    },
                    push: |ptr, partial| unsafe {
                        let vec = &mut *(ptr as *mut Vec<T>);
                        let item = partial.build();
                        vec.push(item);
                    },
                },
                item_shape: T::shape_desc(),
            },
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut Vec<T>) = Vec::new();
            }),
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut Vec<T>);
            }),
        }
    }
}
