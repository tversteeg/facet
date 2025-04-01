use std::mem::MaybeUninit;

use crate::Shapely;

use super::{Opaque, OpaqueConst, OpaqueUninit, Shape, ShapeDesc, ValueVTable};

#[derive(Clone, Copy)]
pub enum Poke<'mem> {
    Scalar(PokeValue<'mem>),
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

/// Lets you write to a value (implements write-only [`ValueVTable`] proxies)
#[derive(Clone, Copy)]
pub struct PokeValue<'mem> {
    pub data: OpaqueUninit<'mem>,
    pub shape: Shape,
    pub vtable: ValueVTable,
}

impl<'mem> PokeValue<'mem> {
    /// Attempts to convert a value from another type into this one
    ///
    /// Returns `Some(Opaque)` if the conversion was successful, `None` otherwise.
    pub fn try_from<'src>(self, source: super::Peek<'src>) -> Result<Opaque<'mem>, Self> {
        if let Some(built_val) = self
            .vtable
            .try_from
            .and_then(|try_from_fn| unsafe { try_from_fn(source, self.data) })
        {
            // Safe because the function will initialize our data if it returns Some
            Ok(built_val)
        } else {
            Err(self)
        }
    }

    /// Attempts to parse a string into this value
    ///
    /// Returns `Some(Opaque)` if parsing was successful, `None` otherwise.
    pub fn parse(self, s: &str) -> Result<Opaque<'mem>, Self> {
        if let Some(parsed_val) = self
            .vtable
            .parse
            .and_then(|parse_fn| unsafe { parse_fn(s, self.data) })
        {
            // Safe because the function will initialize our data if it returns Some
            Ok(parsed_val)
        } else {
            Err(self)
        }
    }

    /// Reads data from an opaque const pointer into this value
    ///
    /// # Safety
    ///
    /// The `source` must be a valid, initialized pointer to a value of the same type
    /// as described by this `PokeValue`'s shape.
    ///
    /// Also, `source` is moved out of after this function is called, so it cannot be used
    /// anymore — it should be deallocated, but it should not be "dropped" anymore.
    pub unsafe fn put<'src>(self, source: OpaqueConst<'src>) -> Opaque<'mem> {
        unsafe {
            std::ptr::copy_nonoverlapping(
                source.as_ptr(),
                self.data.as_mut_ptr(),
                self.shape.layout.size(),
            );
            self.data.assume_init()
        }
    }
}
