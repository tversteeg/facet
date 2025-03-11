use std::{alloc::Layout, collections::HashMap, fmt};

use crate::{mini_typeid, Innards, Shape, Shapely};

impl<V> Shapely for HashMap<String, V>
where
    V: Shapely,
{
    fn shape() -> Shape {
        // This name function doesn't need the type parameter
        fn name<V: Shapely>(_shape: &Shape, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "HashMap<String, ")?;
            let shape = V::shape();
            (shape.name)(&shape, f)?;
            write!(f, ">")
        }

        Shape {
            name: name::<V>,
            typeid: mini_typeid::of::<Self>(),
            layout: Layout::new::<HashMap<String, V>>(),
            innards: Innards::HashMap {
                value_shape: V::shape_desc(),
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
