//! Allows poking (writing to) shapes

use std::mem::MaybeUninit;

use crate::{Shape, Shapely};

use super::{Def, OpaqueUninit};

mod value;
pub use value::*;

mod list;
pub use list::*;

mod map;
pub use map::*;

mod struct_;
pub use struct_::*;

mod enum_;
pub use enum_::*;

/// Allows writing values of different kinds.
pub enum Poke<'mem> {
    /// A scalar value. See [`PokeValue`].
    Scalar(PokeValue<'mem>),
    /// A list (array/vec/etc). See [`PokeList`].
    List(PokeListUninit<'mem>),
    /// A map (HashMap/BTreeMap/etc). See [`PokeMap`].
    Map(PokeMapUninit<'mem>),
    /// A struct, tuple struct, or tuple. See [`PokeStruct`].
    Struct(PokeStruct<'mem>),
    /// An enum variant. See [`PokeEnum`].
    Enum(PokeEnumNoVariant<'mem>),
}

impl<'mem> Poke<'mem> {
    /// Allocates a new poke of a type that implements shapely
    pub fn alloc<S: Shapely>() -> Self {
        let data = S::allocate();
        unsafe { Self::from_opaque_uninit(data, S::SHAPE) }
    }

    /// Creates a new poke from a mutable reference to a MaybeUninit of a type that implements shapely
    pub fn from_maybe_uninit<S: Shapely>(borrow: &'mem mut MaybeUninit<S>) -> Self {
        // This is safe because we're creating an Opaque pointer to read-only data
        // The pointer will be valid for the lifetime 'mem
        let data = OpaqueUninit::from_maybe_uninit(borrow);
        unsafe { Self::from_opaque_uninit(data, S::SHAPE) }
    }

    /// Creates a new peek, for easy manipulation of some opaque data.
    ///
    /// # Safety
    ///
    /// `data` must be initialized and well-aligned, and point to a value
    /// of the type described by `shape`.
    pub unsafe fn from_opaque_uninit(data: OpaqueUninit<'mem>, shape: &'static Shape) -> Self {
        match shape.def {
            Def::Struct(struct_def) | Def::TupleStruct(struct_def) | Def::Tuple(struct_def) => {
                Poke::Struct(unsafe { PokeStruct::new(data, shape, struct_def) })
            }
            Def::Map(map_def) => {
                let pmu = unsafe { PokeMapUninit::new(data, shape, map_def) };
                Poke::Map(pmu)
            }
            Def::List(list_def) => {
                let plu = unsafe { PokeListUninit::new(data, shape, list_def) };
                Poke::List(plu)
            }
            Def::Scalar => Poke::Scalar(unsafe { PokeValue::new(data, shape) }),
            Def::Enum(enum_def) => {
                Poke::Enum(unsafe { PokeEnumNoVariant::new(data, shape, enum_def) })
            }
        }
    }

    /// Converts this Poke into a PokeStruct, panicking if it's not a Struct variant
    pub fn into_struct(self) -> PokeStruct<'mem> {
        match self {
            Poke::Struct(s) => s,
            _ => panic!("expected Struct variant"),
        }
    }

    /// Converts this Poke into a PokeList, panicking if it's not a List variant
    pub fn into_list(self) -> PokeListUninit<'mem> {
        match self {
            Poke::List(l) => l,
            _ => panic!("expected List variant"),
        }
    }

    /// Converts this Poke into a PokeMap, panicking if it's not a Map variant
    pub fn into_map(self) -> PokeMapUninit<'mem> {
        match self {
            Poke::Map(m) => m,
            _ => panic!("expected Map variant"),
        }
    }

    /// Converts this Poke into a PokeValue, panicking if it's not a Scalar variant
    pub fn into_scalar(self) -> PokeValue<'mem> {
        match self {
            Poke::Scalar(s) => s,
            _ => panic!("expected Scalar variant"),
        }
    }

    /// Converts this Poke into a PokeEnum, panicking if it's not an Enum variant
    pub fn into_enum(self) -> PokeEnumNoVariant<'mem> {
        match self {
            Poke::Enum(e) => e,
            _ => panic!("expected Enum variant"),
        }
    }
}

/// Keeps track of which fields were initialized, up to 64 fields
#[derive(Clone, Copy, Default)]
pub struct ISet(u64);

impl ISet {
    /// Sets the bit at the given index.
    pub fn set(&mut self, index: usize) {
        if index >= 64 {
            panic!("ISet can only track up to 64 fields. Index {index} is out of bounds.");
        }
        self.0 |= 1 << index;
    }

    /// Unsets the bit at the given index.
    pub fn unset(&mut self, index: usize) {
        if index >= 64 {
            panic!("ISet can only track up to 64 fields. Index {index} is out of bounds.");
        }
        self.0 &= !(1 << index);
    }

    /// Checks if the bit at the given index is set.
    pub fn has(&self, index: usize) -> bool {
        if index >= 64 {
            panic!("ISet can only track up to 64 fields. Index {index} is out of bounds.");
        }
        (self.0 & (1 << index)) != 0
    }

    /// Checks if all bits up to the given count are set.
    pub fn all_set(&self, count: usize) -> bool {
        if count > 64 {
            panic!("ISet can only track up to 64 fields. Count {count} is out of bounds.");
        }
        let mask = (1 << count) - 1;
        self.0 & mask == mask
    }
}
