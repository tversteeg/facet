use std::{
    alloc::Layout,
    collections::{HashMap, VecDeque},
    fmt,
};

use crate::{
    Innards, MapVTable, OpaqueConst, Shape, Shapely, TypeNameOpts, ValueVTable, mini_typeid,
};

impl<V> Shapely for HashMap<String, V>
where
    V: Shapely + 'static,
{
    fn shape() -> Shape {
        // This name function doesn't need the type parameter
        fn type_name<V: Shapely>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result {
            if let Some(opts) = opts.for_children() {
                write!(f, "HashMap<String, ")?;
                (V::shape().vtable.type_name)(f, opts)?;
                write!(f, ">")
            } else {
                write!(f, "HashMap<…>")
            }
        }

        struct Iterator<'mem> {
            map: OpaqueConst<'mem>,
            keys: VecDeque<String>,
        }

        Shape {
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<HashMap<String, V>>(),
            vtable: ValueVTable {
                type_name: type_name::<V> as _,
                display: None,
                // TODO: at some point, specialize: we can Debug if K or V have Debug in its shape.
                debug: None,
                default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
                // TODO: re-implement eq, (only if K & V implement Eq)
                eq: None,
                cmp: None, // HashMap doesn't have a natural ordering
                // TODO: re-implement hash, (only if K & V implement Hash)
                hash: None,
                drop_in_place: Some(|value| unsafe {
                    std::ptr::drop_in_place(value.as_mut_ptr::<HashMap<String, V>>());
                }),
            },
            innards: Innards::Map {
                value_shape: V::shape_desc(),
                vtable: MapVTable {
                    init: |ptr, size_hint| unsafe {
                        ptr.write(if let Some(capacity) = size_hint {
                            HashMap::with_capacity(capacity)
                        } else {
                            HashMap::<String, V>::new()
                        });
                    },
                    insert: |ptr, key_partial, value_partial| unsafe {
                        let map = ptr.as_mut_ptr::<HashMap<String, V>>();
                        let key = key_partial.build::<String>();
                        let value = value_partial.build::<V>();
                        map.insert(key, value);
                    },
                    len: |ptr| unsafe {
                        let map = ptr.as_ref::<HashMap<String, V>>();
                        map.len()
                    },
                    contains_key: |ptr, key| unsafe {
                        let map = ptr.as_ref::<HashMap<String, V>>();
                        map.contains_key(key)
                    },
                    get_value_ptr: |ptr, key| unsafe {
                        let map = ptr.as_ref::<HashMap<String, V>>();
                        map.get(key)
                            .map(|v| OpaqueConst::new_unchecked(v as *const _))
                    },
                    iter: |ptr| unsafe {
                        let map = ptr.as_ref::<HashMap<String, V>>();
                        let keys: VecDeque<String> = map.keys().cloned().collect();
                        let iter_state = Box::new(Iterator { map: ptr, keys });
                        OpaqueConst::new_unchecked(Box::into_raw(iter_state) as *mut u8)
                    },
                    iter_vtable: crate::MapIterVTable {
                        next: |iter_ptr| unsafe {
                            let state = iter_ptr.as_mut_ptr::<Iterator>();
                            let map = state.map.as_ref::<HashMap<String, V>>();

                            while let Some(key) = state.keys.pop_front() {
                                if let Some(value) = map.get(&key) {
                                    return Some((
                                        &key as *const String,
                                        OpaqueConst::from_ref(value),
                                    ));
                                }
                            }

                            None
                        },
                        dealloc: |iter_ptr| unsafe {
                            drop(Box::from_raw(iter_ptr.as_ptr::<Iterator>() as *mut Iterator));
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
