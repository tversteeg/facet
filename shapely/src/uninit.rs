use crate::{Field, Shape, Shapely, Slot};
use std::alloc;

/// A partially-initialized shape, useful when deserializing for example.
pub struct ShapeUninit {
    /// Address of the value in memory
    pub(crate) addr: *mut u8,

    /// Keeps track of which fields are initialized
    pub(crate) init_fields: InitSet64,

    /// The shape we're building.
    pub(crate) shape: Shape,
}

impl Drop for ShapeUninit {
    fn drop(&mut self) {
        if !self.addr.is_null() {
            let layout = alloc::Layout::from_size_align(self.shape.size, self.shape.align).unwrap();
            unsafe { alloc::dealloc(self.addr, layout) }
        }
    }
}

impl ShapeUninit {
    /// Returns a pointer to the underlying data.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `self` outlives the returned pointer
    /// - The returned pointer is not aliased
    /// - The provided shape matches the shape of the data
    ///
    /// This function performs a cast and of course you could get it to do UB if you expected a different type.
    pub unsafe fn get_addr(&self, expected_shape: &Shape) -> *mut u8 {
        if self.shape == *expected_shape {
            self.addr
        } else {
            panic!(
                "Shape mismatch: expected {:?}, found {:?}",
                expected_shape, self.shape
            )
        }
    }

    fn check_initialization(&self) {
        if let crate::Innards::Struct { fields } = self.shape.innards {
            for (i, field) in fields.iter().enumerate() {
                if !self.init_fields.is_set(i) {
                    panic!(
                        "Field '{}' was not initialized. Complete schema:\n{:?}",
                        field.name, self.shape
                    );
                }
            }
        }
    }

    pub fn slot<'s>(&'s mut self, field: Field) -> Option<Slot<'s>> {
        match self.shape.innards {
            crate::Innards::Struct { fields } => {
                if let Some(field) = fields.iter().find(|f| f.name == field.name) {
                    if let Some(offset) = field.offset {
                        let field_addr = unsafe { self.addr.add(offset.get() as usize) };
                        let init_field_slot = InitFieldSlot::Struct {
                            index: fields.iter().position(|f| f.name == field.name).unwrap(),
                            set: &mut self.init_fields,
                        };
                        let slot =
                            Slot::for_struct_field(field_addr, field.schema, init_field_slot);
                        Some(slot)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            crate::Innards::HashMap { value_shape } => {
                // Create a slot for inserting into the HashMap
                let init_field_slot = InitFieldSlot::Ignored;
                let slot = Slot::for_hash_map(
                    self.addr,
                    value_shape,
                    field.name.to_string(),
                    init_field_slot,
                );
                Some(slot)
            }
            crate::Innards::Array(_shape) => None,
            crate::Innards::Transparent(_shape) => None,
            crate::Innards::Scalar(_scalar) => None,
        }
    }

    pub fn build<T: Shapely>(self) -> T {
        self.check_initialization();

        if self.shape != T::shape() {
            panic!(
                "Shape mismatch: expected {:?}, found {:?}",
                T::shape(),
                self.shape
            );
        }

        let result = unsafe { std::ptr::read(self.addr as *const T) };
        std::mem::forget(self);
        result
    }

    pub fn build_boxed<T: Shapely>(self) -> Box<T> {
        self.check_initialization();

        if self.shape != T::shape() {
            panic!(
                "Shape mismatch: expected {:?}, found {:?}",
                T::shape(),
                self.shape
            );
        }

        let boxed = unsafe { Box::from_raw(self.addr as *mut T) };
        std::mem::forget(self);
        boxed
    }
}

/// A bit array to keep track of which fields were initialized
#[derive(Clone, Copy)]
pub struct InitSet64(u64);

impl InitSet64 {
    pub fn new() -> Self {
        InitSet64(0)
    }

    pub fn set(&mut self, index: usize) {
        if index < 64 {
            self.0 |= 1 << index;
        }
    }

    pub fn is_set(&self, index: usize) -> bool {
        if index < 64 {
            (self.0 & (1 << index)) != 0
        } else {
            false
        }
    }

    pub fn all_set(&self, count: usize) -> bool {
        if count <= 64 {
            let mask = (1 << count) - 1;
            self.0 & mask == mask
        } else {
            false
        }
    }
}

pub enum InitFieldSlot<'s> {
    Struct {
        index: usize,
        set: &'s mut InitSet64,
    },
    Ignored,
}

impl InitFieldSlot<'_> {
    pub fn mark_initialized(&mut self) {
        match self {
            InitFieldSlot::Struct { index, set } => set.set(*index),
            InitFieldSlot::Ignored => {}
        }
    }
}
