use std::{
    collections::HashMap,
    mem::{self},
};

use crate::{Innards, MapInnards, Shape, Shapely};

impl<V> Shapely for HashMap<String, V>
where
    V: Shapely + Send + Sync + 'static,
{
    fn shape() -> Shape {
        Shape {
            name: "HashMap<String, V>",
            size: mem::size_of::<HashMap<String, V>>(),
            align: mem::align_of::<HashMap<String, V>>(),
            innards: Innards::Map(MapInnards::for_hashmap()),
            display: None,
            debug: None,
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut HashMap<String, V>) = HashMap::new();
            }),
        }
    }
}
