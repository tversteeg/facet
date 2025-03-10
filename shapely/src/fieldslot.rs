use std::marker::PhantomData;

use crate::{Shape, Shapely};

/// Allows filling in a field of a struct while deserializing.
pub struct FieldSlot<'s> {
    dest: *mut u8,
    shape: Shape,
    _phantom: PhantomData<&'s mut ()>,
}

impl FieldSlot<'_> {
    /// Construct a new `FieldSlot`, ready to be filled
    #[inline(always)]
    pub fn new<T: Shapely>(dest: *mut T) -> Self {
        Self {
            dest: dest as *mut u8,
            shape: T::shape(),
            _phantom: PhantomData,
        }
    }

    /// Fill this field with a value.
    pub fn fill<T: Shapely>(self, value: T) {
        let value_shape = T::shape();
        if self.shape != value_shape {
            panic!(
                "Attempted to fill a field with an incompatible shape.\n\
                Expected shape: {:?}\n\
                Actual shape: {:?}\n\
                This is unsafe and could lead to undefined behavior.",
                self.shape, value_shape
            );
        }

        unsafe {
            // FIXME: There are several invariants we do not check for here:
            // If self.dest was already initialized, then we're not doing anything about it.
            std::ptr::write(self.dest as *mut T, value);
        }
    }
}
