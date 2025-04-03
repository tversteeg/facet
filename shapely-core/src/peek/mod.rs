//! Allows peeking (reading from) shapes

use crate::Shapely;
use std::cmp::Ordering;

use super::{Opaque, OpaqueConst, Shape};

/// Lets you peek at the innards of a value
///
/// It's possible (in some cases..) to escape the borrow checker by setting `'mem` to `'static`,
/// in which case, you're entirely on your own.
#[derive(Clone, Copy)]
pub enum Peek<'mem> {
    /// cf. [`PeekValue`]
    Scalar(PeekValue<'mem>),
}

impl std::fmt::Debug for Peek<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(v) => match v.debug(f) {
                Some(res) => res,
                None => write!(f, "({:?})", v.shape),
            },
        }
    }
}

/// Lets you read from a value (implements read-only [`ValueVTable`] proxies)
#[derive(Clone, Copy)]
pub struct PeekValue<'mem> {
    data: OpaqueConst<'mem>,
    shape: &'static Shape,
}

impl<'mem> Peek<'mem> {
    /// Creates a new peek from a reference to some initialized value that implements `Shapely`
    pub fn new<S: Shapely>(s: &'mem S) -> Self {
        // This is safe because we're creating an Opaque pointer to read-only data
        // The pointer will be valid for the lifetime 'mem
        let data = OpaqueConst::from_ref(s);
        unsafe { Self::unchecked_new(data, S::SHAPE) }
    }

    /// Creates a new peek, for easy manipulation of some opaque data.
    ///
    /// # Safety
    ///
    /// `data` must be initialized and well-aligned, and point to a value
    /// of the type described by `shape`.
    pub unsafe fn unchecked_new(data: OpaqueConst<'mem>, shape: &'static Shape) -> Self {
        match shape.def {
            super::Def::Struct { .. } => todo!(),
            super::Def::TupleStruct { .. } => todo!(),
            super::Def::Tuple { .. } => todo!(),
            super::Def::Map { .. } => todo!(),
            super::Def::List { .. } => todo!(),
            super::Def::Scalar { .. } => Peek::Scalar(PeekValue { data, shape }),
            super::Def::Enum { .. } => todo!(),
        }
    }
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
                .cmp
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
        unsafe { (self.shape.vtable.debug)(self.data, f) }
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
}
