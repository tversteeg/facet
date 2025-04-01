use super::{Opaque, OpaqueConst, OpaqueUninit};

/// Initialize an empty map at the given pointer
///
/// # Safety
///
/// The `target` parameter must have the correct layout and alignment, but points
/// to uninitialized memory. After this returns, the memory is assumed
/// initialized.
pub type MapInitFn = unsafe fn(target: OpaqueUninit, size_hint: Option<usize>);

/// Insert a key-value pair into the map
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapInsertFn = unsafe fn(map: Opaque, key: crate::Partial, value: crate::Partial);

/// Get the number of entries in the map
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapLenFn = unsafe fn(map: OpaqueConst) -> usize;

/// Check if the map contains a key
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapContainsKeyFn = unsafe fn(map: OpaqueConst, key: &str) -> bool;

/// Get pointer to a value for a given key, returns None if not found
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapGetValuePtrFn =
    for<'a> unsafe fn(map: OpaqueConst<'a>, key: &str) -> Option<OpaqueConst<'a>>;

/// Get an iterator over the map
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapIterFn = unsafe fn(map: OpaqueConst) -> OpaqueConst;

/// Get the next key-value pair from the iterator
///
/// # Safety
///
/// The `iter` parameter must point to aligned, initialized memory of the correct type.
pub type MapIterNextFn = unsafe fn(iter: OpaqueConst) -> Option<(*const String, OpaqueConst)>;

/// Deallocate the iterator
///
/// # Safety
///
/// The `iter` parameter must point to aligned, initialized memory of the correct type.
pub type MapIterDeallocFn = unsafe fn(iter: OpaqueConst);

/// VTable for an iterator over a map
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MapIterVTable {
    /// cf. [`MapIterNextFn`]
    pub next: MapIterNextFn,

    /// cf. [`MapIterDeallocFn`]
    pub dealloc: MapIterDeallocFn,
}

/// Virtual table for a Map<String, T>
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MapVTable {
    /// cf. [`MapInitFn`]
    pub init: MapInitFn,

    /// cf. [`MapInsertFn`]
    pub insert: MapInsertFn,

    /// cf. [`MapLenFn`]
    pub len: MapLenFn,

    /// cf. [`MapContainsKeyFn`]
    pub contains_key: MapContainsKeyFn,

    /// cf. [`MapGetValuePtrFn`]
    pub get_value_ptr: MapGetValuePtrFn,

    /// cf. [`MapIterFn`]
    pub iter: MapIterFn,

    /// Virtual table for map iterator operations
    pub iter_vtable: MapIterVTable,
}
