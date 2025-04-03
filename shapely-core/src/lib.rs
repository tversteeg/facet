#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub use ::impls::impls;

pub mod spez;

mod opaque;
pub use opaque::*;

/// Module containing the virtual table (vtable) definitions and implementations.
///
/// The vtable module provides structures and traits for defining and working with
/// virtual tables, which are used to store metadata and function pointers for
/// dynamic dispatch in the Shapely library.
pub mod vtable;
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
    /// The shape of this type
    const SHAPE: &'static Shape;

    /// An instance of this type — which doesn't have to be valid, but it has to
    /// "not be UB". We never actually read from it, or even actually use it for
    /// dynamic dispatch, we only use it for inference tricks.
    const DUMMY: Self;

    /// Returns true if the type of `self` is equal to the type of `other`
    fn type_eq<Other: Shapely>() -> bool {
        Self::SHAPE == Other::SHAPE
    }
}

/// A wrapper around `Vec<u8>` for binary data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Bytes(pub Vec<u8>);
