/// Virtual table for a Map<String, T>
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MapVTable {
    /// Initialize an empty map at the given pointer
    pub init: unsafe fn(ptr: *mut u8, size_hint: Option<usize>),

    /// Insert a key-value pair into the map
    pub insert: unsafe fn(*mut u8, key: crate::Partial, value: crate::Partial),

    /// Get the number of entries in the map
    pub len: unsafe fn(ptr: *const u8) -> usize,

    /// Check if the map contains a key
    pub contains_key: unsafe fn(ptr: *const u8, key: &str) -> bool,

    /// Get pointer to a value for a given key, returns null if not found
    pub get_value_ptr: unsafe fn(ptr: *const u8, key: &str) -> *const u8,

    /// Get an iterator over the map
    pub iter: unsafe fn(ptr: *const u8) -> *const u8,

    /// Virtual table for map iterator operations
    pub iter_vtable: MapIterVTable,
}

/// VTable for an iterator over a map
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MapIterVTable {
    /// Get the next key-value pair from the iterator
    pub next: unsafe fn(*const u8) -> Option<(*const String, *const u8)>,

    /// Deallocate the iterator
    pub dealloc: unsafe fn(*const u8),
}
