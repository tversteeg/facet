use crate::vtable::TypeNameOpts;
use crate::{Opaque, OpaqueConst, Shape, ValueVTable};
use std::cmp::Ordering;

/// Lets you read from a value (implements read-only [`ValueVTable`] proxies)
#[derive(Clone, Copy)]
pub struct PeekValue<'mem> {
    data: OpaqueConst<'mem>,
    shape: &'static Shape,
}

impl std::fmt::Display for PeekValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(display_fn) = self.vtable().display {
            unsafe { display_fn(self.data, f) }
        } else {
            write!(f, "⟨{}⟩", self.shape)
        }
    }
}

impl std::fmt::Debug for PeekValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(debug_fn) = self.vtable().debug {
            unsafe { debug_fn(self.data, f) }
        } else {
            write!(f, "⟨{}⟩", self.shape)
        }
    }
}

impl std::cmp::PartialEq for PeekValue<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.shape != other.shape {
            return false;
        }
        let eq_fn = match self.shape.vtable.eq {
            Some(eq_fn) => eq_fn,
            None => return false,
        };
        unsafe { eq_fn(self.data, other.data) }
    }
}

impl std::cmp::PartialOrd for PeekValue<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.shape != other.shape {
            return None;
        }
        let partial_ord_fn = self.shape.vtable.partial_ord?;
        unsafe { partial_ord_fn(self.data, other.data) }
    }
}

impl<'mem> PeekValue<'mem> {
    /// Creates a new `PeekValue` instance.
    pub(crate) fn new(data: OpaqueConst<'mem>, shape: &'static Shape) -> Self {
        Self { data, shape }
    }

    /// Returns the vtable
    #[inline(always)]
    pub fn vtable(&self) -> &'static ValueVTable {
        self.shape.vtable
    }

    /// Returns true if this scalar is equal to the other scalar
    ///
    /// # Returns
    ///
    /// `false` if equality comparison is not supported for this scalar type
    #[inline]
    pub fn eq(&self, other: &PeekValue<'_>) -> bool {
        unsafe {
            self.shape
                .vtable
                .eq
                .map(|eq_fn| eq_fn(self.data, other.data))
                .unwrap_or(false)
        }
    }

    /// Compares this scalar with another and returns their ordering
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline]
    #[expect(clippy::should_implement_trait)]
    pub fn partial_cmp(&self, other: &PeekValue<'_>) -> Option<Ordering> {
        unsafe {
            self.shape
                .vtable
                .partial_ord
                .and_then(|partial_ord_fn| partial_ord_fn(self.data, other.data))
        }
    }

    /// Compares this scalar with another and returns their ordering
    ///
    /// # Returns
    ///
    /// `None` if comparison is not supported for this scalar type
    #[inline]
    pub fn cmp(&self, other: &PeekValue<'_>) -> Option<Ordering> {
        unsafe {
            self.shape
                .vtable
                .ord
                .map(|ord_fn| ord_fn(self.data, other.data))
        }
    }

    /// Returns true if this scalar is greater than the other scalar
    ///
    /// # Returns
    ///
    /// `false` if comparison is not supported for this scalar type
    #[inline]
    pub fn gt(&self, other: &PeekValue<'_>) -> bool {
        self.cmp(other)
            .map(|ordering| ordering == Ordering::Greater)
            .unwrap_or(false)
    }

    /// Returns true if this scalar is greater than or equal to the other scalar
    ///
    /// # Returns
    ///
    /// `false` if comparison is not supported for this scalar type
    #[inline]
    pub fn gte(&self, other: &PeekValue<'_>) -> bool {
        self.cmp(other)
            .map(|ordering| ordering == Ordering::Greater || ordering == Ordering::Equal)
            .unwrap_or(false)
    }

    /// Returns true if this scalar is less than the other scalar
    ///
    /// # Returns
    ///
    /// `false` if comparison is not supported for this scalar type
    #[inline]
    pub fn lt(&self, other: &PeekValue<'_>) -> bool {
        self.cmp(other)
            .map(|ordering| ordering == Ordering::Less)
            .unwrap_or(false)
    }

    /// Returns true if this scalar is less than or equal to the other scalar
    ///
    /// # Returns
    ///
    /// `false` if comparison is not supported for this scalar type
    #[inline(always)]
    pub fn lte(&self, other: &PeekValue<'_>) -> bool {
        self.cmp(other)
            .map(|ordering| ordering == Ordering::Less || ordering == Ordering::Equal)
            .unwrap_or(false)
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

    /// Returns the data
    #[inline(always)]
    pub const fn data(&self) -> OpaqueConst<'mem> {
        self.data
    }
}
