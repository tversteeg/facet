use crate::{MapVTable, Opaque, OpaqueConst, Shape, ValueVTable};

/// Allows poking a map (inserting, etc.)
pub struct PokeMap<'mem> {
    data: Opaque<'mem>,
    #[allow(dead_code)]
    shape: Shape,
    #[allow(dead_code)]
    vtable: ValueVTable,
    map_vtable: MapVTable,
}

impl<'mem> PokeMap<'mem> {
    /// Inserts a key-value pair into the map
    ///
    /// # Safety
    ///
    /// `key` and `value` are moved out of (with [`std::ptr::read`]) — they should be deallocated
    /// afterwards but NOT dropped.
    pub unsafe fn insert<'key, 'value>(&mut self, key: Opaque<'key>, value: Opaque<'value>) {
        unsafe { (self.map_vtable.insert)(self.data, key, value) }
    }

    /// Gets the number of entries in the map
    pub fn len(&self) -> usize {
        unsafe { (self.map_vtable.len)(self.data.as_const()) }
    }

    /// Checks if the map contains no entries
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if the map contains a key
    pub fn contains_key<'key>(&self, key: OpaqueConst<'key>) -> bool {
        unsafe { (self.map_vtable.contains_key)(self.data.as_const(), key) }
    }

    /// Gets a pointer to the value for a given key
    ///
    /// Returns `None` if the key is not found.
    pub fn get_value_ptr<'key>(&self, key: OpaqueConst<'key>) -> Option<OpaqueConst<'mem>> {
        unsafe { (self.map_vtable.get_value_ptr)(self.data.as_const(), key) }
    }
}
