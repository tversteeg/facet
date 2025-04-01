use crate::{
    EnumRepr, FieldError, OpaqueUninit, Shape, ShapeDesc, Shapely, ValueVTable, Variant, trace,
};
use std::ptr::NonNull;

use super::{ISet, Poke};

/// Allows poking an enum (selecting variants, setting fields, etc.)
pub struct PokeEnum<'mem> {
    data: OpaqueUninit<'mem>,
    iset: ISet,
    shape_desc: ShapeDesc,
    shape: Shape,
    #[allow(dead_code)]
    vtable: ValueVTable,
    selected_variant: Option<usize>,

    /// all variants for this enum
    variants: &'static [Variant],

    /// representation of the enum
    repr: EnumRepr,
}

impl<'mem> PokeEnum<'mem> {
    /// Creates a new PokeEnum from a MaybeUninit
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
            selected_variant: None,
        }
    }

    /// Creates a new PokeEnum from raw data
    ///
    /// # Safety
    ///
    /// The data buffer must match the size and alignment of the enum shape described by shape_desc
    pub(crate) unsafe fn new(
        data: OpaqueUninit<'mem>,
        shape_desc: ShapeDesc,
        vtable: ValueVTable,
        variants: &'static [Variant],
        repr: EnumRepr,
    ) -> Self {
        let shape = shape_desc.get();
        Self {
            data,
            iset: Default::default(),
            shape_desc,
            shape,
            vtable,
            selected_variant: None,
            variants,
            repr,
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
            selected_variant: None,
        }
    }

    /// Sets the variant of an enum by name.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The shape doesn't represent an enum.
    /// - No variant with the given name exists.
    pub fn set_variant_by_name(&mut self, variant_name: &str) -> Result<(), FieldError> {
        let shape = self.shape;

        if let crate::Def::Enum { variants, repr: _ } = &shape.innards {
            let variant_index = variants
                .iter()
                .enumerate()
                .find(|(_, v)| v.name == variant_name)
                .map(|(i, _)| i)
                .ok_or(FieldError::NoSuchStaticField)?;

            self.set_variant_by_index(variant_index)
        } else {
            Err(FieldError::NotAStruct) // Using NotAStruct as a stand-in for "not an enum"
        }
    }

    /// Sets the variant of an enum by index.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The shape doesn't represent an enum.
    /// - The index is out of bounds.
    pub fn set_variant_by_index(&mut self, variant_index: usize) -> Result<(), FieldError> {
        let shape = self.shape;

        if let crate::Def::Enum { variants, repr } = &shape.innards {
            if variant_index >= variants.len() {
                return Err(FieldError::IndexOutOfBounds);
            }

            // Get the current variant info
            let variant = &variants[variant_index];

            // Prepare memory for the enum
            unsafe {
                // Zero out the memory first to ensure clean state
                std::ptr::write_bytes(self.data.as_mut_ptr(), 0, shape.layout.size());

                // Set up the discriminant (tag)
                // For enums in Rust, the first bytes contain the discriminant
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
            self.selected_variant = Some(variant_index);

            // Reset all field initialization bits (starting from bit 1)
            // ISet can hold 64 bits, so we'll clear bits 1-63
            for i in 1..64 {
                self.iset.unset(i);
            }

            Ok(())
        } else {
            Err(FieldError::NotAStruct) // Using NotAStruct as a stand-in for "not an enum"
        }
    }

    /// Returns the currently selected variant index, if any.
    pub fn selected_variant_index(&self) -> Option<usize> {
        if !self.iset.has(0) {
            return None;
        }

        self.selected_variant
    }

    /// Get a field writer for a field in the currently selected variant.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No variant has been selected yet.
    /// - The field name doesn't exist in the selected variant.
    /// - The selected variant is a unit variant (which has no fields).
    pub fn variant_field_by_name<'s>(
        &'s mut self,
        name: &str,
    ) -> Result<crate::Poke<'s>, FieldError> {
        let variant_index = self
            .selected_variant_index()
            .ok_or(FieldError::NotAStruct)?; // Using NotAStruct as a stand-in for "no variant selected"

        let shape = self.shape;
        if let crate::Def::Enum { variants, repr: _ } = &shape.innards {
            let variant = &variants[variant_index];

            // Find the field in the variant
            match &variant.kind {
                crate::VariantKind::Unit => {
                    // Unit variants have no fields
                    Err(FieldError::NoSuchStaticField)
                }
                crate::VariantKind::Tuple { fields } => {
                    // For tuple variants, find the field by name
                    let (_field_index, field) = fields
                        .iter()
                        .enumerate()
                        .find(|(_, f)| f.name == name)
                        .ok_or(FieldError::NoSuchStaticField)?;

                    // Get the field's address
                    let field_data = unsafe { self.data.field_uninit(field.offset) };
                    let poke = unsafe { Poke::from_opaque_uninit(field_data, field.shape) };
                    Ok(poke)
                }
                crate::VariantKind::Struct { fields } => {
                    // For struct variants, find the field by name
                    let (_field_index, field) = fields
                        .iter()
                        .enumerate()
                        .find(|(_, f)| f.name == name)
                        .ok_or(FieldError::NoSuchStaticField)?;

                    // Get the field's address
                    let field_data = unsafe { self.data.field_uninit(field.offset) };
                    let poke = unsafe { Poke::from_opaque_uninit(field_data, field.shape) };
                    Ok(poke)
                }
            }
        } else {
            Err(FieldError::NotAStruct)
        }
    }

    /// Marks a field in the current variant as initialized.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the field is not already initialized.
    pub unsafe fn mark_field_as_initialized(&mut self, field_index: usize) {
        // Field init bits start at index 1 (index 0 is for variant selection)
        let init_bit = field_index + 1;
        self.iset.set(init_bit);
    }

    /// Checks if all required fields in the enum are initialized.
    /// For enums, this means a variant is selected and all fields in that variant are initialized.
    ///
    /// # Panics
    ///
    /// Panics if no variant is selected or if any field in the selected variant is not initialized.
    pub fn assert_all_fields_initialized(&self) {
        if !self.iset.has(0) {
            panic!(
                "No enum variant was selected. Complete schema:\n{:?}",
                self.shape
            );
        }

        // Get the selected variant
        if let Some(variant_index) = self.selected_variant_index() {
            let shape = self.shape;
            if let crate::Def::Enum { variants, repr: _ } = &shape.innards {
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

    fn assert_matching_shape<T: Shapely>(&self) {
        if self.shape_desc != T::shape_desc() {
            let current_shape = self.shape;
            let target_shape = T::shape();

            panic!(
                "This is a partial \x1b[1;34m{}\x1b[0m, you can't build a \x1b[1;32m{}\x1b[0m out of it",
                current_shape, target_shape,
            );
        }
    }

    /// Asserts that every field in the selected variant has been initialized and forgets the PokeEnum.
    ///
    /// This method is only used when the origin is borrowed.
    /// If this method is not called, all fields will be freed when the PokeEnum is dropped.
    ///
    /// # Panics
    ///
    /// This function will panic if any required field is not initialized.
    pub fn build_in_place(self) {
        // ensure all fields are initialized
        self.assert_all_fields_initialized();

        // prevent field drops when the PokeEnum is dropped
        std::mem::forget(self);
    }

    /// Builds a value of type `T` from the PokeEnum.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - No variant is selected or not all fields in the selected variant have been initialized.
    /// - The generic type parameter T does not match the shape that this PokeEnum is building.
    pub fn build<T: Shapely>(self) -> T {
        self.assert_all_fields_initialized();
        self.assert_matching_shape::<T>();

        let result = unsafe {
            let ptr = self.data.as_ptr() as *const T;
            std::ptr::read(ptr)
        };
        trace!("Built \x1b[1;33m{}\x1b[0m successfully", T::shape());
        std::mem::forget(self);
        result
    }

    /// Build that PokeEnum into a boxed completed shape.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - No variant is selected or not all fields in the selected variant have been initialized.
    /// - The generic type parameter T does not match the shape that this PokeEnum is building.
    pub fn build_boxed<T: Shapely>(self) -> Box<T> {
        self.assert_all_fields_initialized();
        self.assert_matching_shape::<T>();

        let boxed = unsafe { Box::from_raw(self.data.as_mut_ptr() as *mut T) };
        std::mem::forget(self);
        boxed
    }

    /// Moves the contents of this `PokeEnum` into a target memory location.
    ///
    /// # Safety
    ///
    /// The target pointer must be valid and properly aligned,
    /// and must be large enough to hold the value.
    /// The caller is responsible for ensuring that the target memory is properly deallocated
    /// when it's no longer needed.
    pub unsafe fn move_into(self, target: NonNull<u8>) {
        self.assert_all_fields_initialized();
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.data.as_mut_ptr(),
                target.as_ptr(),
                self.shape.layout.size(),
            );
        }
        std::mem::forget(self);
    }
}

