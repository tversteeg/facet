#[doc = include_str!("../README.md")]
#[cfg(feature = "derive")]
pub use shapely_derive::*;

pub use nonmax::{self, NonMaxU32};

use std::mem::MaybeUninit;

mod hashmap_impl;
mod scalar_impls;

mod shape;
pub use shape::*;

mod slot;
pub use slot::Slot;

mod partial;
pub use partial::*;

mod helpers;
pub use helpers::*;

pub mod mini_typeid;

#[doc(hidden)]
pub mod log;
pub use log::*;

#[cfg(test)]
mod tests;

/// Provides reflection so you can shapely about your types.
pub trait Shapely: Sized {
    /// Returns the shape of this type
    fn shape() -> Shape;

    /// Returns a shape def (a function that can describe this shape)
    fn shape_desc() -> ShapeDesc {
        ShapeDesc(Self::shape)
    }

    /// Allocates this shape on the heap and return a partial that allows gradually initializing its fields.
    fn partial() -> Partial<'static> {
        Partial::alloc(Self::shape_desc())
    }

    /// Initializes a `Partial` from a borrowed `MaybeUninit<Self>`.
    ///
    /// Before calling assume_init, make sure to call Partial.build_in_place().
    fn partial_from_uninit(dest: &mut MaybeUninit<Self>) -> Partial<'_> {
        Partial::borrow(dest)
    }

    // TODO: partial_from_mut? where all the fields are already initialized?
}
