use std::{
    alloc::Layout,
    collections::{HashMap, VecDeque},
};

use crate::{Def, MapDef, MapIterVTable, MapVTable, OpaqueConst, Shape, Shapely, ValueVTable};

struct HashMapIterator<'mem, K> {
    map: OpaqueConst<'mem>,
    keys: VecDeque<&'mem K>,
}

impl<K, V> Shapely for HashMap<K, V>
where
    K: Shapely + std::cmp::Eq + std::hash::Hash + 'static,
    V: Shapely + 'static,
{
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<HashMap<K, V>>(),
        vtable: &ValueVTable {
            type_name: |f, opts| {
                if let Some(opts) = opts.for_children() {
                    write!(f, "HashMap<")?;
                    (K::SHAPE.vtable.type_name)(f, opts)?;
                    write!(f, ", ")?;
                    (V::SHAPE.vtable.type_name)(f, opts)?;
                    write!(f, ">")
                } else {
                    write!(f, "HashMap<…>")
                }
            },
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
            k: K::SHAPE,
            v: V::SHAPE,
            vtable: &MapVTable {
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
                    let iter_state = Box::new(HashMapIterator { map: ptr, keys });
                    OpaqueConst::new_unchecked(Box::into_raw(iter_state) as *mut u8)
                },
                iter_vtable: MapIterVTable {
                    next: |iter_ptr| unsafe {
                        let state = iter_ptr.as_mut_ptr::<HashMapIterator<'_, K>>();
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
                        drop(Box::from_raw(iter_ptr.as_ptr::<HashMapIterator<'_, K>>()
                            as *mut HashMapIterator<'_, K>));
                    },
                },
            },
        }),
    };
}
