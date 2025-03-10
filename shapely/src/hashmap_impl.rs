use std::{alloc::Layout, collections::HashMap};

use crate::{mini_typeid, Innards, Shape, Shapely};

impl<V> Shapely for HashMap<String, V>
where
    V: Shapely,
{
    fn shape() -> Shape {
        Shape {
            name: "HashMap<String, V>",
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<HashMap<String, V>>(),
            innards: Innards::HashMap {
                value_shape: V::shape_desc(),
            },
            display: Some(hashmap_display::<V>),
            debug: Some(hashmap_debug::<V>),
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut HashMap<String, V>) = HashMap::new();
            }),
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut HashMap<String, V>);
            }),
        }
    }
}

fn hashmap_display<V: Shapely>(addr: *const u8, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let map = unsafe { &*(addr as *const HashMap<String, V>) };
    write!(f, "{{")?;
    let mut first = true;
    for (key, value) in map {
        if !first {
            write!(f, ", ")?;
        }
        write!(f, "{}: ", key)?;
        if let Some(display_fn) = V::shape().display {
            display_fn(value as *const _ as *const u8, f)?;
        } else {
            write!(f, "<no display>")?;
        }
        first = false;
    }
    write!(f, "}}")
}

fn hashmap_debug<V: Shapely>(addr: *const u8, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let map = unsafe { &*(addr as *const HashMap<String, V>) };
    write!(f, "HashMap {{")?;
    let mut first = true;
    for (key, value) in map {
        if !first {
            write!(f, ", ")?;
        }
        write!(f, "{:?}: ", key)?;
        if let Some(debug_fn) = V::shape().debug {
            debug_fn(value as *const _ as *const u8, f)?;
        } else {
            write!(f, "<no debug>")?;
        }
        first = false;
    }
    write!(f, "}}")
}
