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

#[derive(Clone, Copy)]
pub enum Poke<'mem> {
    Scalar(PokeValue<'mem>),
    List(PokeList<'mem>),
    Map(PokeMap<'mem>),
    /// works for structs, tuple structs, and tuples
    Struct(PokeStruct<'mem>),
    Enum(PokeEnum<'mem>),
}

impl<'mem> Poke<'mem> {
    /// Creates a new poke from a mutable reference to a MaybeUninit of a type that implements shapely
    pub fn new<S: Shapely>(borrow: &'mem mut MaybeUninit<S>) -> Self {
        // This is safe because we're creating an Opaque pointer to read-only data
        // The pointer will be valid for the lifetime 'mem
        let data = OpaqueUninit::from_maybe_uninit(borrow);
        unsafe { Self::unchecked_new(data, S::shape_desc()) }
    }

    /// Creates a new peek, for easy manipulation of some opaque data.
    ///
    /// # Safety
    ///
    /// `data` must be initialized and well-aligned, and point to a value
    /// of the type described by `shape`.
    pub unsafe fn unchecked_new(data: OpaqueUninit<'mem>, shape_desc: ShapeDesc) -> Self {
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
}
