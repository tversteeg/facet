use crate::{ListDef, ListVTable, Opaque, Shape, ValueVTable};

/// Allows poking a list (appending, etc.)
pub struct PokeList<'mem> {
    data: Opaque<'mem>,
    #[allow(dead_code)]
    shape: Shape,
    #[allow(dead_code)]
    vtable: ValueVTable,
    list_def: ListDef,
    list_vtable: ListVTable,
}

impl<'mem> PokeList<'mem> {
    /// Creates a new list write-proxy
    ///
    /// # Safety
    ///
    /// The data buffer must match the size and alignment of the shape.
    pub(crate) unsafe fn new(
        data: Opaque<'mem>,
        shape: Shape,
        vtable: ValueVTable,
        list_def: ListDef,
    ) -> Self {
        let list_vtable = (list_def.vtable)();
        Self {
            data,
            shape,
            vtable,
            list_def,
            list_vtable,
        }
    }

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

    /// Returns true if the list is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
