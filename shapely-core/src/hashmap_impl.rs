use std::{alloc::Layout, collections::HashMap, fmt};

use crate::{mini_typeid, Innards, Shape, Shapely};

impl<V> Shapely for HashMap<String, V>
where
    V: Shapely,
{
    fn shape() -> Shape {
        // This name function doesn't need the type parameter
        fn name(shape: &Shape, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "HashMap<String, ")?;
            
            // Get the value shape from the innards
            if let Innards::HashMap { value_shape } = &shape.innards {
                let value_shape_obj = value_shape.get();
                // Use the shape's name function to write its name
                (value_shape_obj.name)(&value_shape_obj, f)?;
            } else {
                write!(f, "?")?;
            }
            
            write!(f, ">")
        }

        Shape {
            name,
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
