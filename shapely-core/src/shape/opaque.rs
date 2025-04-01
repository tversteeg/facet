//! Opaque pointers

use std::marker::PhantomData;

/// A type-erased read-only pointer to an initialized value
#[derive(Clone, Copy)]
pub struct OpaqueConst<'mem>(pub *const u8, pub PhantomData<&'mem ()>);

/// A type-erased pointer to an uninitialized value
#[derive(Clone, Copy)]
pub struct OpaqueUninit<'mem>(pub *mut u8, pub PhantomData<&'mem mut ()>);

impl<'mem> OpaqueUninit<'mem> {
    /// Assumes the pointer is initialized and returns an `Opaque` pointer
    pub unsafe fn assume_init(self) -> Opaque<'mem> {
        Opaque(self.0, PhantomData)
    }
}

/// A type-erased pointer to an initialized value
#[derive(Clone, Copy)]
pub struct Opaque<'mem>(pub *mut u8, pub PhantomData<&'mem mut ()>);

impl<'mem> Opaque<'mem> {
    /// Make a const ptr out of this mut ptr
    pub fn as_ref<'borrow: 'mem>(&'borrow self) -> OpaqueConst {
        OpaqueConst(self.0, PhantomData)
    }
}
