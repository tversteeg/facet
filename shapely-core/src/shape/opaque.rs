//! Opaque pointers

use std::{marker::PhantomData, ptr::NonNull};

/// A type-erased pointer to an uninitialized value
#[derive(Clone, Copy)]
pub struct OpaqueUninit<'mem>(pub *mut u8, pub PhantomData<&'mem mut ()>);

impl<'mem> OpaqueUninit<'mem> {
    /// Assumes the pointer is initialized and returns an `Opaque` pointer
    pub unsafe fn assume_init(self) -> Opaque<'mem> {
        let ptr = unsafe { NonNull::new_unchecked(self.0) };
        Opaque(ptr, PhantomData)
    }

    /// Write a value to this location and convert to an initialized pointer
    pub unsafe fn write<T>(self, value: T) -> Opaque<'mem> {
        unsafe {
            std::ptr::write(self.0 as *mut T, value);
            self.assume_init()
        }
    }
}

/// A type-erased read-only pointer to an initialized value.
///
/// Cannot be null. May be dangling (for ZSTs)
#[derive(Clone, Copy)]
pub struct OpaqueConst<'mem>(NonNull<u8>, PhantomData<&'mem ()>);

impl<'mem> OpaqueConst<'mem> {
    /// Create a new opaque const pointer from a reference
    pub fn from_ref<T>(r: &'mem T) -> Self {
        Self(NonNull::from(r).cast(), PhantomData)
    }

    /// Create a new opaque const pointer from a raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid, aligned, and point to initialized memory
    /// of the correct type, and be valid for lifetime `'mem`.
    pub unsafe fn new_unchecked<T>(ptr: *const T) -> Self {
        unsafe { Self(NonNull::new_unchecked(ptr as *mut u8), PhantomData) }
    }

    /// Gets the underlying raw pointer as a byte pointer
    pub fn as_byte_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Gets the underlying raw pointer as a pointer of type T
    ///
    /// # Safety
    ///
    /// Must be called with the original type T that was used to create this pointer
    pub unsafe fn as_ptr<T>(&self) -> *const T {
        self.0.as_ptr() as *const T
    }

    /// Gets the underlying raw pointer as a const pointer of type T
    ///
    /// # Safety
    ///
    /// `T` must be the _actual_ underlying type. You're downcasting with no guardrails.
    pub unsafe fn as_ref<'borrow: 'mem, T>(&'borrow self) -> &'borrow T {
        unsafe { &*(self.0.as_ptr() as *const T) }
    }
}

/// A type-erased pointer to an initialized value
#[derive(Clone, Copy)]
pub struct Opaque<'mem>(NonNull<u8>, PhantomData<&'mem mut ()>);

impl<'mem> Opaque<'mem> {
    /// Create a new opaque pointer from a mutable reference
    pub fn from_ref<T>(r: &'mem mut T) -> Self {
        Self(NonNull::from(r).cast(), PhantomData)
    }

    /// Create a new opaque pointer from a raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid, aligned, and point to initialized memory
    /// of the correct type, and be valid for lifetime `'mem`.
    pub unsafe fn new_unchecked<T>(ptr: *mut T) -> Self {
        Self(
            unsafe { NonNull::new_unchecked(ptr as *mut u8) },
            PhantomData,
        )
    }

    /// Gets the underlying raw pointer
    pub fn as_byte_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Gets the underlying raw pointer as mutable
    pub fn as_mut_byte_ptr(&mut self) -> *mut u8 {
        self.0.as_ptr()
    }

    /// Gets the underlying raw pointer as a pointer of type T
    ///
    /// # Safety
    ///
    /// Must be called with the original type T that was used to create this pointer
    pub unsafe fn as_ptr<T>(&self) -> *const T {
        self.0.as_ptr() as *const T
    }

    /// Gets the underlying raw pointer as a mutable pointer of type T
    ///
    /// # Safety
    ///
    /// `T` must be the _actual_ iunderlying type. You're downcasting with no guardrails.
    pub unsafe fn as_mut<'borrow: 'mem, T>(&'borrow mut self) -> &'borrow mut T {
        unsafe { &mut *(self.0.as_ptr() as *mut T) }
    }

    /// Gets the underlying raw pointer as a const pointer of type T
    ///
    /// # Safety
    ///
    /// `T` must be the _actual_ underlying type. You're downcasting with no guardrails.
    pub unsafe fn as_ref<'borrow: 'mem, T>(&'borrow self) -> &'borrow T {
        unsafe { &*(self.0.as_ptr() as *const T) }
    }

    /// Make a const ptr out of this mut ptr
    pub fn as_const<'borrow: 'mem>(&'borrow self) -> OpaqueConst<'borrow> {
        OpaqueConst(self.0, PhantomData)
    }
}
