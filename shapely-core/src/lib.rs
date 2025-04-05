#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod impls;

mod shape;
pub use shape::*;

mod helpers;
pub use helpers::*;

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

pub trait ShapeExt {
    /// Check if this shape is of the given type
    pub fn is_type<Other: Shapely>(&'static self) -> bool {
        self == Other::SHAPE
    }

    /// Assert that this shape is of the given type, panicking if it's not
    pub fn assert_type<Other: Shapely>(&'static self) {
        assert!(
            self.is_type::<Other>(),
            "Type mismatch: expected {:?}, found {:?}",
            ShapeDebug(Other::SHAPE),
            ShapeDebug(self)
        );
    }
}

impl<T> ShapeExt for T where T: Shapely {}

/// A wrapper around `Vec<u8>` for binary data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Bytes(pub Vec<u8>);
