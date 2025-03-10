use std::{
    collections::HashMap,
    marker::PhantomData,
    mem::{self, MaybeUninit},
};

use crate::{FieldSlot, Innards, MapField, MapInnards, Shape, ShapeUninit, Shapely, Slots};

impl<V> Shapely for HashMap<String, V>
where
    V: Shapely + Send + Sync + 'static,
{
    fn shape() -> Shape {
        struct HashMapManipulator<V> {
            _phantom: PhantomData<V>,
            shape: Shape,
        };

        impl<V> Slots for HashMapManipulator<V>
        where
            V: Shapely + Send + Sync + 'static,
        {
            fn slot<'a>(
                &self,
                map: &mut ShapeUninit,
                field: MapField<'_>,
            ) -> Option<FieldSlot<'a>> {
                unsafe {
                    let map: *mut HashMap<String, MaybeUninit<V>> = map.get_addr(&self.shape);
                    Some(FieldSlot::new(
                        (*map)
                            .entry(field.name.to_string())
                            .or_insert_with(|| MaybeUninit::uninit())
                            .as_mut_ptr(),
                    ))
                }
            }
        }

        fn mk_hashmap_manipulator<V>() -> HashMapManipulator<V> {
            HashMapManipulator {
                _phantom: PhantomData::<V>,
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
