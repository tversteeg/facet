use crate::opaque::{Opaque, OpaqueConst, OpaqueUninit};

/// Check if an option contains a value
///
/// # Safety
///
/// The `option` parameter must point to aligned, initialized memory of the correct type.
pub type OptionIsSomeFn = for<'option> unsafe fn(option: OpaqueConst<'option>) -> bool;

/// Get the value contained in an option, if present
///
/// # Safety
///
/// The `option` parameter must point to aligned, initialized memory of the correct type.
pub type OptionGetValueFn =
    for<'option> unsafe fn(option: OpaqueConst<'option>) -> Option<OpaqueConst<'option>>;

/// Initialize an option with Some(value)
///
/// # Safety
///
/// The `option` parameter must point to uninitialized memory of sufficient size.
/// The function must properly initialize the memory.
/// `value` is moved out of (with [`core::ptr::read`]) — it should be deallocated
/// afterwards but NOT dropped.
pub type OptionInitSomeFn =
    for<'option> unsafe fn(option: OpaqueUninit<'option>, value: Opaque<'_>) -> Opaque<'option>;

/// Initialize an option with None
///
/// # Safety
///
/// The `option` parameter must point to uninitialized memory of sufficient size.
/// The function must properly initialize the memory.
pub type OptionInitNoneFn = unsafe fn(option: OpaqueUninit) -> Opaque;

/// Virtual table for Option<T>
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct OptionVTable {
    /// cf. [`OptionIsSomeFn`]
    pub is_some_fn: OptionIsSomeFn,

    /// cf. [`OptionGetValueFn`]
    pub get_value_fn: OptionGetValueFn,

    /// cf. [`OptionInitSomeFn`]
    pub init_some_fn: OptionInitSomeFn,

    /// cf. [`OptionInitNoneFn`]
    pub init_none_fn: OptionInitNoneFn,
}

impl OptionVTable {
    /// Returns a builder for OptionVTable
    pub const fn builder() -> OptionVTableBuilder {
        OptionVTableBuilder::new()
    }
}

/// Builds an [`OptionVTable`]
pub struct OptionVTableBuilder {
    is_some_fn: Option<OptionIsSomeFn>,
    get_value_fn: Option<OptionGetValueFn>,
    init_some_fn: Option<OptionInitSomeFn>,
    init_none_fn: Option<OptionInitNoneFn>,
}

impl OptionVTableBuilder {
    /// Creates a new [`OptionVTableBuilder`] with all fields set to `None`.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            is_some_fn: None,
            get_value_fn: None,
            init_some_fn: None,
            init_none_fn: None,
        }
    }

    /// Sets the is_some_fn field
    pub const fn is_some(mut self, f: OptionIsSomeFn) -> Self {
        self.is_some_fn = Some(f);
        self
    }

    /// Sets the get_value_fn field
    pub const fn get_value(mut self, f: OptionGetValueFn) -> Self {
        self.get_value_fn = Some(f);
        self
    }

    /// Sets the init_some_fn field
    pub const fn init_some(mut self, f: OptionInitSomeFn) -> Self {
        self.init_some_fn = Some(f);
        self
    }

    /// Sets the init_none_fn field
    pub const fn init_none(mut self, f: OptionInitNoneFn) -> Self {
        self.init_none_fn = Some(f);
        self
    }

    /// Builds the [`OptionVTable`] from the current state of the builder.
    ///
    /// # Panics
    ///
    /// This method will panic if any of the required fields are `None`.
    pub const fn build(self) -> OptionVTable {
        OptionVTable {
            is_some_fn: self.is_some_fn.unwrap(),
            get_value_fn: self.get_value_fn.unwrap(),
            init_some_fn: self.init_some_fn.unwrap(),
            init_none_fn: self.init_none_fn.unwrap(),
        }
    }
}
