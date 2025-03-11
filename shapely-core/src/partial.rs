use crate::{trace, FieldError, ShapeDesc, Shapely, Slot};
use std::{alloc, marker::PhantomData, ptr::NonNull};

/// Origin of the partial — did we allocate it? Or is it borrowed?
pub enum Origin<'s> {
    Borrowed {
        parent: Option<&'s Partial<'s>>,
        init_mark: InitMark<'s>,
    },
    HeapAllocated,
}

/// A partially-initialized shape, useful when deserializing for example.
pub struct Partial<'s> {
    /// Where `addr` came from (ie. are we responsible for freeing it?)
    pub(crate) origin: Origin<'s>,

    /// Address of the value we're buildin in memory.
    /// If the type is a ZST, then the addr will be dangling.
    pub(crate) addr: NonNull<u8>,

    /// Keeps track of which fields are initialized
    pub(crate) init_set: InitSet64,

    /// The shape we're building.
    pub(crate) shape: ShapeDesc,
}

/// We can build a tree of partials as the parsing process occurs or deserization occurs, which means they have to be covariant.
fn _assert_partial_covariant<'long: 'short, 'short>(partial: Partial<'long>) -> Partial<'short> {
    partial
}

impl Drop for Partial<'_> {
    fn drop(&mut self) {
        // First drop any initialized fields
        match self.shape.get().innards {
            crate::Innards::Struct { fields } => {
                fields
                    .iter()
                    .enumerate()
                    .filter_map(|(i, field)| {
                        if self.init_set.is_set(i) {
                            Some((field, field.shape.get().drop_in_place?))
                        } else {
                            None
                        }
                    })
                    .for_each(|(field, drop_fn)| {
                        unsafe {
                            // SAFETY: field_addr is valid, aligned, and initialized.
                            //
                            // If the struct is a ZST, then `self.addr` is dangling.
                            // That also means that all the fields are ZSTs, which means
                            // the actual address we pass to the drop fn does not matter,
                            // but we do want the side effects.
                            //
                            // If the struct is not a ZST, then `self.addr` is a valid address.
                            // The fields can still be ZST and that's not a special case, really.
                            drop_fn(self.addr.byte_add(field.offset).as_ptr());
                        }
                    })
            }
            crate::Innards::Scalar(_) => {
                if self.init_set.is_set(0) {
                    // Drop the scalar value if it has a drop function
                    if let Some(drop_fn) = self.shape.get().drop_in_place {
                        // SAFETY: self.addr is always valid for Scalar types,
                        // even for ZSTs where it might be dangling.
                        unsafe {
                            drop_fn(self.addr.as_ptr());
                        }
                    }
                }
            }
            _ => {}
        }

        self.deallocate()
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
            init_set: Default::default(),
            shape: shape_desc,
        }
    }

    /// Borrows a `MaybeUninit<Self>` and returns a `Partial`.
    ///
    /// Before calling assume_init, make sure to call Partial.build_in_place().
    pub fn borrow<T: Shapely>(uninit: &mut std::mem::MaybeUninit<T>) -> Self {
        Self {
            origin: Origin::Borrowed {
                parent: None,
                init_mark: InitMark::Ignored,
            },
            phantom: PhantomData,
            addr: Some(NonNull::new(uninit.as_mut_ptr() as _).unwrap()),
            init_set: Default::default(),
            shape: T::shape_desc(),
        }
    }

    /// Checks if all fields in the struct or scalar value have been initialized.
    /// Panics if any field is not initialized, providing details about the uninitialized field.
    pub(crate) fn check_initialization(&self) {
        trace!(
            "Checking initialization of \x1b[1;33m{}\x1b[0m partial at addr \x1b[1;36m{:p}\x1b[0m",
            self.shape.get().name,
            self.addr_for_display()
        );
        match self.shape.get().innards {
            crate::Innards::Struct { fields } => {
                for (i, field) in fields.iter().enumerate() {
                    if self.init_set.is_set(i) {
                        trace!("Field \x1b[1;33m{}\x1b[0m is initialized", field.name);
                    } else {
                        panic!(
                            "Field '{}' was not initialized. Complete schema:\n{:?}",
                            field.name,
                            self.shape.get()
                        );
                    }
                }
            }
            crate::Innards::Scalar(_) => {
                if !self.init_set.is_set(0) {
                    panic!(
                        "Scalar value was not initialized. Complete schema:\n{:?}",
                        self.shape.get()
                    );
                }
            }
            _ => {}
        }
    }

    /// Returns a slot for assigning this whole shape as a scalar
    pub fn scalar_slot(&mut self) -> Option<Slot<'_>> {
        match self.shape.get().innards {
            crate::Innards::Scalar(_) => {
                let slot = Slot::for_ptr(
                    self.addr,
                    self.shape,
                    InitMark::Struct {
                        index: 0,
                        set: &mut self.init_set,
                    },
                );
                Some(slot)
            }
            crate::Innards::Transparent(inner_shape) => {
                let slot = Slot::for_ptr(
                    self.addr,
                    inner_shape,
                    InitMark::Struct {
                        index: 0,
                        set: &mut self.init_set,
                    },
                );
                Some(slot)
            }
            _ => panic!(
                "Expected scalar innards, found {:?}",
                self.shape.get().innards
            ),
        }
    }

    /// Returns a slot for initializing a field in the shape.
    pub fn slot_by_name<'s>(&'s mut self, name: &str) -> Result<Slot<'s>, FieldError> {
        match self.shape.get().innards {
            crate::Innards::Struct { fields } => {
                if let Some((index, field)) =
                    fields.iter().enumerate().find(|(_, f)| f.name == name)
                {
                    if let Some(offset) = field.offset {
                        let field_addr = self.addr.map(|addr| unsafe {
                            NonNull::new(addr.as_ptr().byte_offset(offset.get() as isize)).unwrap()
                        });
                        let init_field_slot = InitMark::Struct {
                            index,
                            set: &mut self.init_set,
                        };
                        let slot = Slot::for_ptr(field_addr, field.shape, init_field_slot);
                        Ok(slot)
                    } else {
                        Err(FieldError::NoSuchStaticField)
                    }
                } else {
                    Err(FieldError::NoSuchStaticField)
                }
            }
            crate::Innards::HashMap { value_shape } => {
                // Create a slot for inserting into the HashMap
                let init_field_slot = InitMark::Ignored;
                let slot = Slot::for_hash_map(
                    self.addr.unwrap(),
                    value_shape,
                    name.to_string(),
                    init_field_slot,
                );
                Ok(slot)
            }
            crate::Innards::Array(_shape) => Err(FieldError::NoStaticFields),
            crate::Innards::Transparent(_shape) => Err(FieldError::NoStaticFields),
            crate::Innards::Scalar(_scalar) => Err(FieldError::NoStaticFields),
        }
    }

    /// Returns a slot for initializing a field in the shape by index.
    pub fn slot_by_index<'s>(&'s mut self, index: usize) -> Result<Slot<'s>, FieldError> {
        match self.shape.get().innards {
            crate::Innards::Struct { fields } => {
                if index < fields.len() {
                    let field = &fields[index];
                    if let Some(offset) = field.offset {
                        let field_addr = self.addr.map(|addr| unsafe {
                            NonNull::new(addr.as_ptr().byte_offset(offset.get() as isize)).unwrap()
                        });
                        let init_field_slot = InitMark::Struct {
                            index,
                            set: &mut self.init_set,
                        };
                        let slot = Slot::for_ptr(field_addr, field.shape, init_field_slot);
                        Ok(slot)
                    } else {
                        Err(FieldError::NoSuchStaticField)
                    }
                } else {
                    Err(FieldError::IndexOutOfBounds)
                }
            }
            crate::Innards::Array(_) => {
                unimplemented!()
            }
            crate::Innards::Scalar(_) => {
                if index != 0 {
                    return Err(FieldError::IndexOutOfBounds);
                }
                let slot = Slot::for_ptr(self.addr, self.shape, self.init_set.field(0));
                Ok(slot)
            }
            crate::Innards::Transparent(inner_shape) => {
                if index != 0 {
                    return Err(FieldError::IndexOutOfBounds);
                }
                let slot = Slot::for_ptr(self.addr, inner_shape, self.init_set.field(0));
                Ok(slot)
            }
            crate::Innards::HashMap { .. } => Err(FieldError::NoStaticFields),
        }
    }

    pub fn build_in_place(mut self) {
        self.check_initialization();

        match &mut self.origin {
            Origin::Borrowed {
                init_mark: init_field_slot,
                ..
            } => {
                // Mark the borrowed field as initialized
                init_field_slot.set();
            }
            Origin::HeapAllocated => {
                panic!("Cannot build in place for heap allocated ShapeUninit");
            }
        }
        std::mem::forget(self);
    }

    fn check_shape_desc_matches<T: Shapely>(&self) {
        if self.shape != T::shape_desc() {
            panic!(
                "This is a partial \x1b[1;34m{}\x1b[0m, you can't build a \x1b[1;32m{}\x1b[0m out of it",
                self.shape.get().name,
                T::shape().name,
            );
        }
    }

    fn deallocate(&mut self) {
        if let Some(addr) = self.addr {
            unsafe { alloc::dealloc(addr.as_ptr(), self.shape.get().layout) }
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
        self.shape
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
                    self.shape.get().layout.size(),
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
    /// Sets the bit at the given index.
    pub fn set(&mut self, index: usize) {
        if index >= 64 {
            panic!("InitSet64 can only track up to 64 fields. Index {index} is out of bounds.");
        }
        self.0 |= 1 << index;
    }

    /// Unsets the bit at the given index.
    pub fn unset(&mut self, index: usize) {
        if index >= 64 {
            panic!("InitSet64 can only track up to 64 fields. Index {index} is out of bounds.");
        }
        self.0 &= !(1 << index);
    }

    /// Checks if the bit at the given index is set.
    pub fn is_set(&self, index: usize) -> bool {
        if index >= 64 {
            panic!("InitSet64 can only track up to 64 fields. Index {index} is out of bounds.");
        }
        (self.0 & (1 << index)) != 0
    }

    /// Checks if all bits up to the given count are set.
    pub fn all_set(&self, count: usize) -> bool {
        if count > 64 {
            panic!("InitSet64 can only track up to 64 fields. Count {count} is out of bounds.");
        }
        let mask = (1 << count) - 1;
        self.0 & mask == mask
    }

    /// Gets an [InitMark] to track the initialization state of a single field
    pub fn field(&mut self, index: usize) -> InitMark {
        InitMark::Struct { index, set: self }
    }
}

/// `InitMark` is used to track the initialization state of a single field within an `InitSet64`.
/// It is part of a system used to progressively initialize structs, where each field's
/// initialization status is represented by a bit in a 64-bit set.
pub enum InitMark<'s> {
    /// Represents a field in a struct that needs to be tracked for initialization.
    Struct {
        /// The index of the field in the struct (0-63).
        index: usize,
        /// A reference to the `InitSet64` that tracks all fields' initialization states.
        set: &'s mut InitSet64,
    },
    /// Represents a field or value that doesn't need initialization tracking.
    Ignored,
}

impl InitMark<'_> {
    /// Marks the field as initialized by setting its corresponding bit in the `InitSet64`.
    pub fn set(&mut self) {
        if let Self::Struct { index, set } = self {
            set.set(*index);
        }
    }

    /// Marks the field as uninitialized by clearing its corresponding bit in the `InitSet64`.
    pub fn unset(&mut self) {
        if let Self::Struct { index, set } = self {
            set.0 &= !(1 << *index);
        }
    }

    /// Checks if the field is marked as initialized.
    ///
    /// Returns `true` if the field is initialized, `false` otherwise.
    /// Always returns `true` for `Ignored` fields.
    pub fn get(&self) -> bool {
        match self {
            Self::Struct { index, set } => set.is_set(*index),
            Self::Ignored => true,
        }
    }
}
