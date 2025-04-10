use facet_core::{Opaque, OpaqueUninit, OptionDef, OptionVTable, Shape};

/// Allows initializing an uninitialized option
pub struct PokeOptionUninit<'mem> {
    data: OpaqueUninit<'mem>,
    shape: &'static Shape,
    def: OptionDef,
}

impl<'mem> PokeOptionUninit<'mem> {
    /// Creates a new uninitialized option poke
    ///
    /// # Safety
    ///
    /// `data` must be properly aligned and sized for this shape.
    pub(crate) unsafe fn new(
        data: OpaqueUninit<'mem>,
        shape: &'static Shape,
        def: OptionDef,
    ) -> Self {
        Self { data, shape, def }
    }

    /// Returns the shape of this option
    pub fn shape(&self) -> &'static Shape {
        self.shape
    }

    /// Returns the option definition
    pub fn def(&self) -> OptionDef {
        self.def
    }

    /// Returns the option vtable
    pub fn vtable(&self) -> &'static OptionVTable {
        self.def.vtable
    }

    /// Get a reference to the underlying PokeValue
    #[inline(always)]
    pub fn into_value(self) -> crate::PokeValue<'mem> {
        unsafe { crate::PokeValue::new(self.data, self.shape) }
    }

    /// Initialize the option as None
    pub fn init_none(self) -> PokeOption<'mem> {
        let inited = unsafe { (self.vtable().init_none_fn)(self.data) };
        unsafe { PokeOption::new(inited, self.shape, self.def) }
    }

    /// Initialize the option as Some, taking ownership of the given value
    pub fn init_some<'a>(self, mut value: crate::PokeValue<'a>) -> PokeOption<'mem> {
        // Verify that the value type matches what the option expects
        let value_shape = value.shape();
        assert_eq!(self.def.t, value_shape, "Value type mismatch for option");

        // Initialize the option as Some
        let inited = unsafe {
            (self.vtable().init_some_fn)(self.data, Opaque::new(value.data().as_mut_bytes()))
        };
        unsafe { PokeOption::new(inited, self.shape, self.def) }
    }
}

/// Allows poking an option (setting Some/None)
pub struct PokeOption<'mem> {
    data: Opaque<'mem>,
    shape: &'static Shape,
    def: OptionDef,
}

impl<'mem> PokeOption<'mem> {
    /// Creates a new option poke
    ///
    /// # Safety
    ///
    /// `data` must be properly aligned and sized for this shape.
    pub(crate) unsafe fn new(data: Opaque<'mem>, shape: &'static Shape, def: OptionDef) -> Self {
        Self { data, shape, def }
    }

    /// Returns the shape of this option
    pub fn shape(&self) -> &'static Shape {
        self.shape
    }

    /// Returns the option definition
    pub fn def(&self) -> OptionDef {
        self.def
    }

    /// Get a reference to the underlying value
    #[inline(always)]
    pub fn into_value(self) -> crate::PokeValue<'mem> {
        unsafe { crate::PokeValue::new(OpaqueUninit::new(self.data.as_mut_byte_ptr()), self.shape) }
    }
}
