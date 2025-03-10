use std::{collections::HashMap, marker::PhantomData};

use crate::{Innards, MapField, Shape, ShapeUninit, Shapely};

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

/// Given the map's address, returns a FieldSlot for the requested field
pub trait Slots: Send + Sync {
    /// Returns a FieldSlot for a given field. If the map accommodates dynamically-added fields,
    /// this might, for example, insert an entry into a HashMap.
    ///
    /// Returns None if the field is not known and the data structure does not accommodate for arbitrary fields.
    fn slot<'a>(&'a mut self, map: &'a mut ShapeUninit, field: MapField<'_>) -> Option<Slot<'a>>;
}

/// All known slots types
pub enum AllSlots {
    Struct(StructSlots),
    Map(HashMapSlots),
}

/// Manipulator for struct-like types with known field offsets
pub struct StructSlots {
    struct_shape: Shape,
}

impl StructSlots {
    /// Create a new Slots suitable for a struct — only fields listed
    /// in the map innards will be accepted, and they all must have an offset.
    pub fn new(struct_shape: Shape) -> Self {
        Self { struct_shape }
    }
}

impl Slots for StructSlots {
    fn slot<'a>(&'a mut self, map: &'a mut ShapeUninit, field: MapField<'_>) -> Option<Slot<'a>> {
        if let Innards::Map(innards) = self.struct_shape.innards {
            if let Some(field) = innards.fields().iter().find(|f| f.name == field.name) {
                if let Some(offset) = field.offset {
                    Some(Slot::for_struct_field(unsafe {
                        map.get_addr(&self.struct_shape).add(offset.get() as usize)
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            panic!(
                "Unexpected shape kind: expected Map, found {:?}",
                self.struct_shape.innards
            );
        }
    }
}

/// Slots for hashmaps
struct HashMapSlots {
    map_shape: &'static Shape,
}

impl HashMapSlots {
    /// Create a new Slots suitable for a hash map.
    /// Validates that the provided shape is a Map with an Array inner type.
    pub fn new<K, V>(map_shape: &'static Shape) -> Self
    where
        K: Shapely,
        V: Shapely,
    {
        if let Innards::Map(innards) = map_shape.innards {
            if let Innards::Array(elem_shape) = innards.fields()[0].schema().innards {
                if elem_shape == &V::shape() {
                    return Self { map_shape };
                }
            }
        }
        panic!("Invalid shape for HashMap: expected Map with Array inner type");
    }
}

impl Slots for HashMapSlots {
    fn slot<'a>(&'a mut self, map: &'a mut ShapeUninit, field: MapField<'_>) -> Option<Slot<'a>> {
        unsafe {
            Some(Slot::for_hash_map(
                map.get_addr(self.map_shape) as *mut HashMap<String, _>,
                field.name.to_string(),
            ))
        }
    }
}
