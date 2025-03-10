use crate::{Field, ShapeDesc, Shapely, Slot};
use std::{alloc, marker::PhantomData, ptr::NonNull};

pub enum MemSource {
    Borrowed,
    HeapAllocated,
}

/// A partially-initialized shape, useful when deserializing for example.
pub struct ShapeUninit<'s> {
    pub(crate) source: MemSource,

    pub(crate) phantom: PhantomData<&'s ()>, // NonNull is covariant...

    /// Address of the value in memory
    pub(crate) addr: NonNull<u8>,

    /// Keeps track of which fields are initialized
    pub(crate) init_fields: InitSet64,

    /// The shape we're building.
    pub(crate) shape_desc: ShapeDesc,
}

impl Drop for ShapeUninit<'_> {
    fn drop(&mut self) {
        if matches!(self.source, MemSource::HeapAllocated) {
            unsafe { alloc::dealloc(self.addr.as_ptr(), self.shape_desc.get().layout) }
        }
    }
}

impl ShapeUninit<'_> {
    /// Returns a pointer to the underlying data, if the shape matches the expected shape.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `self` outlives the returned pointer
    /// - The returned pointer is not aliased
    /// - The provided shape matches the shape of the data
    pub unsafe fn as_ptr(&self, expected_desc: ShapeDesc) -> *mut u8 {
        if self.shape_desc == expected_desc {
            self.addr.as_ptr()
        } else {
            panic!(
                "Shape mismatch: expected {:?}, found {:?}",
                expected_desc.get(),
                self.shape_desc.get()
            )
        }
    }

    fn check_initialization(&self) {
        match self.shape_desc.get().innards {
            crate::Innards::Struct { fields } => {
                for (i, field) in fields.iter().enumerate() {
                    if !self.init_fields.is_set(i) {
                        panic!(
                            "Field '{}' was not initialized. Complete schema:\n{:?}",
                            field.name,
                            self.shape_desc.get()
                        );
                    }
                }
            }
            crate::Innards::Scalar(_) => {
                if !self.init_fields.is_set(0) {
                    panic!(
                        "Scalar value was not initialized. Complete schema:\n{:?}",
                        self.shape_desc.get()
                    );
                }
            }
            _ => {}
        }
    }

    /// Returns a slot for assigning this whole shape as a scalar
    pub fn scalar_slot(&mut self) -> Option<Slot<'_>> {
        match self.shape_desc.get().innards {
            crate::Innards::Scalar(_) => {
                let slot = Slot::for_struct_field(
                    self.addr,
                    self.shape_desc,
                    InitFieldSlot::Struct {
                        index: 0,
                        set: &mut self.init_fields,
                    },
                );
                Some(slot)
            }
            _ => panic!(
                "Expected scalar innards, found {:?}",
                self.shape_desc.get().innards
            ),
        }
    }

    /// Returns a slot for initializing a field in the shape.
    pub fn slot<'s>(&'s mut self, field: Field) -> Option<Slot<'s>> {
        match self.shape_desc.get().innards {
            crate::Innards::Struct { fields } => {
                if let Some((index, field)) = fields
                    .iter()
                    .enumerate()
                    .find(|(_, f)| f.name == field.name)
                {
                    if let Some(offset) = field.offset {
                        let field_addr = unsafe { self.addr.add(offset.get() as usize) };
                        let init_field_slot = InitFieldSlot::Struct {
                            index,
                            set: &mut self.init_fields,
                        };
                        let slot = Slot::for_struct_field(field_addr, field.shape, init_field_slot);
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

        if self.shape_desc != T::shape_desc() {
            panic!(
                "We were building a {:?}\n..but .build() was called expecting a {:?}",
                self.shape_desc.get(),
                T::shape(),
            );
        }

        let result = unsafe { std::ptr::read(self.addr.as_ptr() as *const T) };
        std::mem::forget(self);
        result
    }

    pub fn build_boxed<T: Shapely>(self) -> Box<T> {
        self.check_initialization();

        if self.shape_desc != T::shape_desc() {
            panic!(
                "We were building a {:?}\n..but .build_boxed() was called expecting a {:?}",
                self.shape_desc.get(),
                T::shape(),
            );
        }

        let boxed = unsafe { Box::from_raw(self.addr.as_ptr() as *mut T) };
        std::mem::forget(self);
        boxed
    }
}

/// A bit array to keep track of which fields were initialized, up to 64 fields
#[derive(Clone, Copy, Default)]
pub struct InitSet64(u64);

impl InitSet64 {
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
    pub fn mark_as_init(&mut self) {
        match self {
            InitFieldSlot::Struct { index, set } => set.set(*index),
            InitFieldSlot::Ignored => {}
        }
    }

    pub fn is_init(&self) -> bool {
        match self {
            InitFieldSlot::Struct { index, set } => set.is_set(*index),
            InitFieldSlot::Ignored => true,
        }
    }
}
