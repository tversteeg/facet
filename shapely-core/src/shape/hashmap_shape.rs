#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HashMapVTable {
    // Initialize an empty HashMap at the given pointer
    pub init: unsafe fn(ptr: *mut u8, size_hint: Option<usize>),

    // Insert a key-value pair into the HashMap
    pub insert: unsafe fn(*mut u8, key: crate::Partial, value: crate::Partial),

    // Get the number of entries in the HashMap
    pub len: unsafe fn(ptr: *const u8) -> usize,

    // Check if the HashMap contains a key
    pub contains_key: unsafe fn(ptr: *const u8, key: &str) -> bool,

    // Get pointer to a value for a given key, returns null if not found
    pub get_value_ptr: unsafe fn(ptr: *const u8, key: &str) -> *const u8,

    // Get an iterator over the hashmap
    pub iter: unsafe fn(ptr: *const u8) -> *const u8,

    pub iter_vtable: HashMapIterVtable,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HashMapIterVtable {
    // Get the next key-value pair from the iterator
    pub next: unsafe fn(*const u8) -> Option<(*const String, *const u8)>,

    // Deallocate the iterator
    pub dealloc: unsafe fn(*const u8),
}
