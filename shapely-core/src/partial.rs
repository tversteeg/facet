use crate::{trace, Field, ShapeDesc, Shapely, Slot};
use std::{alloc, marker::PhantomData, ptr::NonNull};

/// Origin of the partial — did we allocate it? Or is it borrowed?
pub enum Origin<'s> {
    Borrowed {
        parent: Option<&'s Partial<'s>>,
        init_field_slot: InitFieldSlot<'s>,
    },
    HeapAllocated,
}

/// A partially-initialized shape, useful when deserializing for example.
pub struct Partial<'s> {
    pub(crate) origin: Origin<'s>,

    pub(crate) phantom: PhantomData<&'s ()>, // NonNull is covariant...

    /// Address of the value in memory. If None, the value is zero-sized.
    pub(crate) addr: Option<NonNull<u8>>,

    /// Keeps track of which fields are initialized
    pub(crate) init_fields: InitSet64,

    /// The shape we're building.
    pub(crate) shape_desc: ShapeDesc,
}

/// We can build a tree of partials as the parsing process occurs or deserization occurs, which means they have to be covariant.
fn _assert_partial_covariant<'long: 'short, 'short>(partial: Partial<'long>) -> Partial<'short> {
    partial
}

impl Drop for Partial<'_> {
    fn drop(&mut self) {
        // First drop any initialized fields
        match self.shape_desc.get().innards {
            crate::Innards::Struct { fields } => {
                for (i, field) in fields.iter().enumerate() {
                    if self.init_fields.is_set(i) && field.offset.is_some() {
                        // Drop the field using its drop function if available
                        if let Some(drop_fn) = field.shape.get().drop_in_place {
                            let field_addr = {
                                let offset = field.offset.unwrap().get() as usize;
                                if let Some(addr) = self.addr {
                                    unsafe { addr.byte_add(offset) }
                                } else {
                                    // Zero-sized types don't have a valid address
                                    NonNull::dangling()
                                }
                            };
                            drop_fn(field_addr.as_ptr());
                        }
                    }
                }
            }
            crate::Innards::Scalar(_) => {
                if self.init_fields.is_set(0) {
                    // Drop the scalar value if it has a drop function
                    if let Some(drop_fn) = self.shape_desc.get().drop_in_place {
                        // Use NonNull::dangling() if we don't have an addr.
                        // This is safe because for zero-sized types, the actual address doesn't matter.
                        // The drop function for zero-sized types should not dereference the pointer.
                        let ptr = self.addr.map_or(NonNull::dangling(), |addr| addr).as_ptr();
                        drop_fn(ptr);
                    }
                }
            }
            _ => {}
        }

        self.deallocate()
    }
}

/// Errors encountered when calling `slot_by_index` or `slot_by_key`
#[derive(Debug)]
pub enum SlotError {
    /// `slot_by_index` was called on a dynamic collection, that has no
    /// static fields. a HashMap doesn't have a "first slot", it can only
    /// associate by keys.
    NoStaticFields,

    /// `slot_by_key` was called on a struct, and there is no static field
    /// with the given key.
    NoSuchStaticField,

    /// `slot_by_index` was called on a fixed-size collection (like a tuple,
    /// a struct, or a fixed-size array) and the index was out of bounds.
    OutOfBounds,
}

impl std::error::Error for SlotError {}

impl std::fmt::Display for SlotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlotError::NoStaticFields => write!(f, "No static fields available"),
            SlotError::NoSuchStaticField => write!(f, "No such static field"),
            SlotError::OutOfBounds => write!(f, "Index out of bounds"),
        }
    }
}

