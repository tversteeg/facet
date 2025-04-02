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

/// Allows querying the [Shape] of a type, which in turn lets us inspect any fields, build a value of
/// this type progressively, etc.
pub trait Shapely: Sized {
    /// Returns the shape function of this type
    const SHAPE: &'static Shape;

    /// Heap-allocate a value of this shape
    fn allocate() -> OpaqueUninit<'static> {
        OpaqueUninit::new(unsafe { std::alloc::alloc(Self::SHAPE.layout) })
    }
}

/// A wrapper around `Vec<u8>` for binary data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Bytes(pub Vec<u8>);
