use std::{alloc::Layout, fmt};

use crate::{Innards, ListVTable, NameOpts, Shape, Shapely, mini_typeid};

impl<T> Shapely for Vec<T>
where
    T: Shapely,
{
    fn shape() -> Shape {
        fn name<T: Shapely>(f: &mut fmt::Formatter, opts: NameOpts) -> fmt::Result {
            if let Some(opts) = opts.for_children() {
                write!(f, "Vec<")?;
                let shape = T::shape();
                (shape.name)(f, opts)?;
                write!(f, ">")
            } else {
                write!(f, "Vec<…>")
            }
        }

        Shape {
            name: name::<T> as _,
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<Vec<T>>(),
            innards: Innards::List {
                vtable: ListVTable {
                    init: |ptr, size_hint| unsafe {
                        let vec = if let Some(capacity) = size_hint {
                            let layout = Layout::array::<T>(capacity).unwrap();
                            let ptr = std::alloc::alloc(layout) as *mut T;
                            if ptr.is_null() {
                                std::alloc::handle_alloc_error(layout);
                            }
                            Vec::from_raw_parts(ptr, 0, capacity)
                        } else {
                            Vec::<T>::new()
                        };
                        std::ptr::write(ptr as *mut Vec<T>, vec);
                    },
                    push: |ptr, partial| unsafe {
                        let vec = &mut *(ptr as *mut Vec<T>);
                        let item = partial.build();
                        vec.push(item);
                    },
                    len: |ptr| unsafe {
                        let vec = &*(ptr as *const Vec<T>);
                        vec.len()
                    },
                    get_item_ptr: |ptr, index| unsafe {
                        let vec = &*(ptr as *const Vec<T>);
                        if index >= vec.len() {
                            panic!(
                                "Index out of bounds: the len is {} but the index is {}",
                                vec.len(),
                                index
                            );
                        }
                        vec.as_ptr().add(index) as *const u8
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
