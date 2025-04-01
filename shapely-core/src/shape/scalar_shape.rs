use super::{OpaqueConst, OpaqueUninit, ShapeDesc};
use std::cmp::Ordering;

/// Writes a `Display` format of the scalar to the given formatter. The assumption is that
/// if there's a [`ScalarFromStrFn`] function, it should be able to parse the output
/// of this function.
///
/// # Safety
///
/// The `this` parameter must point to aligned, initialized memory of the correct type.
pub type ScalarDisplayFn = unsafe fn(this: OpaqueConst, f: std::fmt::Formatter) -> String;

/// Writes a `Debug` format of the scalar to the given formatter
///
/// # Safety
///
/// The `this` parameter must point to aligned, initialized memory of the correct type.
pub type ScalarDebugFn = unsafe fn(this: OpaqueConst, f: std::fmt::Formatter) -> String;

/// Function to set a scalar to its default value in-place
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. After this function is called, the memory is assumed initialized.
pub type ScalarDefaultInPlaceFn = unsafe fn(target: OpaqueUninit);

/// Function to create a scalar from a string representation
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory.
pub type ScalarFromStrFn = unsafe fn(target: OpaqueUninit, s: &str) -> Result<(), String>;

/// Function to check if two scalar values are equal
///
/// # Safety
///
/// Both `left` and `right` parameters must point to aligned, initialized memory of the correct type.
pub type ScalarEqFn = unsafe fn(left: OpaqueConst, right: OpaqueConst) -> bool;

/// Function to compare two scalar values and return their ordering
///
/// # Safety
///
/// Both `left` and `right` parameters must point to aligned, initialized memory of the correct type.
pub type ScalarCmpFn = unsafe fn(left: OpaqueConst, right: OpaqueConst) -> Ordering;

/// Function to hash a scalar value
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
/// The hasher pointer must be a valid pointer to a Hasher trait object.
pub type ScalarHashFn = unsafe fn(value: OpaqueConst, hasher: *const dyn std::hash::Hasher);

/// Function to try to convert a value from one scalar type to another
///
/// # Safety
///
/// The `src` parameter must point to aligned, initialized memory of the type described by `src_shape`.
/// The `dst` parameter must point to aligned, uninitialized memory of the correct type.
/// If this function returns `Ok(())`, the memory at `dst` is considered initialized.
pub type ScalarTryFromFn =
    unsafe fn(src_shape: ShapeDesc, src: OpaqueConst, dst: OpaqueUninit) -> Result<(), ()>;

pub enum TryFromError {
    /// Those two types have nothing in common
    UnrelatedTypes { src: ShapeDesc, dst: ShapeDesc },

    /// A similar conversion is possible, but the src was out of range
    OutOfRange {
        src: ShapeDesc,
        dst: ShapeDesc,
        min_value: OpaqueConst<'static>,
        max_value: OpaqueConst<'static>,
    },
}

/// VTable for a scalar: common operations we can do with a scalar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScalarVTable {
    /// cf. [`ScalarDisplayFn`]
    pub display: Option<ScalarDisplayFn>,

    /// cf. [`ScalarDebugFn`]
    pub debug: Option<ScalarDebugFn>,

    /// cf. [`ScalarDefaultInPlaceFn`]
    pub default_in_place: Option<ScalarDefaultInPlaceFn>,

    /// cf. [`ScalarFromStrFn`]
    pub from_str: Option<ScalarFromStrFn>,

    /// cf. [`ScalarEqFn`]
    pub eq: Option<ScalarEqFn>,

    /// cf. [`ScalarCmpFn`]
    pub cmp: Option<ScalarCmpFn>,

    /// cf. [`ScalarHashFn`]
    pub hash: Option<ScalarHashFn>,
}
