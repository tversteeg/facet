use std::{
    alloc::Layout,
    collections::{HashMap, VecDeque},
    fmt,
};

use crate::{
    Def, MapDef, MapVTable, OpaqueConst, Shape, Shapely, TypeNameOpts, ValueVTable, mini_typeid,
};

impl<K, V> Shapely for HashMap<K, V>
where
    K: Shapely + std::cmp::Eq + std::hash::Hash + 'static,
    V: Shapely + 'static,
{
    fn shape() -> Shape {
        fn type_name<K: Shapely, V: Shapely>(
            f: &mut fmt::Formatter,
            opts: TypeNameOpts,
        ) -> fmt::Result {
            if let Some(opts) = opts.for_children() {
                write!(f, "HashMap<")?;
                (K::shape().vtable().type_name)(f, opts)?;
                write!(f, ", ")?;
                (V::shape().vtable().type_name)(f, opts)?;
                write!(f, ">")
            } else {
                write!(f, "HashMap<…>")
            }
        }

        struct Iterator<'mem, K> {
            map: OpaqueConst<'mem>,
            keys: VecDeque<&'mem K>,
        }

        Shape {
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<HashMap<K, V>>(),
            vtable: || ValueVTable {
                type_name: type_name::<K, V> as _,
                display: None,
                debug: None,
                default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
                eq: None,
                cmp: None,
                hash: None,
                drop_in_place: Some(|value| unsafe {
                    std::ptr::drop_in_place(value.as_mut_ptr::<HashMap<K, V>>());
                }),
                parse: None,
                try_from: None,
            },
            def: Def::Map(MapDef {
                k: K::SHAPE_FN,
                v: V::SHAPE_FN,
                vtable: || MapVTable {
                    init_in_place_with_capacity: |uninit, capacity| unsafe {
                        Ok(uninit.write(Self::with_capacity(capacity)))
                    },
                    insert: |ptr, key, value| unsafe {
                        let map = ptr.as_mut_ptr::<HashMap<K, V>>();
                        let key = key.read::<K>();
                        let value = value.read::<V>();
                        map.insert(key, value);
                    },
                    len: |ptr| unsafe {
                        let map = ptr.as_ref::<HashMap<K, V>>();
                        map.len()
                    },
                    contains_key: |ptr, key| unsafe {
                        let map = ptr.as_ref::<HashMap<K, V>>();
                        map.contains_key(key.as_ref())
                    },
                    get_value_ptr: |ptr, key| unsafe {
                        let map = ptr.as_ref::<HashMap<K, V>>();
                        map.get(key.as_ref())
                            .map(|v| OpaqueConst::new_unchecked(v as *const _))
                    },
                    iter: |ptr| unsafe {
                        let map = ptr.as_ref::<HashMap<K, V>>();
                        let keys: VecDeque<&K> = map.keys().collect();
                        let iter_state = Box::new(Iterator { map: ptr, keys });
                        OpaqueConst::new_unchecked(Box::into_raw(iter_state) as *mut u8)
                    },
                    iter_vtable: crate::MapIterVTable {
                        next: |iter_ptr| unsafe {
                            let state = iter_ptr.as_mut_ptr::<Iterator<'_, K>>();
                            let map = state.map.as_ref::<HashMap<K, V>>();

                            while let Some(key) = state.keys.pop_front() {
                                if let Some(value) = map.get(key) {
                                    return Some((
                                        OpaqueConst::new_unchecked(key as *const K),
                                        OpaqueConst::new_unchecked(value as *const V),
                                    ));
                                }
                            }

                            None
                        },
                        dealloc: |iter_ptr| unsafe {
                            drop(Box::from_raw(
                                iter_ptr.as_ptr::<Iterator<'_, K>>() as *mut Iterator<'_, K>
                            ));
                        },
                    },
                },
            }),
        }
    }
}
