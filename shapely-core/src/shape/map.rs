use super::{Opaque, OpaqueConst};

/// Insert a key-value pair into the map
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapInsertFn = for<'map, 'key, 'value> unsafe fn(
    map: Opaque<'map>,
    key: crate::Partial<'key>,
    value: crate::Partial<'value>,
);

/// Get the number of entries in the map
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapLenFn = for<'map> unsafe fn(map: OpaqueConst<'map>) -> usize;

/// Check if the map contains a key
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapContainsKeyFn =
    for<'map, 'key> unsafe fn(map: OpaqueConst<'map>, key: OpaqueConst<'key>) -> bool;

/// Get pointer to a value for a given key, returns None if not found
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapGetValuePtrFn = for<'map, 'key> unsafe fn(
    map: OpaqueConst<'map>,
    key: OpaqueConst<'key>,
) -> Option<OpaqueConst<'map>>;

/// Get an iterator over the map
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapIterFn = for<'map> unsafe fn(map: OpaqueConst<'map>) -> OpaqueConst<'map>;

/// Get the next key-value pair from the iterator
///
/// # Safety
///
/// The `iter` parameter must point to aligned, initialized memory of the correct type.
pub type MapIterNextFn =
    for<'iter> unsafe fn(iter: Opaque<'iter>) -> Option<(OpaqueConst<'iter>, OpaqueConst<'iter>)>;

/// Deallocate the iterator
///
/// # Safety
///
/// The `iter` parameter must point to aligned, initialized memory of the correct type.
pub type MapIterDeallocFn = for<'iter> unsafe fn(iter: Opaque<'iter>);

/// VTable for an iterator over a map
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MapIterVTable {
    /// cf. [`MapIterNextFn`]
    pub next: MapIterNextFn,

    /// cf. [`MapIterDeallocFn`]
    pub dealloc: MapIterDeallocFn,
}

/// Virtual table for a Map<K, V>
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MapVTable {
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
