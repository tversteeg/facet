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
    field_shape: fn() -> Shape,

    _phantom: PhantomData<&'s mut ()>,
}

impl Slot<'_> {
    #[inline(always)]
    pub fn for_struct_field<TField: Shapely>(field_addr: *mut TField) -> Self {
        Self {
            dest: Destination::StructField {
                field_addr: field_addr as *mut u8,
            },
            field_shape: TField::shape,
            _phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn for_hash_map(map: *mut u8, field_shape: fn() -> Shape, key: String) -> Self {
        Self {
            dest: Destination::HashMap { map, key },
            field_shape,
            _phantom: PhantomData,
        }
    }

    pub fn fill<T: Shapely>(self, value: T) {
        let value_shape = T::shape();
        if (self.field_shape)() != value_shape {
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

pub trait Slots {
    /// If the map accommodates dynamically-added fields, this might, for example, insert an entry into a HashMap.
    ///
    /// Returns None if the field is not known and the data structure does not accommodate for arbitrary fields.
    fn slot<'a>(&'a mut self, map: &'a mut ShapeUninit, field: MapField<'_>) -> Option<Slot<'a>>;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotsKind {
    Struct,
    // Reminder: hashmaps are homogeneous
    HashMap { value_shape: fn() -> Shape },
}

impl SlotsKind {
    pub fn to_slots(self, shape: Shape) -> AllSlots {
        match self {
            SlotsKind::Struct => AllSlots::Struct(StructSlots {
                struct_shape: shape,
            }),
            SlotsKind::HashMap { value_shape } => AllSlots::HashMap(HashMapSlots {
                map_shape: shape,
                value_shape,
            }),
        }
    }
}

/// All known slots types
pub enum AllSlots {
    Struct(StructSlots),
    HashMap(HashMapSlots),
}

impl Slots for AllSlots {
    fn slot<'a>(&'a mut self, map: &'a mut ShapeUninit, field: MapField<'_>) -> Option<Slot<'a>> {
        match self {
            AllSlots::Struct(inner) => inner.slot(map, field),
            AllSlots::HashMap(inner) => inner.slot(map, field),
        }
    }
}

/// Manipulator for struct-like types with known field offsets
struct StructSlots {
    struct_shape: Shape,
}

impl Slots for StructSlots {
    fn slot<'a>(&'a mut self, map: &'a mut ShapeUninit, field: MapField<'_>) -> Option<Slot<'a>> {
        if let Innards::Map(innards) = self.struct_shape.innards {
            if let Some(field) = innards
                .static_fields()
                .iter()
                .find(|f| f.name == field.name)
            {
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
    map_shape: Shape,
    value_shape: fn() -> Shape,
}

impl Slots for HashMapSlots {
    fn slot<'a>(&'a mut self, map: &'a mut ShapeUninit, field: MapField<'_>) -> Option<Slot<'a>> {
        unsafe {
            Some(Slot::for_hash_map(
                map.get_addr(&self.map_shape),
                self.value_shape,
                field.name.to_string(),
            ))
        }
    }
}
