//! Allows poking (writing to) shapes

use std::mem::MaybeUninit;

use crate::Shapely;

use super::{OpaqueUninit, ShapeDesc};

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

pub enum Poke<'mem> {
    Scalar(PokeValue<'mem>),
    List(PokeList<'mem>),
    Map(PokeMap<'mem>),
    /// works for structs, tuple structs, and tuples
    Struct(PokeStruct<'mem>),
    Enum(PokeEnum<'mem>),
}

impl<'mem> Poke<'mem> {
    /// Allocates a new poke of a type that implements shapely
    pub fn alloc<S: Shapely>() -> Self {
        let data = S::shape_desc().allocate();
        unsafe { Self::from_opaque_uninit(data, S::shape_desc()) }
    }

    /// Creates a new poke from a mutable reference to a MaybeUninit of a type that implements shapely
    pub fn from_maybe_uninit<S: Shapely>(borrow: &'mem mut MaybeUninit<S>) -> Self {
        // This is safe because we're creating an Opaque pointer to read-only data
        // The pointer will be valid for the lifetime 'mem
        let data = OpaqueUninit::from_maybe_uninit(borrow);
        unsafe { Self::from_opaque_uninit(data, S::shape_desc()) }
    }

    /// Creates a new peek, for easy manipulation of some opaque data.
    ///
    /// # Safety
    ///
    /// `data` must be initialized and well-aligned, and point to a value
    /// of the type described by `shape`.
    pub unsafe fn from_opaque_uninit(data: OpaqueUninit<'mem>, shape_desc: ShapeDesc) -> Self {
        let shape = shape_desc.get();
        match shape.innards {
            super::Innards::Struct { .. } => todo!(),
            super::Innards::TupleStruct { .. } => todo!(),
            super::Innards::Tuple { .. } => todo!(),
            super::Innards::Map { .. } => todo!(),
            super::Innards::List { .. } => todo!(),
            super::Innards::Scalar => Poke::Scalar(PokeValue {
                data,
                shape,
                // let's cache that
                vtable: shape.vtable(),
            }),
            super::Innards::Enum { .. } => todo!(),
        }
    }

    pub fn into_struct(self) -> PokeStruct<'mem> {
        match self {
            Poke::Struct(s) => s,
            _ => panic!("expected Struct variant"),
        }
    }

    pub fn into_list(self) -> PokeList<'mem> {
        match self {
            Poke::List(l) => l,
            _ => panic!("expected List variant"),
        }
    }

    pub fn into_map(self) -> PokeMap<'mem> {
        match self {
            Poke::Map(m) => m,
            _ => panic!("expected Map variant"),
        }
    }

    pub fn into_scalar(self) -> PokeValue<'mem> {
        match self {
            Poke::Scalar(s) => s,
            _ => panic!("expected Scalar variant"),
        }
    }

    pub fn into_enum(self) -> PokeEnum<'mem> {
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
