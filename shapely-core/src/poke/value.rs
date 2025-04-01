use crate::{Opaque, OpaqueConst, OpaqueUninit, Peek, Shape, ValueVTable};

/// Lets you write to a value (implements write-only [`ValueVTable`] proxies)
pub struct PokeValue<'mem> {
    data: OpaqueUninit<'mem>,
    shape: Shape,
    vtable: ValueVTable,
}

impl<'mem> PokeValue<'mem> {
    /// Creates a value write-proxy from its essential components
    ///
    /// # Safety
    ///
    /// The data buffer must match the size and alignment of the shape.
    pub(crate) unsafe fn new(data: OpaqueUninit<'mem>, shape: Shape, vtable: ValueVTable) -> Self {
        Self {
            data,
            shape,
            vtable,
        }
    }

    /// Attempts to convert a value from another type into this one
    ///
    /// Returns `Some(Opaque)` if the conversion was successful, `None` otherwise.
    pub fn try_from<'src>(self, source: Peek<'src>) -> Result<Opaque<'mem>, Self> {
        if let Some(built_val) = self
            .vtable
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
            .vtable
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
}
