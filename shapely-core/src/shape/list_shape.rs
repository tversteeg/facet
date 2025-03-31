#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ListVTable {
    /// init given pointer to be an empty vec (with capacity)
    pub init: unsafe fn(ptr: *mut u8, size_hint: Option<usize>),

    /// push an item
    pub push: unsafe fn(*mut u8, crate::Partial),

    /// get length of the collection
    pub len: unsafe fn(ptr: *const u8) -> usize,

    /// get address of the item at the given index. panics if out of bound.
    pub get_item_ptr: unsafe fn(ptr: *const u8, index: usize) -> *const u8,
}