impl Drop for PokeEnum<'_> {
    fn drop(&mut self) {
        // If no variant is selected, there's nothing to drop
        if !self.iset.has(0) {
            return;
        }

        if let Some(variant_index) = self.selected_variant_index() {
            let shape = self.shape;
            if let crate::Def::Enum { variants, repr: _ } = &shape.innards {
                let variant = &variants[variant_index];

                // Drop fields based on the variant kind
                match &variant.kind {
                    crate::VariantKind::Unit => {
                        // Unit variants have no fields to drop
                    }
                    crate::VariantKind::Tuple { fields }
                    | crate::VariantKind::Struct { fields } => {
                        // Drop each initialized field
                        for (field_index, field) in fields.iter().enumerate() {
                            let init_bit = field_index + 1;
                            if self.iset.has(init_bit) {
                                if let Some(drop_fn) = field.shape.get().vtable().drop_in_place {
                                    unsafe {
                                        drop_fn(self.data.field_init(field.offset));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// All possible errors when getting a variant by index or by name
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VariantError {
    /// `variant_by_index` was called with an index that is out of bounds.
    IndexOutOfBounds,

    /// `variant_by_name` or `variant_by_index` was called on a non-enum type.
    NotAnEnum,

    /// `variant_by_name` was called with a name that doesn't match any variant.
    NoSuchVariant,
}

impl std::error::Error for VariantError {}

impl std::fmt::Display for VariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariantError::IndexOutOfBounds => write!(f, "Variant index out of bounds"),
            VariantError::NotAnEnum => write!(f, "Not an enum"),
            VariantError::NoSuchVariant => write!(f, "No such variant"),
        }
    }
}
