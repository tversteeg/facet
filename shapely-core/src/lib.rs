#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod opaque;
pub use opaque::*;

mod vtable;
pub use vtable::*;

mod impls;

mod shape;
pub use shape::*;

mod helpers;
pub use helpers::*;

mod poke;
pub use poke::*;

mod peek;
pub use peek::*;

#[doc(hidden)]
pub mod log;
pub use log::*;

#[cfg(test)]
mod tests;

/// A unique identifier for a type's shape
#[derive(Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ShapeId(u64, u64);

impl std::fmt::Debug for ShapeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}{:016x}", self.0, self.1)
    }
}

impl ShapeId {
    /// Returns a unique identifier for the given shape
    #[inline(always)]
    pub fn of(shape: &'static Shape) -> ShapeId {
        ShapeId(shape as *const _ as u64, 0)
    }
}

/// Allows querying the [Shape] of a type, which in turn lets us inspect any fields, build a value of
/// this type progressively, etc.
pub trait Shapely: Sized {
    /// Returns the shape function of this type
    const SHAPE: &'static Shape;

    /// Returns a unique identifier for this type
    #[inline(always)]
    fn shape_id() -> ShapeId {
        ShapeId::of(Self::SHAPE)
    }

    /// Returns true if the type of `self` is equal to the type of `other`
    fn type_eq<Other: Shapely>() -> bool {
        Self::shape_id() == Other::shape_id()
    }
}

/// A wrapper around `Vec<u8>` for binary data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Bytes(pub Vec<u8>);
