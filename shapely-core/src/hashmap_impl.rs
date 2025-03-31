use std::{alloc::Layout, collections::HashMap, fmt};

use crate::{Innards, Shape, Shapely, mini_typeid};

impl<V> Shapely for HashMap<String, V>
where
    V: Shapely,
{
    fn shape() -> Shape {
        // This name function doesn't need the type parameter
        fn name<V: Shapely>(f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "HashMap<String, ")?;
            let shape = V::shape();
            (shape.name)(f)?;
            write!(f, ">")
        }

        Shape {
            name: name::<V> as _,
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<HashMap<String, V>>(),
            innards: Innards::HashMap {
                value_shape: V::shape_desc(),
                vtable: crate::HashMapVtable {
                    init: |ptr, size_hint| unsafe {
                        let map = if let Some(capacity) = size_hint {
                            HashMap::with_capacity(capacity)
                        } else {
                            HashMap::<String, V>::new()
                        };
                        std::ptr::write(ptr as *mut HashMap<String, V>, map);
                    },
                    insert: |ptr, key_partial, value_partial| unsafe {
                        let map = &mut *(ptr as *mut HashMap<String, V>);
                        let key = key_partial.build::<String>();
                        let value = value_partial.build::<V>();
                        map.insert(key, value);
                    },
                    len: |ptr| unsafe {
                        let map = &*(ptr as *const HashMap<String, V>);
                        map.len()
                    },
                    contains_key: |ptr, key| unsafe {
                        let map = &*(ptr as *const HashMap<String, V>);
                        map.contains_key(key)
                    },
                    get_value_ptr: |ptr, key| unsafe {
                        let map = &*(ptr as *const HashMap<String, V>);
                        if let Some(value) = map.get(key) {
                            value as *const V as *const u8
                        } else {
                            std::ptr::null()
                        }
                    },
                    iter: |ptr| unsafe {
                        let map = &*(ptr as *const HashMap<String, V>);
                        let entries: Vec<(String, *const V)> = map
                            .iter()
                            .map(|(k, v)| (k.clone(), v as *const V))
                            .collect();

                        let iter_state = Box::new((entries, 0usize));
                        Box::into_raw(iter_state) as *const u8
                    },
                    iter_vtable: crate::HashMapIterVtable {
                        next: |iter_ptr| unsafe {
                            let state = &mut *(iter_ptr as *mut (Vec<(String, *const V)>, usize));
                            let (entries, index) = state;

                            if *index < entries.len() {
                                let current = &entries[*index];
                                let key_ptr = &current.0 as *const String;
                                let value_ptr = current.1 as *const u8;
                                *index += 1;
                                Some((key_ptr, value_ptr))
                            } else {
                                None
                            }
                        },
                        dealloc: |iter_ptr| unsafe {
                            drop(Box::from_raw(
                                iter_ptr as *mut (Vec<(String, *const V)>, usize),
                            ));
                        },
                    },
                },
            },
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut HashMap<String, V>) = HashMap::new();
            }),
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut HashMap<String, V>);
            }),
        }
    }
}
