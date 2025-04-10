use facet_core::{Facet, Opaque, OpaqueConst, OpaqueUninit, Shape, TryFromError, ValueVTable};
use facet_peek::Peek;

/// A strongly-typed value writer that ensures type safety at compile-time
pub struct TypedPokeValue<'mem, T: Facet> {
    poke_value: PokeValue<'mem>,
    _phantom: core::marker::PhantomData<T>,
}

impl<'mem, T: Facet> TypedPokeValue<'mem, T> {
    /// Create a new TypedPokeValue from a PokeValue
    fn new(poke_value: PokeValue<'mem>) -> Self {
        Self {
            poke_value,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Place a value of type T in the space provided
    pub fn put(self, value: T) -> Opaque<'mem> {
        // We already verified the shape matches T when we created this TypedPokeValue
        unsafe { self.poke_value.data.put(value) }
    }
}

/// Lets you write to a value (implements write-only [`ValueVTable`] proxies)
pub struct PokeValue<'mem> {
    data: OpaqueUninit<'mem>,
    shape: &'static Shape,
}

impl core::fmt::Debug for PokeValue<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PokeValue")
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}

impl<'mem> PokeValue<'mem> {
    #[inline(always)]
    /// Coerce back into a `PokeValue`
    pub fn into_value(self) -> Self {
        self
    }

    /// Converts to a type-checked [`TypedPokeValue<T>`] if the shape matches type `T`
    ///
    /// Returns `None` if the shape doesn't match the type `T`.
    pub fn typed<T: Facet>(self) -> Option<TypedPokeValue<'mem, T>> {
        if self.shape.is_type::<T>() {
            Some(TypedPokeValue::new(self))
        } else {
            None
        }
    }

    /// Shape getter
    #[inline(always)]
    pub fn shape(&self) -> &'static Shape {
        self.shape
    }
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

    /// Exposes the internal data buffer as a mutable reference
    ///
    /// # Safety
    ///
    /// The caller must ensure that they don't violate any invariants of the underlying type.
    pub unsafe fn data(&mut self) -> OpaqueUninit<'mem> {
        self.data
    }

    /// Attempts to convert a value from another type into this one
    ///
    /// Returns `Ok(Opaque)` if the conversion was successful, `Err((Self, TryFromError))` otherwise.
    pub fn try_from<'src>(
        self,
        source: OpaqueConst<'src>,
    ) -> Result<Opaque<'mem>, (Self, TryFromError)> {
        if let Some(try_from_fn) = self.vtable().try_from {
            match unsafe { try_from_fn(source, self.data) } {
                Ok(built_val) => Ok(built_val),
                Err(err) => Err((self, err)),
            }
        } else {
            let shape = self.shape;
            Err((self, TryFromError::Unimplemented(shape)))
        }
    }

    /// Attempts to parse a string into this value
    ///
    /// Returns `Ok(Opaque)` if parsing was successful, `Err(Self)` otherwise.
    pub fn parse(self, s: &str) -> Result<Opaque<'mem>, Self> {
        if let Some(parse_fn) = self.vtable().parse {
            match unsafe { parse_fn(s, self.data) } {
                Ok(parsed_val) => Ok(parsed_val),
                Err(_) => Err(self),
            }
        } else {
            Err(self)
        }
    }

    /// Place a value in the space provided
    ///
    /// This function places a value of type T into the destination space,
    /// checking that T exactly matches the expected shape.
    pub fn put<'src, T>(self, value: T) -> Opaque<'mem>
    where
        T: Facet + 'src,
    {
        self.shape.assert_type::<T>();
        unsafe { self.data.put(value) }
    }

    /// Attempts to set the value to its default
    ///
    /// Returns `Ok(Opaque)` if setting to default was successful, `Err(Self)` otherwise.
    pub fn default_in_place(self) -> Result<Opaque<'mem>, Self> {
        if let Some(default_in_place_fn) = self.vtable().default_in_place {
            let default_val = unsafe { default_in_place_fn(self.data) };
            Ok(default_val)
        } else {
            Err(self)
        }
    }

    /// Attempts to clone `source` into this value
    ///
    /// Returns `Ok(Peek)` if cloning was successful, `Err(Self)` otherwise.
    pub fn clone_from<'src>(self, source: Peek<'src>) -> Result<Peek<'mem>, Self> {
        if let Some(clone_fn) = self.vtable().clone_into {
            let cloned_val = unsafe { clone_fn(source.data(), self.data) };
            // Safe because the function will initialize our data if it returns Some
            Ok(unsafe { Peek::unchecked_new(cloned_val.as_const(), self.shape) })
        } else {
            Err(self)
        }
    }
}
