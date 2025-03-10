use crate::Shape;
use std::alloc;

/// A partially-initialized shape
pub struct ShapeUninit {
    /// Address of the value in memory
    addr: *mut u8,

    /// Keeps track of which fields are initialized
    init_fields: InitFields64,

    /// The shape of the value
    shape: Shape,
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
