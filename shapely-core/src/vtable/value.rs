use crate::{Opaque, OpaqueConst, OpaqueUninit, Peek};
use std::cmp::Ordering;

/// A function that formats the name of a type.
///
/// This helps avoid allocations, and it takes options.
pub type TypeNameFn = fn(f: &mut std::fmt::Formatter, opts: TypeNameOpts) -> std::fmt::Result;

/// Options for formatting the name of a type
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct TypeNameOpts {
    /// as long as this is > 0, keep formatting the type parameters
    /// when it reaches 0, format type parameters as `...`
    /// if negative, all type parameters are formatted
    pub recurse_ttl: isize,
}

impl Default for TypeNameOpts {
    fn default() -> Self {
        Self { recurse_ttl: -1 }
    }
}

impl TypeNameOpts {
    /// Create a new `NameOpts` for which none of the type parameters are formatted
    pub fn none() -> Self {
        Self { recurse_ttl: 0 }
    }

    /// Create a new `NameOpts` for which only the direct children are formatted
    pub fn one() -> Self {
        Self { recurse_ttl: 1 }
    }

    /// Create a new `NameOpts` for which all type parameters are formatted
    pub fn infinite() -> Self {
        Self { recurse_ttl: -1 }
    }

    /// Decrease the `recurse_ttl` — if it's != 0, returns options to pass when
    /// formatting children type parameters.
    ///
    /// If this returns `None` and you have type parameters, you should render a
    /// `…` (unicode ellipsis) character instead of your list of types.
    ///
    /// See the implementation for `Vec` for examples.
    pub fn for_children(&self) -> Option<Self> {
        if self.recurse_ttl > 0 {
            Some(Self {
                recurse_ttl: self.recurse_ttl - 1,
            })
        } else if self.recurse_ttl < 0 {
            Some(Self {
                recurse_ttl: self.recurse_ttl,
            })
        } else {
            None
        }
    }
}

/// Function to format a value for display
///
/// If both [`DisplayFn`] and [`ParseFn`] are set, we should be able to round-trip the value.
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
pub type DisplayFn =
    for<'mem> unsafe fn(value: OpaqueConst<'mem>, f: &mut std::fmt::Formatter) -> std::fmt::Result;

/// Function to format a value for debug.
/// If this returns None, the shape did not implement Debug.
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
pub type DebugFn = for<'mem> unsafe fn(
    value: OpaqueConst<'mem>,
    f: &mut std::fmt::Formatter,
) -> Option<std::fmt::Result>;

/// Default debug fn, just returns None
pub const DEFAULT_DEBUG_FN: DebugFn = |_value, _f| None;

/// Builds a `DebugFn` for a type that implements it (useful when implementing on scalars)
pub const fn debug_fn_for<T: std::fmt::Debug>() -> DebugFn {
    |value, f| {
        Some(<T as std::fmt::Debug>::fmt(
            unsafe { value.as_ref::<T>() },
            f,
        ))
    }
}

/// Function to set a value to its default in-place
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. If this function succeeds, it should return `Some` with the
/// same pointer wrapped in an [`Opaque`].
pub type DefaultInPlaceFn = for<'mem> unsafe fn(target: OpaqueUninit<'mem>) -> Option<Opaque<'mem>>;

/// Function to parse a value from a string.
///
/// If both [`DisplayFn`] and [`ParseFn`] are set, we should be able to round-trip the value.
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. If this function succeeds, it should return `Some` with the
/// same pointer wrapped in an [`Opaque`].
pub type ParseFn = for<'mem> unsafe fn(s: &str, target: OpaqueUninit<'mem>) -> Option<Opaque<'mem>>;

/// Function to try converting from another type
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. If this function succeeds, it should return `Some` with the
/// same pointer wrapped in an [`Opaque`].
pub type TryFromFn = for<'src, 'mem> unsafe fn(
    source: Peek<'src>,
    target: OpaqueUninit<'mem>,
) -> Option<Opaque<'mem>>;

/// Function to check if two values are equal
///
/// # Safety
///
/// Both `left` and `right` parameters must point to aligned, initialized memory of the correct type.
pub type EqFn = for<'l, 'r> unsafe fn(left: OpaqueConst<'l>, right: OpaqueConst<'r>) -> bool;

/// Function to compare two values and return their ordering
///
/// # Safety
///
/// Both `left` and `right` parameters must point to aligned, initialized memory of the correct type.
pub type CmpFn = for<'l, 'r> unsafe fn(left: OpaqueConst<'l>, right: OpaqueConst<'r>) -> Ordering;

/// Function to hash a value
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
/// The hasher pointer must be a valid pointer to a Hasher trait object.
pub type HashFn = for<'mem> unsafe fn(
    value: OpaqueConst<'mem>,
    hasher_this: Opaque<'mem>,
    hasher_write_fn: HasherWriteFn,
);

/// Provides an implementation of [`std::hash::Hasher`] for a given hasher pointer and write function
///
/// See [`HashFn`] for more details on the parameters.
///
/// Example usage (for a type that already implements `Hasher`)
///
/// ```rust,ignore
/// hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
///     value
///         .as_ref::<Self>()
///         .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
/// }),
/// ```
pub struct HasherProxy<'a> {
    hasher_this: Opaque<'a>,
    hasher_write_fn: HasherWriteFn,
}

impl<'a> HasherProxy<'a> {
    /// Create a new `HasherProxy` from a hasher pointer and a write function
    ///
    /// # Safety
    ///
    /// The `hasher_this` parameter must be a valid pointer to a Hasher trait object.
    /// The `hasher_write_fn` parameter must be a valid function pointer.
    pub unsafe fn new(hasher_this: Opaque<'a>, hasher_write_fn: HasherWriteFn) -> Self {
        Self {
            hasher_this,
            hasher_write_fn,
        }
    }
}

impl<'a> std::hash::Hasher for HasherProxy<'a> {
    fn finish(&self) -> u64 {
        unimplemented!("finish is not needed for this implementation")
    }
    fn write(&mut self, bytes: &[u8]) {
        unsafe { (self.hasher_write_fn)(self.hasher_this, bytes) }
    }
}

/// Function to write bytes to a hasher
///
/// # Safety
///
/// The `hasher_self` parameter must be a valid pointer to a hasher
pub type HasherWriteFn = for<'mem> unsafe fn(hasher_self: Opaque<'mem>, bytes: &[u8]);

/// Function to drop a value
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
pub type DropInPlaceFn = for<'mem> unsafe fn(value: Opaque<'mem>);

/// VTable for common operations that can be performed on any shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueVTable {
    /// cf. [`TypeNameFn`]
    pub type_name: TypeNameFn,

    /// cf. [`DisplayFn`]
    pub display: Option<DisplayFn>,

    /// cf. [`DebugFn`]
    pub debug: DebugFn,

    /// cf. [`DefaultInPlaceFn`]
    pub default_in_place: Option<DefaultInPlaceFn>,

    /// cf. [`EqFn`]
    pub eq: Option<EqFn>,

    /// cf. [`CmpFn`]
    pub cmp: Option<CmpFn>,

    /// cf. [`HashFn`]
    pub hash: Option<HashFn>,

    /// cf. [`DropInPlaceFn`] — if None, drops without side-effects
    pub drop_in_place: Option<DropInPlaceFn>,

    /// cf. [`ParseFn`]
    pub parse: Option<ParseFn>,

    /// cf. [`TryFromFn`]
    pub try_from: Option<TryFromFn>,
}
