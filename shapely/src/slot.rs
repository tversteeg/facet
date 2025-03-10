use std::{collections::HashMap, marker::PhantomData};

use crate::{Shape, Shapely};

/// Type alias for user data pointer
type UserData = *mut u8;

/// Type alias for destination pointer
type StructField = *mut u8;

pub enum Destination {
    /// Writes directly to an (uninitialized) struct field
    StructField { field_addr: *mut u8 },

    /// Inserts into a HashMap
    HashMap { map: *mut u8, key: String },
}

/// Allows filling in a field of a struct, or inserting a value into a hashmap while deserializing.
pub struct Slot<'s> {
    /// Where to write the value
    dest: Destination,

    // shape of the struct we're assigning / the value in the hashmap
    field_shape: Shape,

    _phantom: PhantomData<&'s mut ()>,
}

impl Slot<'_> {
    /// Construct a new `FieldSlot` for a struct field, ready to be filled
    #[inline(always)]
    pub fn for_struct_field<TField: Shapely>(field_addr: *mut TField) -> Self {
        Self {
            dest: Destination::StructField {
                field_addr: field_addr as *mut u8,
            },
            field_shape: TField::shape(),
            _phantom: PhantomData,
        }
    }

    /// Construct a new `FieldSlot` for a HashMap entry, ready to be filled
    #[inline(always)]
    pub fn for_hash_map<TField: Shapely>(map: *mut HashMap<String, TField>, key: String) -> Self {
        Self {
            dest: Destination::HashMap {
                map: map as *mut u8,
                key,
            },
            field_shape: TField::shape(),
            _phantom: PhantomData,
        }
    }

    /// Fill this field with a value.
    pub fn fill<T: Shapely>(self, value: T) {
        let value_shape = T::shape();
        if self.field_shape != value_shape {
            panic!(
                "Attempted to fill a field with an incompatible shape.\n\
                Expected shape: {:?}\n\
                Actual shape: {:?}\n\
                This is undefined behavior and we're refusing to proceed.",
                self.field_shape, value_shape
            );
        }

        unsafe {
            match self.dest {
                Destination::StructField { field_addr } => {
                    std::ptr::write(field_addr as *mut T, value);
                }
                Destination::HashMap { map, key } => {
                    let map = &mut *(map as *mut HashMap<String, T>);
                    map.insert(key, value);
                }
            }
        }
    }
}
