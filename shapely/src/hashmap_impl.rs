use std::{
    collections::HashMap,
    marker::PhantomData,
    mem::{self},
};

use crate::{Innards, MapField, MapInnards, Shape, ShapeUninit, Shapely, Slot, Slots};

impl<V> Shapely for HashMap<String, V>
where
    V: Shapely + Send + Sync + 'static,
{
    fn shape() -> Shape {
        fn mk_hashmap_manipulator<V: Shapely + Send + Sync + 'static>(
            shape: Shape,
        ) -> &'static dyn Slots {
            Box::leak(Box::new(HashMapManipulator {
                _phantom: PhantomData::<V>,
                map_shape: Box::leak(Box::new(shape)),
            }))
        }

        Shape {
            name: "HashMap<String, V>",
            size: mem::size_of::<HashMap<String, V>>(),
            align: mem::align_of::<HashMap<String, V>>(),
            innards: Innards::Map(
                MapInnards::builder()
                    .open_ended(true)
                    .mk_slots(mk_hashmap_manipulator::<V>)
                    .build(),
            ),
            display: None,
            debug: None,
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut HashMap<String, V>) = HashMap::new();
            }),
        }
    }
}
