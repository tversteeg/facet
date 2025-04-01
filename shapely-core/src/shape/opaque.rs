//! Opaque pointers

use std::marker::PhantomData;

/// A type-erased read-only pointer to an initialized value
#[derive(Clone, Copy)]
pub struct OpaqueConst<'mem>(pub *const u8, pub PhantomData<&'mem ()>);

impl<'mem> OpaqueConst<'mem> {
    /// Create a new opaque const pointer from a reference
    pub fn from_ref<T>(r: &'mem T) -> Self {
        Self(r as *const T as *const u8, PhantomData)
    }

    /// Create a new opaque const pointer from a raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid, aligned, and point to initialized memory
    /// of the correct type, and be valid for lifetime `'mem`.
    pub unsafe fn new_unchecked<T>(ptr: *const T) -> Self {
        Self(ptr as *const u8, PhantomData)
    }
}

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
    /// Create a new opaque pointer from a mutable reference
    pub fn from_ref<T>(r: &'mem mut T) -> Self {
        Self(r as *mut T as *mut u8, PhantomData)
    }

    /// Create a new opaque pointer from a raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid, aligned, and point to initialized memory
    /// of the correct type, and be valid for lifetime `'mem`.
    pub unsafe fn new_unchecked<T>(ptr: *mut T) -> Self {
        Self(ptr as *mut u8, PhantomData)
    }
}

impl<'mem> Opaque<'mem> {
    /// Make a const ptr out of this mut ptr
    pub fn as_ref<'borrow: 'mem>(&'borrow self) -> OpaqueConst {
        OpaqueConst(self.0, PhantomData)
    }
}
