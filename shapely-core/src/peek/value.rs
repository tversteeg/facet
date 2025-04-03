use crate::vtable::TypeNameOpts;
use crate::{Opaque, OpaqueConst, Shape};
use std::cmp::Ordering;

/// Lets you read from a value (implements read-only [`ValueVTable`] proxies)
#[derive(Clone, Copy)]
pub struct PeekValue<'mem> {
    pub(crate) data: OpaqueConst<'mem>,
    pub(crate) shape: &'static Shape,
}

impl<'mem> PeekValue<'mem> {
    /// Returns true if this scalar is equal to the other scalar
    ///
    /// # Returns
    ///
    /// `None` if equality comparison is not supported for this scalar type
    #[inline(always)]
    pub fn eq(&self, other: &PeekValue<'_>) -> Option<bool> {
        unsafe {
            self.shape
                .vtable
                .eq
                .map(|eq_fn| eq_fn(self.data, other.data))
        }
    }

    /// Compares this scalar with another and returns their ordering
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline(always)]
    #[expect(clippy::should_implement_trait)]
    pub fn cmp(&self, other: &PeekValue<'_>) -> Option<Ordering> {
        unsafe {
            self.shape
                .vtable
                .ord
                .map(|cmp_fn| cmp_fn(self.data, other.data))
        }
    }

    /// Returns true if this scalar is greater than the other scalar
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline(always)]
    pub fn gt(&self, other: &PeekValue<'_>) -> Option<bool> {
        self.cmp(other)
            .map(|ordering| ordering == Ordering::Greater)
    }

    /// Returns true if this scalar is greater than or equal to the other scalar
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline(always)]
    pub fn gte(&self, other: &PeekValue<'_>) -> Option<bool> {
        self.cmp(other)
            .map(|ordering| ordering == Ordering::Greater || ordering == Ordering::Equal)
    }

    /// Returns true if this scalar is less than the other scalar
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline(always)]
    pub fn lt(&self, other: &PeekValue<'_>) -> Option<bool> {
        self.cmp(other).map(|ordering| ordering == Ordering::Less)
    }

    /// Returns true if this scalar is less than or equal to the other scalar
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline(always)]
    pub fn lte(&self, other: &PeekValue<'_>) -> Option<bool> {
        self.cmp(other)
            .map(|ordering| ordering == Ordering::Less || ordering == Ordering::Equal)
    }

    /// Formats this scalar for display
    ///
    /// # Returns
    ///
    /// `None` if display formatting is not supported for this scalar type
    #[inline(always)]
    pub fn display(&self, f: &mut std::fmt::Formatter<'_>) -> Option<std::fmt::Result> {
        unsafe {
            self.shape
                .vtable
                .display
                .map(|display_fn| display_fn(self.data, f))
        }
    }

    /// Formats this scalar for debug
    ///
    /// # Returns
    ///
    /// `None` if debug formatting is not supported for this scalar type
    #[inline(always)]
    pub fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> Option<std::fmt::Result> {
        unsafe {
            self.shape
                .vtable
                .debug
                .map(|debug_fn| debug_fn(self.data, f))
        }
    }

    /// Hashes this scalar
    ///
    /// # Returns
    ///
    /// `false` if hashing is not supported for this scalar type, `true` otherwise
    #[inline(always)]
    pub fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) -> bool {
        unsafe {
            if let Some(hash_fn) = self.shape.vtable.hash {
                let hasher_opaque = Opaque::from_ref(hasher);
                hash_fn(self.data, hasher_opaque, |opaque, bytes| {
                    opaque.as_mut_ptr::<H>().write(bytes)
                });
                true
            } else {
                false
            }
        }
    }

    /// Returns the type name of this scalar
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `std::fmt::Formatter`
    /// * `opts` - The `TypeNameOpts` to use for formatting
    ///
    /// # Returns
    ///
    /// The result of the type name formatting
    #[inline(always)]
    pub fn type_name(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: TypeNameOpts,
    ) -> std::fmt::Result {
        (self.shape.vtable.type_name)(f, opts)
    }

    /// Returns the shape
    #[inline(always)]
    pub const fn shape(&self) -> &'static Shape {
        self.shape
    }
}
