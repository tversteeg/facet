use super::{Opaque, OpaqueConst, OpaqueUninit};

/// Initialize an empty list at the given pointer
///
/// # Safety
///
/// The `target` parameter must have the correct layout and alignment, but points
/// to uninitialized memory. After this returns, the memory is assumed
/// initialized.
pub type ListInitFn = unsafe fn(target: OpaqueUninit, size_hint: Option<usize>);

/// Push an item to the list
///
/// # Safety
///
/// The `list` parameter must point to aligned, initialized memory of the correct type.
pub type ListPushFn = unsafe fn(list: Opaque, item: crate::Partial);

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
    /// cf. [`ListInitFn`]
    pub init: ListInitFn,

    /// cf. [`ListPushFn`]
    pub push: ListPushFn,

    /// cf. [`ListLenFn`]
    pub len: ListLenFn,

    /// cf. [`ListGetItemPtrFn`]
    pub get_item_ptr: ListGetItemPtrFn,
}
