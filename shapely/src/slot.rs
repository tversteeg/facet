use std::collections::HashMap;

use crate::{InitFieldSlot, ShapeDesc, Shapely};

enum Destination {
    /// Writes directly to an (uninitialized) struct field
    StructField { field_addr: *mut u8 },

    /// Inserts into a HashMap
    HashMap { map: *mut u8, key: String },
}

/// Allows writing into a struct field or inserting into a hash map.
pub struct Slot<'s> {
    /// where to write the value
    dest: Destination,

    /// shape of the field / hashmap value we're writing
    field_shape: ShapeDesc,

    /// lifetime marker
    init_field_slot: InitFieldSlot<'s>,
}

impl<'s> Slot<'s> {
    #[inline(always)]
    pub fn for_struct_field(
        field_addr: *mut u8,
        field_shape: ShapeDesc,
        init_field_slot: InitFieldSlot<'s>,
    ) -> Self {
        Self {
            dest: Destination::StructField { field_addr },
            field_shape,
            init_field_slot,
        }
    }

    #[inline(always)]
    pub fn for_hash_map(
        map: *mut u8,
        field_shape: ShapeDesc,
        key: String,
        init_field_slot: InitFieldSlot<'s>,
    ) -> Self {
        Self {
            dest: Destination::HashMap { map, key },
            field_shape,
            init_field_slot,
        }
    }

    pub fn fill<T: Shapely>(mut self, value: T) {
        if self.field_shape != T::shape_desc() {
            panic!(
                "Attempted to fill a field with an incompatible shape.\n\
                Expected shape: {:?}\n\
                Actual shape: {:?}\n\
                This is undefined behavior and we're refusing to proceed.",
                self.field_shape.get(),
                T::shape()
            );
        }

        unsafe {
            match self.dest {
                Destination::StructField { field_addr } => {
                    if self.init_field_slot.is_init() {
                        std::ptr::drop_in_place(field_addr as *mut T);
                    }
                    std::ptr::write(field_addr as *mut T, value);
                }
                Destination::HashMap { map, key } => {
                    let map = &mut *(map as *mut HashMap<String, T>);
                    map.insert(key, value);
                }
            }
        }
        self.init_field_slot.mark_as_init();
    }
}
