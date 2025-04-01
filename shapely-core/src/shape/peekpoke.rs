use crate::Shapely;
use std::cmp::Ordering;

use super::{OpaqueConst, ScalarVTable, ShapeDesc};

/// Lets you peek at the innards of a value
///
/// It's possible (in some cases..) to escape the borrow checker by setting `'mem` to `'static`,
/// in which case, you're entirely on your own.
#[derive(Clone, Copy)]
pub enum Peek<'mem> {
    Scalar(PeekScalar<'mem>),
}

/// Lets you read from a scalar
#[derive(Clone, Copy)]
pub struct PeekScalar<'mem> {
    data: OpaqueConst<'mem>,
    vtable: ScalarVTable,
}

impl<'mem> Peek<'mem> {
    /// Creates a new peek from a reference to some initialized value that implements `Shapely`
    pub fn new<S: Shapely>(s: &'mem S) -> Self {
        // This is safe because we're creating an Opaque pointer to read-only data
        // The pointer will be valid for the lifetime 'mem
        let data = OpaqueConst::from_ref(s);
        unsafe { Self::unchecked_new(data, S::shape_desc()) }
    }

    /// Creates a new peek, for easy manipulation of some opaque data.
    ///
    /// # Safety
    ///
    /// `data` must be initialized and well-aligned, and point to a value
    /// of the type described by `shape`.
    pub unsafe fn unchecked_new(data: OpaqueConst<'mem>, shape: ShapeDesc) -> Self {
        let sh = shape.get();
        match sh.innards {
            super::Innards::Struct { .. } => todo!(),
            super::Innards::TupleStruct { .. } => todo!(),
            super::Innards::Tuple { .. } => todo!(),
            super::Innards::Map { .. } => todo!(),
            super::Innards::List { .. } => todo!(),
            super::Innards::Transparent(_) => todo!(),
            super::Innards::Scalar { vtable } => Peek::Scalar(PeekScalar { data, vtable }),
            super::Innards::Enum { .. } => todo!(),
        }
    }
}

impl PeekScalar<'_> {
    /// Returns true if this scalar is equal to the other scalar
    ///
    /// # Returns
    ///
    /// `None` if equality comparison is not supported for this scalar type
    #[inline(always)]
    pub fn eq(&self, other: &PeekScalar) -> Option<bool> {
        unsafe { self.vtable.eq.map(|eq_fn| eq_fn(self.data, other.data)) }
    }

    /// Compares this scalar with another and returns their ordering
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline(always)]
    pub fn cmp(&self, other: &PeekScalar) -> Option<Ordering> {
        unsafe { self.vtable.cmp.map(|cmp_fn| cmp_fn(self.data, other.data)) }
    }

    /// Returns true if this scalar is greater than the other scalar
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline(always)]
    pub fn gt(&self, other: &PeekScalar) -> Option<bool> {
        self.cmp(other)
            .map(|ordering| ordering == Ordering::Greater)
    }

    /// Returns true if this scalar is greater than or equal to the other scalar
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline(always)]
    pub fn gte(&self, other: &PeekScalar) -> Option<bool> {
        self.cmp(other)
            .map(|ordering| ordering == Ordering::Greater || ordering == Ordering::Equal)
    }

    /// Returns true if this scalar is less than the other scalar
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline(always)]
    pub fn lt(&self, other: &PeekScalar) -> Option<bool> {
        self.cmp(other).map(|ordering| ordering == Ordering::Less)
    }

    /// Returns true if this scalar is less than or equal to the other scalar
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline(always)]
    pub fn lte(&self, other: &PeekScalar) -> Option<bool> {
        self.cmp(other)
            .map(|ordering| ordering == Ordering::Less || ordering == Ordering::Equal)
    }

    /// Formats this scalar for display
    ///
    /// # Returns
    ///
    /// `None` if display formatting is not supported for this scalar type
    #[inline(always)]
    pub fn display(&self, f: std::fmt::Formatter<'_>) -> Option<String> {
        unsafe {
            self.vtable
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
    pub fn debug(&self, f: std::fmt::Formatter<'_>) -> Option<String> {
        unsafe { self.vtable.debug.map(|debug_fn| debug_fn(self.data, f)) }
    }

    /// Hashes this scalar
    ///
    /// # Returns
    ///
    /// `false` if hashing is not supported for this scalar type, `true` otherwise
    #[inline(always)]
    pub fn hash(&self, hasher: &mut dyn std::hash::Hasher) -> bool {
        unsafe {
            if let Some(hash_fn) = self.vtable.hash {
                hash_fn(self.data, hasher as *const dyn std::hash::Hasher);
                true
            } else {
                false
            }
        }
    }
}
