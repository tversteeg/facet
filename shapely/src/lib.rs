//! Provides the core traits for shapely

#[cfg(feature = "derive")]
pub use shapely_derive::*;

pub use nonmax::{self, NonMaxU32};

use std::alloc;

mod hashmap_impl;
mod scalar_impls;

mod shape;
pub use shape::*;

mod slot;
pub use slot::Slot;

mod uninit;
pub use uninit::*;

mod helpers;
pub use helpers::*;

#[cfg(all(test, feature = "derive"))]
mod derive_tests;

#[cfg(test)]
mod tests;

/// Provides reflection so you can shapely about your types.
pub trait Shapely {
    /// Returns the shape of this type
    fn shape() -> Shape;

    /// Returns a shape def (a function that can describe this shape)
    fn shape_desc() -> ShapeDesc {
        ShapeDesc(Self::shape)
    }

    /// allocates the right amount of memory to build such a shape on the heap and returns it in the
    /// form of a ShapeUninit
    fn shape_uninit() -> ShapeUninit {
        let shape = Self::shape();
        let layout = alloc::Layout::from_size_align(shape.size, shape.align).unwrap();
        let addr = unsafe { alloc::alloc(layout) };
        if addr.is_null() {
            alloc::handle_alloc_error(layout);
        }
        ShapeUninit {
            addr,
            init_fields: Default::default(),
            shape_desc: Self::shape_desc(),
        }
    }

    // TODO: provide a way to bring your own alloc
}
