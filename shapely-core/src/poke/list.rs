use crate::{ListVTable, Opaque, Shape, ValueVTable};

/// Allows poking a list (appending, etc.)
pub struct PokeList<'mem> {
    pub data: Opaque<'mem>,
    pub shape: Shape,
    pub vtable: ValueVTable,
    pub list_vtable: ListVTable,
}

impl<'mem> PokeList<'mem> {
    /// Pushes an item to the list
    ///
    /// # Safety
    ///
    /// `item` is moved out of (with [`std::ptr::read`]) — it should be deallocated
    /// afterwards but NOT dropped.
    pub unsafe fn push<'src>(&mut self, item: Opaque<'src>) {
        unsafe { (self.list_vtable.push)(self.data, item) }
    }

    /// Gets the number of items in the list
    pub fn len(&self) -> usize {
        unsafe { (self.list_vtable.len)(self.data.as_const()) }
    }

    /// Gets a pointer to the item at the given index
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn get_item_ptr(&self, index: usize) -> crate::OpaqueConst {
        unsafe { (self.list_vtable.get_item_ptr)(self.data.as_const(), index) }
    }
}
