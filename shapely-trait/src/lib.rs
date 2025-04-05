#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub use shapely_spez;

pub use shapely_opaque::*;
pub use shapely_types::*;

mod impls;

mod macros;
pub use macros::*;

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

/// Extension trait to provide `is_type` and `assert_type`
pub trait ShapeExt {
    /// Check if this shape is of the given type
    fn is_type<Other: Shapely>(&'static self) -> bool;

    /// Assert that this shape is of the given type, panicking if it's not
    fn assert_type<Other: Shapely>(&'static self);
}

impl ShapeExt for Shape {
    /// Check if this shape is of the given type
    fn is_type<Other: Shapely>(&'static self) -> bool {
        self == Other::SHAPE
    }

    /// Assert that this shape is of the given type, panicking if it's not
    fn assert_type<Other: Shapely>(&'static self) {
        assert!(
            self.is_type::<Other>(),
            "Type mismatch: expected {}, found {self}",
            Other::SHAPE,
        );
    }
}
