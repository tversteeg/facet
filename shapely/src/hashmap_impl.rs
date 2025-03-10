use std::{
    collections::HashMap,
    marker::PhantomData,
    mem::{self, MaybeUninit},
};

use crate::{Innards, MapField, MapInnards, Shape, ShapeUninit, Shapely, Slot, Slots};

impl<V> Shapely for HashMap<String, V>
where
    V: Shapely + Send + Sync + 'static,
{
    fn shape() -> Shape {
        struct HashMapManipulator<V> {
            _phantom: PhantomData<V>,
            map_shape: Shape,
        }

        impl<V> Slots for HashMapManipulator<V>
        where
            V: Shapely + Send + Sync + 'static,
        {
            fn slot<'a>(
                &'a mut self,
                map: &'a mut ShapeUninit,
                field: MapField<'_>,
            ) -> Option<Slot<'a>> {
                unsafe {
                    let map: *mut HashMap<String, MaybeUninit<V>> = map.get_addr(&self.map_shape);
                    Some(Slot::for_hash_map(
                        map as *mut HashMap<String, V>,
                        field.name.to_string(),
                    ))
                }
            }
        }

        fn mk_hashmap_manipulator<V>(shape: Shape) -> HashMapManipulator<V> {
            HashMapManipulator {
                _phantom: PhantomData::<V>,
                map_shape: shape,
            }
        }

        Shape {
            name: "HashMap<String, V>",
            size: mem::size_of::<HashMap<String, V>>(),
            align: mem::align_of::<HashMap<String, V>>(),
            innards: Innards::Map(MapInnards {
                fields: &[],
                open_ended: true,
                mk_slots: mk_hashmap_manipulator::<V>,
            }),
            display: None,
            debug: None,
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut HashMap<String, V>) = HashMap::new();
            }),
        }
    }
}
