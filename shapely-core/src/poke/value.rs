use crate::{Opaque, OpaqueConst, OpaqueUninit, Peek, Shape, ShapeDebug, ValueVTable};

use super::Poke;

/// Lets you write to a value (implements write-only [`ValueVTable`] proxies)
pub struct PokeValue<'mem> {
    data: OpaqueUninit<'mem>,
    /// The shape of the value
    pub shape: &'static Shape,
}

impl std::fmt::Debug for PokeValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PokeValue")
            .field("shape", &ShapeDebug(self.shape))
            .finish_non_exhaustive()
    }
}

impl<'mem> PokeValue<'mem> {
    /// Creates a value write-proxy from its essential components
    ///
    /// # Safety
    ///
    /// The data buffer must match the size and alignment of the shape.
    pub(crate) unsafe fn new(data: OpaqueUninit<'mem>, shape: &'static Shape) -> Self {
        Self { data, shape }
    }

    /// Gets the vtable for the value
    #[inline(always)]
    fn vtable(&self) -> &'static ValueVTable {
        self.shape.vtable
    }

    /// Attempts to convert a value from another type into this one
    ///
    /// Returns `Some(Opaque)` if the conversion was successful, `None` otherwise.
    pub fn try_from<'src>(self, source: Peek<'src>) -> Result<Opaque<'mem>, Self> {
        if let Some(built_val) = self
            .vtable()
            .try_from
            .and_then(|try_from_fn| unsafe { try_from_fn(source, self.data) })
        {
            // Safe because the function will initialize our data if it returns Some
            Ok(built_val)
        } else {
            Err(self)
        }
    }

    /// Attempts to parse a string into this value
    ///
    /// Returns `Some(Opaque)` if parsing was successful, `None` otherwise.
    pub fn parse(self, s: &str) -> Result<Opaque<'mem>, Self> {
        if let Some(parsed_val) = self
            .vtable()
            .parse
            .and_then(|parse_fn| unsafe { parse_fn(s, self.data) })
        {
            // Safe because the function will initialize our data if it returns Some
            Ok(parsed_val)
        } else {
            Err(self)
        }
    }

    /// Reads data from an opaque const pointer into this value
    ///
    /// # Safety
    ///
    /// The `source` must be a valid, initialized pointer to a value of the same type
    /// as described by this `PokeValue`'s shape.
    ///
    /// Also, `source` is moved out of after this function is called, so it cannot be used
    /// anymore — it should be deallocated, but it should not be "dropped" anymore.
    pub unsafe fn put<'src>(self, source: OpaqueConst<'src>) -> Opaque<'mem> {
        unsafe {
            std::ptr::copy_nonoverlapping(
                source.as_ptr(),
                self.data.as_mut_ptr(),
                self.shape.layout.size(),
            );
            self.data.assume_init()
        }
    }

    /// Attempts to set the value to its default
    ///
    /// Returns `Some(Opaque)` if setting to default was successful, `None` otherwise.
    pub fn default_in_place(self) -> Result<Opaque<'mem>, Self> {
        if let Some(default_val) = self
            .vtable()
            .default_in_place
            .and_then(|default_fn| unsafe { default_fn(self.data) })
        {
            Ok(default_val)
        } else {
            Err(self)
        }
    }

    /// Attempts to clone `source` into this value
    ///
    /// Returns `Ok(Peek)` if cloning was successful, `Err(Self)` otherwise.
    pub fn clone_from<'src>(self, source: Peek<'src>) -> Result<Peek<'mem>, Self> {
        if let Some(cloned_val) = self
            .vtable()
            .clone_into
            .and_then(|clone_fn| unsafe { clone_fn(source.as_value().data, self.data) })
        {
            // Safe because the function will initialize our data if it returns Some
            Ok(unsafe { Peek::unchecked_new(cloned_val.as_const(), self.shape) })
        } else {
            Err(self)
        }
    }

    /// Unwrap back into a poke
    pub fn unwrap(self) -> Poke<'mem> {
        unsafe { Poke::from_opaque_uninit(self.data, self.shape) }
    }
}
