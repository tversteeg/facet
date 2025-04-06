use facet_opaque::{Opaque, OpaqueConst, OpaqueUninit};

/// Initialize a list in place with a given capacity
///
/// # Safety
///
/// The `list` parameter must point to uninitialized memory of sufficient size.
/// The function must properly initialize the memory.
pub type ListInitInPlaceWithCapacityFn =
    unsafe fn(list: OpaqueUninit, capacity: usize) -> Result<Opaque, ()>;

/// Push an item to the list
///
/// # Safety
///
/// The `list` parameter must point to aligned, initialized memory of the correct type.
/// `item` is moved out of (with [`std::ptr::read`]) — it should be deallocated
/// afterwards but NOT dropped.
pub type ListPushFn = unsafe fn(list: Opaque, item: Opaque);
// FIXME: this forces allocating item separately, copying it, and then dropping it — it's not great.

/// Get the number of items in the list
///
/// # Safety
///
/// The `list` parameter must point to aligned, initialized memory of the correct type.
pub type ListLenFn = unsafe fn(list: OpaqueConst) -> usize;

/// Get pointer to the item at the given index. Panics if out of bounds.
///
/// # Safety
///
/// The `list` parameter must point to aligned, initialized memory of the correct type.
pub type ListGetItemPtrFn = unsafe fn(list: OpaqueConst, index: usize) -> OpaqueConst;

/// Virtual table for a list-like type (like `Vec<T>`,
/// but also `HashSet<T>`, etc.)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ListVTable {
    /// cf. [`ListInitInPlaceWithCapacityFn`]
    pub init_in_place_with_capacity: ListInitInPlaceWithCapacityFn,

    /// cf. [`ListPushFn`]
    pub push: ListPushFn,

    /// cf. [`ListLenFn`]
    pub len: ListLenFn,

    /// cf. [`ListGetItemPtrFn`]
    pub get_item_ptr: ListGetItemPtrFn,
}