impl Partial<'_> {
    /// Allocates a partial on the heap for the given shape descriptor.
    pub fn alloc(shape_desc: ShapeDesc) -> Self {
        let layout = shape_desc.get().layout;
        let addr = if layout.size() != 0 {
            let addr = unsafe { alloc::alloc(layout) };
            if addr.is_null() {
                alloc::handle_alloc_error(layout);
            }
            Some(NonNull::new(addr).unwrap())
        } else {
            None
        };
        Self {
            origin: Origin::HeapAllocated,
            phantom: PhantomData,
            addr,
            init_fields: Default::default(),
            shape_desc,
        }
    }

    /// Borrows a `MaybeUninit<Self>` and returns a `Partial`.
    ///
    /// Before calling assume_init, make sure to call Partial.build_in_place().
    pub fn borrow<T: Shapely>(uninit: &mut std::mem::MaybeUninit<T>) -> Self {
        Self {
            origin: Origin::Borrowed {
                parent: None,
                init_field_slot: InitFieldSlot::Ignored,
            },
            phantom: PhantomData,
            addr: Some(NonNull::new(uninit.as_mut_ptr() as _).unwrap()),
            init_fields: Default::default(),
            shape_desc: T::shape_desc(),
        }
    }

    /// Checks if all fields in the struct or scalar value have been initialized.
    /// Panics if any field is not initialized, providing details about the uninitialized field.
    pub(crate) fn check_initialization(&self) {
        trace!(
            "Checking initialization of \x1b[1;33m{}\x1b[0m partial at addr \x1b[1;36m{:p}\x1b[0m",
            self.shape_desc.get().name,
            self.addr_for_display()
        );
        match self.shape_desc.get().innards {
            crate::Innards::Struct { fields } => {
                for (i, field) in fields.iter().enumerate() {
                    if self.init_fields.is_set(i) {
                        trace!("Field \x1b[1;33m{}\x1b[0m is initialized", field.name);
                    } else {
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
            crate::Innards::Transparent(inner_shape) => {
                let slot = Slot::for_struct_field(
                    self.addr,
                    inner_shape,
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
    pub fn slot_by_name<'s>(&'s mut self, name: &str) -> Result<Slot<'s>, SlotError> {
        match self.shape_desc.get().innards {
            crate::Innards::Struct { fields } => {
                if let Some((index, field)) =
                    fields.iter().enumerate().find(|(_, f)| f.name == name)
                {
                    if let Some(offset) = field.offset {
                        let field_addr = self.addr.map(|addr| unsafe {
                            NonNull::new(addr.as_ptr().byte_offset(offset.get() as isize)).unwrap()
                        });
                        let init_field_slot = InitFieldSlot::Struct {
                            index,
                            set: &mut self.init_fields,
                        };
                        let slot = Slot::for_struct_field(field_addr, field.shape, init_field_slot);
                        Ok(slot)
                    } else {
                        Err(SlotError::NoSuchStaticField)
                    }
                } else {
                    Err(SlotError::NoSuchStaticField)
                }
            }
            crate::Innards::HashMap { value_shape } => {
                // Create a slot for inserting into the HashMap
                let init_field_slot = InitFieldSlot::Ignored;
                let slot = Slot::for_hash_map(
                    self.addr.unwrap(),
                    value_shape,
                    name.to_string(),
                    init_field_slot,
                );
                Ok(slot)
            }
            crate::Innards::Array(_shape) => Err(SlotError::NoStaticFields),
            crate::Innards::Transparent(_shape) => Err(SlotError::NoStaticFields),
            crate::Innards::Scalar(_scalar) => Err(SlotError::NoStaticFields),
        }
    }

    /// Returns a slot for initializing a field in the shape by index.
    pub fn slot_by_index<'s>(&'s mut self, index: usize) -> Result<Slot<'s>, SlotError> {
        match self.shape_desc.get().innards {
            crate::Innards::Struct { fields } => {
                if index < fields.len() {
                    let field = &fields[index];
                    if let Some(offset) = field.offset {
                        let field_addr = self.addr.map(|addr| unsafe {
                            NonNull::new(addr.as_ptr().byte_offset(offset.get() as isize)).unwrap()
                        });
                        let init_field_slot = InitFieldSlot::Struct {
                            index,
                            set: &mut self.init_fields,
                        };
                        let slot = Slot::for_struct_field(field_addr, field.shape, init_field_slot);
                        Ok(slot)
                    } else {
                        Err(SlotError::NoSuchStaticField)
                    }
                } else {
                    Err(SlotError::OutOfBounds)
                }
            }
            crate::Innards::Array(shape) => {
                unimplemented!()
            }
            crate::Innards::Scalar(_) => {
                if index == 0 {
                    let slot = Slot::for_struct_field(
                        self.addr,
                        self.shape_desc,
                        InitFieldSlot::Struct {
                            index: 0,
                            set: &mut self.init_fields,
                        },
                    );
                    Ok(slot)
                } else {
                    Err(SlotError::OutOfBounds)
                }
            }
            crate::Innards::Transparent(inner_shape) => {
                if index == 0 {
                    let slot = Slot::for_struct_field(
                        self.addr,
                        inner_shape,
                        InitFieldSlot::Struct {
                            index: 0,
                            set: &mut self.init_fields,
                        },
                    );
                    Ok(slot)
                } else {
                    Err(SlotError::OutOfBounds)
                }
            }
            crate::Innards::HashMap { .. } => Err(SlotError::NoStaticFields),
        }
    }

    pub fn build_in_place(mut self) {
        self.check_initialization();

        match &mut self.origin {
            Origin::Borrowed {
                init_field_slot, ..
            } => {
                // Mark the borrowed field as initialized
                init_field_slot.mark_as_init();
            }
            Origin::HeapAllocated => {
                panic!("Cannot build in place for heap allocated ShapeUninit");
            }
        }
        std::mem::forget(self);
    }

    fn check_shape_desc_matches<T: Shapely>(&self) {
        if self.shape_desc != T::shape_desc() {
            panic!(
                "This is a partial \x1b[1;34m{}\x1b[0m, you can't build a \x1b[1;32m{}\x1b[0m out of it",
                self.shape_desc.get().name,
                T::shape().name,
            );
        }
    }

    fn deallocate(&mut self) {
        if let Some(addr) = self.addr {
            unsafe { alloc::dealloc(addr.as_ptr(), self.shape_desc.get().layout) }
        }
    }

    pub fn build<T: Shapely>(mut self) -> T {
        self.check_initialization();
        self.check_shape_desc_matches::<T>();

        let result = unsafe {
            let ptr = self.addr.map_or(NonNull::dangling(), |ptr| ptr).as_ptr() as *const T;
            std::ptr::read(ptr)
        };
        trace!(
            "Built \x1b[1;33m{}\x1b[0m successfully",
            std::any::type_name::<T>()
        );
        self.deallocate();
        std::mem::forget(self);
        result
    }

    pub fn build_boxed<T: Shapely>(mut self) -> Box<T> {
        self.check_initialization();
        self.check_shape_desc_matches::<T>();

        let boxed = unsafe {
            let ptr = self.addr.map_or(NonNull::dangling(), |ptr| ptr).as_ptr() as *mut T;
            Box::from_raw(ptr)
        };
        self.deallocate();
        std::mem::forget(self);
        boxed
    }

    pub fn shape_desc(&self) -> ShapeDesc {
        self.shape_desc
    }

    /// Returns the address of the underlying data — for debugging purposes only.
    /// Returns null if the data is zero-sized.
    pub fn addr_for_display(&self) -> *const u8 {
        self.addr.map_or(std::ptr::null(), |ptr| ptr.as_ptr())
    }

    /// # Safety
    ///
    /// The target pointer must be valid and properly aligned,
    /// and must be large enough to hold the value.
    pub unsafe fn move_into(mut self, target: *mut u8) {
        self.check_initialization();
        if let Some(addr) = self.addr {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    addr.as_ptr(),
                    target,
                    self.shape_desc.get().layout.size(),
                );
            }
        }
        self.deallocate();
        std::mem::forget(self);
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
