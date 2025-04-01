use crate::{
    FieldError, Innards, ListVTable, MapIterVTable, MapVTable, Opaque, OpaqueConst, OpaqueUninit,
    Shape, ShapeDesc, Shapely, ValueVTable, trace,
};
use std::{marker::PhantomData, ptr::NonNull};

/// A partially-initialized struct, tuple struct, tuple, or enum.
///
/// This type keeps track of the initialized state of every field and only allows getting out the
/// concrete type or the boxed concrete type or moving out of this partial into a pointer if all the
/// fields have been initialized.
pub struct Partial<'s> {
    /// Address of the value we're building in memory.
    /// If the type is a ZST, then the addr will be dangling.
    pub(crate) data: OpaqueUninit<'s>,

    /// Keeps track of which fields are initialized
    pub(crate) iset: ISet,

    /// The shape desc we're building
    pub(crate) shape_desc: ShapeDesc,

    /// The shape we're building (cache `shape_desc.get()`)
    pub(crate) shape: Shape,

    /// The value vtable of the shape we're building.
    pub(crate) vtable: ValueVTable,
}

impl<'mem> Partial<'mem> {
    /// Borrows a `MaybeUninit<Self>` and returns a `Partial`.
    ///
    /// Before calling assume_init, make sure to call Partial.build_in_place().
    pub fn from_maybe_uninit<T: Shapely>(uninit: &'mem mut std::mem::MaybeUninit<T>) -> Self {
        let shape_desc = T::shape_desc();
        let shape = shape_desc.get();
        let vtable = shape.vtable();

        Self {
            data: OpaqueUninit::from_maybe_uninit(uninit),
            iset: Default::default(),
            shape_desc,
            shape,
            vtable,
        }
    }

    /// # Safety
    ///
    /// The `data` and the `shape_desc` must match
    pub unsafe fn from_opaque_uninit(data: OpaqueUninit<'mem>, shape_desc: ShapeDesc) -> Self {
        let shape = shape_desc.get();
        let vtable = shape.vtable();

        Self {
            data,
            iset: Default::default(),
            shape_desc,
            shape,
            vtable,
        }
    }

