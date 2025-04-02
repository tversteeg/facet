use crate::{ListDef, ListVTable, Opaque, ShapeDesc};

/// Allows poking a list (appending, etc.)
pub struct PokeList<'mem> {
    data: Opaque<'mem>,
    shape_desc: ShapeDesc,
    def: ListDef,
}

impl<'mem> PokeList<'mem> {
    /// Creates a new list write-proxy
    ///
    /// # Safety
    ///
    /// The data buffer must match the size and alignment of the shape.
    pub(crate) unsafe fn new(data: Opaque<'mem>, shape_desc: ShapeDesc, def: ListDef) -> Self {
        Self {
            data,
            shape_desc,
            def,
        }
    }

    /// Gets the vtable for the list
    #[inline(always)]
    fn list_vtable(&self) -> ListVTable {
        (self.def.vtable)()
    }

    /// Initialize the list in place with a given capacity
    ///
    /// # Safety
    ///
    /// The list must point to uninitialized memory of sufficient size.
    /// The function must properly initialize the memory.
    pub unsafe fn init_in_place_with_capacity(&mut self, capacity: usize) {
        unsafe { (self.list_vtable().init_in_place_with_capacity)(self.data, capacity) }
    }

    /// Pushes an item to the list
    ///
    /// # Safety
    ///
    /// `item` is moved out of (with [`std::ptr::read`]) — it should be deallocated
    /// afterwards but NOT dropped.
    pub unsafe fn push<'src>(&mut self, item: Opaque<'src>) {
        unsafe { (self.list_vtable().push)(self.data, item) }
    }

    /// Gets the number of items in the list
    pub fn len(&self) -> usize {
        unsafe { (self.list_vtable().len)(self.data.as_const()) }
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
        unsafe { (self.list_vtable().get_item_ptr)(self.data.as_const(), index) }
    }
}
