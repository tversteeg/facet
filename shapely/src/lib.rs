//! Provides the core traits for shapely

#[cfg(feature = "derive")]
pub use shapely_derive::*;

pub use nonmax::{self, NonMaxU32};

use std::{alloc, marker::PhantomData, mem::MaybeUninit, ptr::NonNull};

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
pub trait Shapely: Sized {
    /// Returns the shape of this type
    fn shape() -> Shape;

    /// Returns a shape def (a function that can describe this shape)
    fn shape_desc() -> ShapeDesc {
        ShapeDesc(Self::shape)
    }

    /// allocates the right amount of memory to build such a shape on the heap and returns it in the
    /// form of a ShapeUninit
    fn shape_uninit() -> ShapeUninit<'static> {
        let shape = Self::shape();
        let addr = unsafe { alloc::alloc(shape.layout) };
        if addr.is_null() {
            alloc::handle_alloc_error(shape.layout);
        }
        ShapeUninit {
            origin: Origin::HeapAllocated,
            phantom: PhantomData,
            addr: NonNull::new(addr).unwrap(),
            init_fields: Default::default(),
            shape_desc: Self::shape_desc(),
        }
    }

    /// Initializes a `ShapeUninit` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The memory pointed to by `ptr` is valid for the size of `Self`
    /// - The memory comes from a `MaybeUninit<Self>`
    /// - The pointer remains valid for the lifetime of the returned `ShapeUninit`
    unsafe fn shape_uninit_from_raw(dest: &mut MaybeUninit<Self>) -> ShapeUninit<'_> {
        ShapeUninit {
            origin: Origin::Borrowed {
                parent: None,
                init_field_slot: InitFieldSlot::Ignored,
            },
            phantom: PhantomData,
            addr: NonNull::new(dest.as_mut_ptr() as *mut ()).unwrap(),
            init_fields: Default::default(),
            shape_desc: Self::shape_desc(),
        }
    }
}
