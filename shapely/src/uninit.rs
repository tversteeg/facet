use crate::{Shape, Shapely};
use std::alloc;

/// A partially-initialized shape, useful when deserializing for example.
pub struct ShapeUninit {
    /// Address of the value in memory
    pub(crate) addr: *mut u8,

    /// Keeps track of which fields are initialized
    pub(crate) init_fields: InitFields64,

    /// The shape we're building.
    pub(crate) shape: Shape,
}

impl Drop for ShapeUninit {
    fn drop(&mut self) {
        if !self.addr.is_null() {
            let layout = alloc::Layout::from_size_align(self.shape.size, self.shape.align).unwrap();
            unsafe { alloc::dealloc(self.addr, layout) }
        }
    }
}

impl ShapeUninit {
    /// Returns a pointer to the underlying data.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `self` outlives the returned pointer
    /// - The returned pointer is not aliased
    /// - The provided shape matches the shape of the data
    ///
    /// This function performs a cast and of course you could get it to do UB if you expected a different type.
    pub unsafe fn get_addr(&self, expected_shape: &Shape) -> *mut u8 {
        if self.shape == *expected_shape {
            self.addr
        } else {
            panic!(
                "Shape mismatch: expected {:?}, found {:?}",
                expected_shape, self.shape
            )
        }
    }
}

/// A bit array to keep track of which fields were initialized
#[derive(Clone, Copy)]
pub struct InitFields64(u64);

impl InitFields64 {
    pub fn new() -> Self {
        InitFields64(0)
    }

    pub fn set(&mut self, index: usize) {
        if index < 64 {
            self.0 |= 1 << index;
        }
    }

    pub fn is_set(&self, index: usize) -> bool {
        if index < 64 {
            (self.0 & (1 << index)) != 0
        } else {
            false
        }
    }

    pub fn all_set(&self, count: usize) -> bool {
        if count <= 64 {
            let mask = (1 << count) - 1;
            self.0 & mask == mask
        } else {
            false
        }
    }
}
