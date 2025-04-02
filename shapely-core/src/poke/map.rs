use crate::{MapDef, MapVTable, Opaque, OpaqueConst, ShapeDesc};

/// Allows poking a map (inserting, etc.)
pub struct PokeMap<'mem> {
    data: Opaque<'mem>,
    shape_desc: ShapeDesc,
    def: MapDef,
}

impl<'mem> PokeMap<'mem> {
    /// Creates a value-proxy for a map
    ///
    /// # Safety
    ///
    /// The data buffer must match the size and alignment of the shape.
    #[inline]
    pub(crate) unsafe fn new(data: Opaque<'mem>, shape_desc: ShapeDesc, def: MapDef) -> Self {
        Self {
            data,
            shape_desc,
            def,
        }
    }

    /// Gets the vtable for the map
    #[inline(always)]
    pub fn map_vtable(&self) -> MapVTable {
        (self.def.vtable)()
    }

    /// Inserts a key-value pair into the map
    ///
    /// # Safety
    ///
    /// `key` and `value` are moved out of (with [`std::ptr::read`]) — they should be deallocated
    /// afterwards but NOT dropped.
    #[inline]
    pub unsafe fn insert<'key, 'value>(&mut self, key: Opaque<'key>, value: Opaque<'value>) {
        unsafe { (self.map_vtable().insert)(self.data, key, value) }
    }

    /// Gets the number of entries in the map
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { (self.map_vtable().len)(self.data.as_const()) }
    }

    /// Checks if the map contains no entries
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if the map contains a key
    #[inline]
    pub fn contains_key<'key>(&self, key: OpaqueConst<'key>) -> bool {
        unsafe { (self.map_vtable().contains_key)(self.data.as_const(), key) }
    }

    /// Gets a pointer to the value for a given key
    ///
    /// Returns `None` if the key is not found.
    #[inline]
    pub fn get_value_ptr<'key>(&self, key: OpaqueConst<'key>) -> Option<OpaqueConst<'mem>> {
        unsafe { (self.map_vtable().get_value_ptr)(self.data.as_const(), key) }
    }
}
