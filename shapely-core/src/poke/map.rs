use crate::{MapDef, MapVTable, Opaque, OpaqueConst, OpaqueUninit, PokeValue, ShapeFn};

/// Allows initializing an uninitialized map
pub struct PokeMapUninit<'mem> {
    data: OpaqueUninit<'mem>,
    shape_fn: ShapeFn,
    def: MapDef,
}

impl<'mem> PokeMapUninit<'mem> {
    /// Creates a new uninitialized map write-proxy
    ///
    /// # Safety
    ///
    /// The data buffer must match the size and alignment of the shape.
    pub(crate) unsafe fn new(data: OpaqueUninit<'mem>, shape_fn: ShapeFn, def: MapDef) -> Self {
        Self {
            data,
            shape_fn,
            def,
        }
    }

    /// Initializes the map with an optional size hint
    pub fn init(self, size_hint: Option<usize>) -> Result<PokeMap<'mem>, OpaqueUninit<'mem>> {
        let res = if let Some(capacity) = size_hint {
            let init_in_place_with_capacity = self.def.vtable().init_in_place_with_capacity;
            unsafe { init_in_place_with_capacity(self.data, capacity) }
        } else {
            let pv = unsafe { PokeValue::new(self.data, self.shape_fn) };
            pv.default_in_place().map_err(|_| ())
        };
        let data = res.map_err(|_| self.data)?;
        Ok(unsafe { PokeMap::new(data, self.shape_fn, self.def) })
    }
}

/// Allows poking a map (inserting, etc.)
pub struct PokeMap<'mem> {
    data: Opaque<'mem>,
    shape_fn: ShapeFn,
    def: MapDef,
}

impl<'mem> PokeMap<'mem> {
    /// Creates a value-proxy for a map
    ///
    /// # Safety
    ///
    /// The data buffer must match the size and alignment of the shape.
    #[inline]
    pub(crate) unsafe fn new(data: Opaque<'mem>, shape_fn: ShapeFn, def: MapDef) -> Self {
        Self {
            data,
            shape_fn,
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
