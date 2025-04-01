use crate::Shapely;

use super::{Opaque, OpaqueConst, ScalarVTable, ShapeDesc};

/// Lets you peek at the innards of a value
///
/// It's possible (in some cases..) to escape the borrow checker by setting `'mem` to `'static`,
/// in which case, you're entirely on your own.
pub enum Peek<'mem> {
    Scalar(PeekScalar<'mem>),
}

/// Lets you read from a scalar
pub struct PeekScalar<'mem> {
    data: Opaque<'mem>,
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
    /// of the type described by `ShapeDesc`
    pub unsafe fn unchecked_new(data: OpaqueConst, shape: ShapeDesc) -> Self {
        let sh = shape.get();
        match sh.innards {
            super::Innards::Struct { fields } => todo!(),
            super::Innards::TupleStruct { fields } => todo!(),
            super::Innards::Tuple { fields } => todo!(),
            super::Innards::Map {
                vtable,
                value_shape,
            } => todo!(),
            super::Innards::List { vtable, item_shape } => todo!(),
            super::Innards::Transparent(shape_desc) => todo!(),
            super::Innards::Scalar { vtable } => Peek::Scalar(PeekScalar { data, vtable }),
            super::Innards::Enum { variants, repr } => todo!(),
        }
    }
}

impl PeekScalar {
    /// Returns true if this scalar is equal to the other scalar
    ///
    /// # Returns
    ///
    /// `None` if equality comparison is not supported for this scalar type
    pub fn eq(&self, other: &PeekScalar) -> Option<bool> {
        unsafe {
            self.vtable
                .eq
                .map(|eq_fn| eq_fn(self.data.as_ref(), other.data.as_ref()))
        }
    }
    /// Returns true if this scalar is greater than the other scalar
    ///
    /// # Returns
    ///
    /// `None` if greater-than comparison is not supported for this scalar type
    pub fn gt(&self, other: &PeekScalar) -> Option<bool> {
        unsafe {
            self.vtable
                .gt
                .map(|gt_fn| gt_fn(self.data.as_ref(), other.data.as_ref()))
        }
    }

    /// Returns true if this scalar is greater than or equal to the other scalar
    ///
    /// # Returns
    ///
    /// `None` if greater-than-or-equal comparison is not supported for this scalar type
    pub fn gte(&self, other: &PeekScalar) -> Option<bool> {
        unsafe {
            self.vtable
                .gte
                .map(|gte_fn| gte_fn(self.data.as_ref(), other.data.as_ref()))
        }
    }

    /// Returns true if this scalar is less than the other scalar
    ///
    /// # Returns
    ///
    /// `None` if less-than comparison is not supported for this scalar type
    pub fn lt(&self, other: &PeekScalar) -> Option<bool> {
        unsafe {
            self.vtable
                .lt
                .map(|lt_fn| lt_fn(self.data.as_ref(), other.data.as_ref()))
        }
    }

    /// Returns true if this scalar is less than or equal to the other scalar
    ///
    /// # Returns
    ///
    /// `None` if less-than-or-equal comparison is not supported for this scalar type
    pub fn lte(&self, other: &PeekScalar) -> Option<bool> {
        unsafe {
            self.vtable
                .lte
                .map(|lte_fn| lte_fn(self.data.as_ref(), other.data.as_ref()))
        }
    }

    /// Formats this scalar for display
    ///
    /// # Returns
    ///
    /// `None` if display formatting is not supported for this scalar type
    pub fn display(&self, f: std::fmt::Formatter<'_>) -> Option<String> {
        unsafe {
            self.vtable
                .display
                .map(|display_fn| display_fn(self.data.as_ref(), f))
        }
    }

    /// Formats this scalar for debug
    ///
    /// # Returns
    ///
    /// `None` if debug formatting is not supported for this scalar type
    pub fn debug(&self, f: std::fmt::Formatter<'_>) -> Option<String> {
        unsafe {
            self.vtable
                .debug
                .map(|debug_fn| debug_fn(self.data.as_ref(), f))
        }
    }

    /// Hashes this scalar
    ///
    /// # Returns
    ///
    /// `false` if hashing is not supported for this scalar type, `true` otherwise
    pub fn hash(&self, hasher: &mut dyn std::hash::Hasher) -> bool {
        unsafe {
            if let Some(hash_fn) = self.vtable.hash {
                hash_fn(self.data.as_ref(), hasher as *const dyn std::hash::Hasher);
                true
            } else {
                false
            }
        }
    }
}