    /// Checks if all fields in the struct or scalar value have been initialized.
    /// Panics if any field is not initialized, providing details about the uninitialized field.
    pub(crate) fn assert_all_fields_initialized(&self) {
        match self.shape.innards {
            crate::Innards::Struct { fields } => {
                for (i, field) in fields.iter().enumerate() {
                    if !self.iset.has(i) {
                        panic!(
                            "Field '{}' was not initialized. Complete schema:\n{:?}",
                            field.name, self.shape
                        );
                    }
                }
            }
            crate::Innards::Enum {
                variants: _,
                repr: _,
            } => {
                // Check if a variant has been selected (bit 0)
                if !self.iset.has(0) {
                    panic!(
                        "No enum variant was selected. Complete schema:\n{:?}",
                        self.shape
                    );
                }

                // Get the selected variant
                if let Some(variant_index) = self.selected_variant_index() {
                    let shape = self.shape;
                    if let crate::Innards::Enum { variants, repr: _ } = &shape.innards {
                        let variant = &variants[variant_index];

                        // Check if all fields of the selected variant are initialized
                        match &variant.kind {
                            crate::VariantKind::Unit => {
                                // Unit variants don't have fields, so they're initialized if the variant is selected
                            }
                            crate::VariantKind::Tuple { fields }
                            | crate::VariantKind::Struct { fields } => {
                                // Check each field
                                for (field_index, field) in fields.iter().enumerate() {
                                    // Field init bits start at index 1 (index 0 is for variant selection)
                                    let init_bit = field_index + 1;
                                    if !self.iset.has(init_bit) {
                                        panic!(
                                            "Field '{}' of variant '{}' was not initialized. Complete schema:\n{:?}",
                                            field.name, variant.name, self.shape
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn assert_matching_shape<T: Shapely>(&self) {
        if self.shape_desc != T::shape_desc() {
            let partial_shape = self.shape;
            let target_shape = T::shape();

            panic!(
                "This is a partial \x1b[1;34m{}\x1b[0m, you can't build a \x1b[1;32m{}\x1b[0m out of it",
                partial_shape, target_shape,
            );
        }
    }

    /// Asserts that every field has been initialized and forgets the Partial.
    ///
    /// This method is only used when the origin is borrowed.
    /// If this method is not called, all fields will be freed when the Partial is dropped.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - The origin is not borrowed (i.e., it's heap allocated).
    /// - Any field is not initialized.
    pub fn build_in_place(self) {
        // ensure all fields are initialized
        self.assert_all_fields_initialized();

        // prevent field drops when the Partial is dropped
        std::mem::forget(self);
    }

    /// Builds a value of type `T` from the partial representation.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - Not all the fields have been initialized.
    /// - The generic type parameter T does not match the shape that this partial is building.
    pub fn build<T: Shapely>(mut self) -> T {
        self.assert_all_fields_initialized();
        self.assert_matching_shape::<T>();

        let shape = self.shape;

        // Special handling for enums to ensure the correct variant is built
        if let crate::Innards::Enum { variants, repr } = &shape.innards {
            if !self.iset.has(0) {
                panic!("Enum variant not selected");
            }

            // Check if explicit enum representation is used
            if let crate::EnumRepr::Default = repr {
                panic!(
                    "Enum must have an explicit representation (e.g. #[repr(u8)]). Default representation is not supported."
                );
            }

            if let Some(variant_idx) = self.selected_variant_index() {
                // Create a properly initialized result with the correct variant
                let mut result_mem = std::mem::MaybeUninit::<T>::uninit();

                unsafe {
                    // Zero out memory first for safety
                    std::ptr::write_bytes(
                        result_mem.as_mut_ptr() as *mut u8,
                        0,
                        std::mem::size_of::<T>(),
                    );

                    // Get the variant info
                    let variant = &variants[variant_idx];

                    // Set discriminant value - this is the key part for fixing the enum issue
                    let discriminant_value = match &variant.discriminant {
                        Some(disc) => *disc,
                        None => variant_idx as i64,
                    };

                    // Write the discriminant value based on the representation
                    match repr {
                        crate::EnumRepr::U8 => {
                            let tag_ptr = result_mem.as_mut_ptr() as *mut u8;
                            *tag_ptr = discriminant_value as u8;
                        }
                        crate::EnumRepr::U16 => {
                            let tag_ptr = result_mem.as_mut_ptr() as *mut u16;
                            *tag_ptr = discriminant_value as u16;
                        }
                        crate::EnumRepr::U32 => {
                            let tag_ptr = result_mem.as_mut_ptr() as *mut u32;
                            *tag_ptr = discriminant_value as u32;
                        }
                        crate::EnumRepr::U64 => {
                            let tag_ptr = result_mem.as_mut_ptr() as *mut u64;
                            *tag_ptr = discriminant_value as u64;
                        }
                        crate::EnumRepr::USize => {
                            let tag_ptr = result_mem.as_mut_ptr() as *mut usize;
                            *tag_ptr = discriminant_value as usize;
                        }
                        crate::EnumRepr::I8 => {
                            let tag_ptr = result_mem.as_mut_ptr() as *mut i8;
                            *tag_ptr = discriminant_value as i8;
                        }
                        crate::EnumRepr::I16 => {
                            let tag_ptr = result_mem.as_mut_ptr() as *mut i16;
                            *tag_ptr = discriminant_value as i16;
                        }
                        crate::EnumRepr::I32 => {
                            let tag_ptr = result_mem.as_mut_ptr() as *mut i32;
                            *tag_ptr = discriminant_value as i32;
                        }
                        crate::EnumRepr::I64 => {
                            let tag_ptr = result_mem.as_mut_ptr() as *mut i64;
                            *tag_ptr = discriminant_value;
                        }
                        crate::EnumRepr::ISize => {
                            let tag_ptr = result_mem.as_mut_ptr() as *mut isize;
                            *tag_ptr = discriminant_value as isize;
                        }
                        crate::EnumRepr::Default => {
                            // Use a heuristic based on the number of variants
                            if variants.len() <= 256 {
                                // Can fit in a u8
                                let tag_ptr = result_mem.as_mut_ptr() as *mut u8;
                                *tag_ptr = discriminant_value as u8;
                            } else if variants.len() <= 65536 {
                                // Can fit in a u16
                                let tag_ptr = result_mem.as_mut_ptr() as *mut u16;
                                *tag_ptr = discriminant_value as u16;
                            } else {
                                // Default to u32
                                let tag_ptr = result_mem.as_mut_ptr() as *mut u32;
                                *tag_ptr = discriminant_value as u32;
                            }
                        }
                    }

                    // For non-unit variants, copy the initialized fields
                    match &variant.kind {
                        crate::VariantKind::Tuple { fields } => {
                            // Copy the fields from our partial to the result
                            for field in fields.iter() {
                                let src_ptr =
                                    (self.data.as_mut_ptr() as *const u8).add(field.offset);
                                let dst_ptr =
                                    (result_mem.as_mut_ptr() as *mut u8).add(field.offset);
                                // Access the layout from the shape field
                                let size = field.shape.get().layout.size();
                                std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, size);
                            }
                        }
                        crate::VariantKind::Struct { fields } => {
                            // Copy the fields from our partial to the result
                            for field in fields.iter() {
                                let src_ptr =
                                    (self.data.as_mut_ptr() as *const u8).add(field.offset);
                                let dst_ptr =
                                    (result_mem.as_mut_ptr() as *mut u8).add(field.offset);
                                // Access the layout from the shape field
                                let size = field.shape.get().layout.size();
                                std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, size);
                            }
                        }
                        crate::VariantKind::Unit => {
                            // Nothing to copy for unit variants, just the discriminant is enough
                        }
                    }

                    // Return the completed enum
                    let result = result_mem.assume_init();
                    trace!("Built \x1b[1;33m{}\x1b[0m successfully", T::shape());
                    std::mem::forget(self);
                    return result;
                }
            }
        }

        // For non-enum types, use the original implementation
        let result = unsafe {
            let ptr = self.data.as_mut_ptr() as *const T;
            std::ptr::read(ptr)
        };
        trace!("Built \x1b[1;33m{}\x1b[0m successfully", T::shape());
        std::mem::forget(self);
        result
    }

    /// Build that partial into a boxed completed shape.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - Not all the fields have been initialized.
    /// - The generic type parameter T does not match the shape that this partial is building.
    ///
    /// # Safety
    ///
    /// This function uses unsafe code to create a Box from a raw pointer.
    /// It's safe because we've verified the initialization and shape matching,
    /// and we forget `self` to prevent double-freeing.
    pub fn build_boxed<T: Shapely>(self) -> Box<T> {
        self.assert_all_fields_initialized();
        self.assert_matching_shape::<T>();

        let boxed = unsafe { Box::from_raw(self.data.as_mut_ptr() as *mut T) };
        std::mem::forget(self);
        boxed
    }

    /// Moves the contents of this `Partial` into a target memory location.
    ///
    /// This function is useful when you need to place the fully initialized value
    /// into a specific memory address, such as when working with FFI or custom allocators.
    ///
    /// # Safety
    ///
    /// The target pointer must be valid and properly aligned,
    /// and must be large enough to hold the value.
    /// The caller is responsible for ensuring that the target memory is properly deallocated
    /// when it's no longer needed.
    pub unsafe fn move_into(mut self, target: NonNull<u8>) {
        self.assert_all_fields_initialized();
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.data.as_mut_ptr(),
                target.as_ptr(),
                // note: copy_nonoverlapping takes a count,
                // since we're dealing with `*mut u8`, it's a byte count.
                // if we were dealing with `*mut ()`, we'd have a nasty surprise.
                self.shape.layout.size(),
            );
        }
        std::mem::forget(self);
    }

    /// Returns the shape we're currently building.
    pub fn shape(&self) -> ShapeDesc {
        self.shape
    }

    /// Returns the address of the value we're building in memory.
    pub fn addr(&self) -> NonNull<u8> {
        self.data
    }

    /// Sets the variant of an enum by name.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The shape doesn't represent an enum.
    /// - No variant with the given name exists.
    pub fn set_variant_by_name(&mut self, variant_name: &str) -> Result<(), crate::FieldError> {
        let shape = self.shape.get();

        if let crate::Innards::Enum { variants, repr: _ } = &shape.innards {
            let variant_index = variants
                .iter()
                .enumerate()
                .find(|(_, v)| v.name == variant_name)
                .map(|(i, _)| i)
                .ok_or(crate::FieldError::NoSuchStaticField)?;

            self.set_variant_by_index(variant_index)
        } else {
            Err(crate::FieldError::NotAStruct) // Using NotAStruct as a stand-in for "not an enum"
        }
    }

    /// Sets the variant of an enum by index.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The shape doesn't represent an enum.
    /// - The index is out of bounds.
    pub fn set_variant_by_index(&mut self, variant_index: usize) -> Result<(), crate::FieldError> {
        let shape = self.shape.get();

        if let crate::Innards::Enum { variants, repr } = &shape.innards {
            if variant_index >= variants.len() {
                return Err(crate::FieldError::IndexOutOfBounds);
            }

            // Get the current variant info
            let variant = &variants[variant_index];

            // Prepare memory for the enum
            unsafe {
                // Zero out the memory first to ensure clean state
                std::ptr::write_bytes(self.data.as_mut_ptr(), 0, shape.layout.size());

                // Set up the discriminant (tag)
                // For enums in Rust, the first bytes contain the discriminant
                // By default, we should use the smallest type that can represent all variants
                let discriminant_value = match &variant.discriminant {
                    // If we have an explicit discriminant, use it
                    Some(discriminant) => *discriminant,
                    // Otherwise, use the variant index directly
                    None => variant_index as i64,
                };

                // Write the discriminant value based on the representation
                match repr {
                    crate::EnumRepr::U8 => {
                        let tag_ptr = self.data.as_mut_ptr();
                        *tag_ptr = discriminant_value as u8;
                    }
                    crate::EnumRepr::U16 => {
                        let tag_ptr = self.data.as_mut_ptr() as *mut u16;
                        *tag_ptr = discriminant_value as u16;
                    }
                    crate::EnumRepr::U32 => {
                        let tag_ptr = self.data.as_mut_ptr() as *mut u32;
                        *tag_ptr = discriminant_value as u32;
                    }
                    crate::EnumRepr::U64 => {
                        let tag_ptr = self.data.as_mut_ptr() as *mut u64;
                        *tag_ptr = discriminant_value as u64;
                    }
                    crate::EnumRepr::USize => {
                        let tag_ptr = self.data.as_mut_ptr() as *mut usize;
                        *tag_ptr = discriminant_value as usize;
                    }
                    crate::EnumRepr::I8 => {
                        let tag_ptr = self.data.as_mut_ptr() as *mut i8;
                        *tag_ptr = discriminant_value as i8;
                    }
                    crate::EnumRepr::I16 => {
                        let tag_ptr = self.data.as_mut_ptr() as *mut i16;
                        *tag_ptr = discriminant_value as i16;
                    }
                    crate::EnumRepr::I32 => {
                        let tag_ptr = self.data.as_mut_ptr() as *mut i32;
                        *tag_ptr = discriminant_value as i32;
                    }
                    crate::EnumRepr::I64 => {
                        let tag_ptr = self.data.as_mut_ptr() as *mut i64;
                        *tag_ptr = discriminant_value;
                    }
                    crate::EnumRepr::ISize => {
                        let tag_ptr = self.data.as_mut_ptr() as *mut isize;
                        *tag_ptr = discriminant_value as isize;
                    }
                    crate::EnumRepr::Default => {
                        // Use a heuristic based on the number of variants
                        if variants.len() <= 256 {
                            // Can fit in a u8
                            let tag_ptr = self.data.as_mut_ptr();
                            *tag_ptr = discriminant_value as u8;
                        } else if variants.len() <= 65536 {
                            // Can fit in a u16
                            let tag_ptr = self.data.as_mut_ptr() as *mut u16;
                            *tag_ptr = discriminant_value as u16;
                        } else {
                            // Default to u32
                            let tag_ptr = self.data.as_mut_ptr() as *mut u32;
                            *tag_ptr = discriminant_value as u32;
                        }
                    }
                }
            }

            // Mark the variant as selected (bit 0)
            self.iset.set(0);

            // Reset all field initialization bits (starting from bit 1)
            // InitSet64 can hold 64 bits, so we'll clear bits 1-63
            for i in 1..64 {
                self.iset.unset(i);
            }

            Ok(())
        } else {
            Err(crate::FieldError::NotAStruct) // Using NotAStruct as a stand-in for "not an enum"
        }
    }

    /// Returns the currently selected variant index, if any.
    pub fn selected_variant_index(&self) -> Option<usize> {
        if !self.iset.has(0) {
            return None;
        }

        let shape = self.shape.get();

        // We need to read the discriminant and map it back to the variant index
        if let crate::Innards::Enum { variants, repr } = &shape.innards {
            unsafe {
                // Attempt to read the tag based on the representation
                let discriminant_value = match repr {
                    crate::EnumRepr::U8 => {
                        let tag_ptr = self.data.as_mut_ptr() as *const u8;
                        *tag_ptr as i64
                    }
                    crate::EnumRepr::U16 => {
                        let tag_ptr = self.data.as_mut_ptr() as *const u16;
                        *tag_ptr as i64
                    }
                    crate::EnumRepr::U32 => {
                        let tag_ptr = self.data.as_mut_ptr() as *const u32;
                        *tag_ptr as i64
                    }
                    crate::EnumRepr::U64 => {
                        let tag_ptr = self.data.as_mut_ptr() as *const u64;
                        *tag_ptr as i64
                    }
                    crate::EnumRepr::USize => {
                        let tag_ptr = self.data.as_mut_ptr() as *const usize;
                        *tag_ptr as i64
                    }
                    crate::EnumRepr::I8 => {
                        let tag_ptr = self.data.as_mut_ptr() as *const i8;
                        *tag_ptr as i64
                    }
                    crate::EnumRepr::I16 => {
                        let tag_ptr = self.data.as_mut_ptr() as *const i16;
                        *tag_ptr as i64
                    }
                    crate::EnumRepr::I32 => {
                        let tag_ptr = self.data.as_mut_ptr() as *const i32;
                        *tag_ptr as i64
                    }
                    crate::EnumRepr::I64 => {
                        let tag_ptr = self.data.as_mut_ptr() as *const i64;
                        *tag_ptr
                    }
                    crate::EnumRepr::ISize => {
                        let tag_ptr = self.data.as_mut_ptr() as *const isize;
                        *tag_ptr as i64
                    }
                    crate::EnumRepr::Default => {
                        // Use a heuristic based on the number of variants
                        if variants.len() <= 256 {
                            // Likely a u8 discriminant
                            let tag_ptr = self.data.as_mut_ptr() as *const u8;
                            *tag_ptr as i64
                        } else if variants.len() <= 65536 {
                            // Likely a u16 discriminant
                            let tag_ptr = self.data.as_mut_ptr() as *const u16;
                            *tag_ptr as i64
                        } else {
                            // Default to u32
                            let tag_ptr = self.data.as_mut_ptr() as *const u32;
                            *tag_ptr as i64
                        }
                    }
                };

                // Find the variant with this discriminant or index
                // Try matching by discriminant first
                for (idx, variant) in variants.iter().enumerate() {
                    if let Some(disc) = variant.discriminant {
                        if disc == discriminant_value {
                            return Some(idx);
                        }
                    } else if idx as i64 == discriminant_value {
                        // Fallback to index-based match
                        return Some(idx);
                    }
                }

                // If we couldn't find a match, but we know a variant is selected,
                // assume it's the variant at the discriminant index if in bounds
                if (discriminant_value as usize) < variants.len() {
                    return Some(discriminant_value as usize);
                }
            }
        }

        None
    }

    /// Get a slot for a field in the currently selected variant.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The shape doesn't represent an enum.
    /// - No variant has been selected yet.
    /// - The field name doesn't exist in the selected variant.
    /// - The selected variant is a unit variant (which has no fields).
    pub fn variant_field_by_name<'s>(
        &'s mut self,
        name: &str,
    ) -> Result<FieldWriter<'s>, crate::FieldError> {
        let variant_index = self
            .selected_variant_index()
            .ok_or(crate::FieldError::NotAStruct)?; // Using NotAStruct as a stand-in for "no variant selected"

        let shape = self.shape;
        if let crate::Innards::Enum { variants, repr: _ } = &shape.innards {
            let variant = &variants[variant_index];

            // Find the field in the variant
            match &variant.kind {
                crate::VariantKind::Unit => {
                    // Unit variants have no fields
                    Err(crate::FieldError::NoSuchStaticField)
                }
                crate::VariantKind::Tuple { fields } => {
                    // For tuple variants, find the field by name
                    let (field_index, field) = fields
                        .iter()
                        .enumerate()
                        .find(|(_, f)| f.name == name)
                        .ok_or(crate::FieldError::NoSuchStaticField)?;

                    // The field's initialization bit is offset by 1 (since bit 0 is used for variant selection)
                    let init_bit = field_index + 1;

                    // Get the field's address
                    let field_addr = unsafe {
                        // The actual offset may depend on the variant's layout, but we use the field index for now
                        // This is technically incorrect, as it assumes a simple layout where offsets are contiguous
                        self.data.as_mut_ptr().add(field.offset)
                    };

                    Ok(FieldWriter::for_ptr(
                        field_addr,
                        field.shape,
                        self.iset.field(init_bit),
                    ))
                }
                crate::VariantKind::Struct { fields } => {
                    // For struct variants, find the field by name
                    let (field_index, field) = fields
                        .iter()
                        .enumerate()
                        .find(|(_, f)| f.name == name)
                        .ok_or(crate::FieldError::NoSuchStaticField)?;

                    // The field's initialization bit is offset by 1 (since bit 0 is used for variant selection)
                    let init_bit = field_index + 1;

                    // Get the field's address
                    let field_addr = unsafe { self.data.as_mut_ptr().add(field.offset) };

                    Ok(FieldWriter::for_ptr(
                        field_addr,
                        field.shape,
                        self.iset.field(init_bit),
                    ))
                }
            }
        } else {
            Err(crate::FieldError::NotAStruct)
        }
    }
}

impl Drop for Partial<'_> {
    // This drop function is only really called when a partial is dropped without being fully
    // built out. Otherwise, it's forgotten because the value has been moved elsewhere.
    //
    // As a result, its only job is to drop any fields that may have been initialized.
    fn drop(&mut self) {
        match self.shape.innards {
            crate::Innards::Struct { fields } => {
                fields
                    .iter()
                    .enumerate()
                    .filter_map(|(i, field)| {
                        if self.iset.has(i) {
                            Some((field, field.shape.get().vtable().drop_in_place?))
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
                            drop_fn(self.data.field_init(field.offset));
                        }
                    })
            }
            // TODO: drop partially initialized tuples, etc.
            _ => {}
        }
    }
}

/// Keeps track of which fields were initialized, up to 64 fields
#[derive(Clone, Copy, Default)]
pub struct ISet(u64);

impl ISet {
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
    pub fn has(&self, index: usize) -> bool {
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
}

/// A helper struct to fill up lists — note that it is designed for `Vec<T>`
/// rather than fixed-size arrays or slices, so it's a bit of a misnomer at the moment.
pub struct ListWriter<'mem> {
    pub(crate) data: Opaque<'mem>,
    pub(crate) vtable: ListVTable,
}

impl<'mem> ListWriter<'mem> {
    /// Create a new ArraySlot with the given address and vtable
    pub(crate) unsafe fn new(addr: Opaque<'mem>, vtable: ListVTable) -> Self {
        Self { data: addr, vtable }
    }

    /// Push a partial value onto the array
    ///
    /// # Safety
    ///
    /// This function uses unsafe code to push a value into the array.
    /// It's safe to use because the vtable's push function handles
    /// proper memory management and initialization.
    pub fn push(&mut self, item: Opaque) {
        // Call the vtable's push function to add the item to the array
        unsafe {
            (self.vtable.push)(self.data, item);
        }
    }
}

/// Provides insert, length check, and iteration over a type-erased map
pub struct MapWriter<'mem> {
    pub(crate) data: Opaque<'mem>,
    pub(crate) vtable: MapVTable,
}

impl<'mem> MapWriter<'mem> {
    /// Create a new MapSlot with the given address and vtable
    pub(crate) unsafe fn new(data: Opaque<'mem>, vtable: MapVTable) -> Self {
        Self { data, vtable }
    }

    /// Insert a key-value pair into the Map
    ///
    /// # Safety
    ///
    /// This function uses unsafe code to insert a key-value pair into the Map.
    /// It's safe to use because the vtable's insert function handles
    /// proper memory management and initialization.
    pub fn insert(&mut self, key: Opaque, value: Opaque) {
        // Call the vtable's insert function to add the key-value pair to the Map
        unsafe {
            (self.vtable.insert)(self.data, key, value);
        }
    }

    /// Get the number of entries in the Map
    pub fn len(&self) -> usize {
        unsafe { (self.vtable.len)(self.data.as_const()) }
    }

    /// Check if the Map is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if the Map contains a key
    pub fn contains_key<'key>(&self, key: OpaqueConst<'key>) -> bool {
        unsafe { (self.vtable.contains_key)(self.data.as_const(), key) }
    }
}

/// An iterator over key-value pairs in a Map
pub struct MapIter<'mem> {
    iter: Opaque<'mem>,
    vtable: MapIterVTable,
}

impl<'mem> MapIter<'mem> {
    /// Get the next key-value pair from the iterator
    pub fn next(&mut self) -> Option<(OpaqueConst<'mem>, OpaqueConst<'mem>)> {
        unsafe { (self.vtable.next)(self.iter) }
    }
}

impl<'mem> Drop for MapIter<'mem> {
    fn drop(&mut self) {
        unsafe {
            (self.vtable.dealloc)(self.iter);
        }
    }
}
