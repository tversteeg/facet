use crate::{
    Field, FieldError, Innards, OpaqueUninit, Shape, ShapeDesc, Shapely, ValueVTable, trace,
};
use std::ptr::NonNull;

use super::ISet;

/// Allows poking a struct (setting fields, etc.)
pub struct PokeStruct<'mem> {
    pub data: OpaqueUninit<'mem>,
    pub iset: ISet,
    pub shape_desc: ShapeDesc,
    pub shape: Shape,
    pub vtable: ValueVTable,
    pub fields: &'static [Field],
}

impl<'mem> PokeStruct<'mem> {
    /// Creates a new PokeStruct from a MaybeUninit. Panic if it's not a struct.
    pub fn from_maybe_uninit<T: Shapely>(uninit: &'mem mut std::mem::MaybeUninit<T>) -> Self {
        let data = OpaqueUninit::from_maybe_uninit(uninit);
        unsafe { Self::from_opaque_uninit(data, T::shape_desc()) }
    }

    /// # Safety
    ///
    /// The `data` and the `shape_desc` must match
    pub unsafe fn from_opaque_uninit(data: OpaqueUninit<'mem>, shape_desc: ShapeDesc) -> Self {
        let shape = shape_desc.get();
        let vtable = shape.vtable();
        let fields = match &shape.innards {
            Innards::Struct { fields } => *fields,
            _ => panic!("Expected a struct"),
        };

        Self {
            data,
            iset: Default::default(),
            shape_desc,
            shape,
            vtable,
            fields,
        }
    }

    /// Checks if all fields in the struct have been initialized.
    /// Panics if any field is not initialized, providing details about the uninitialized field.
    pub fn assert_all_fields_initialized(&self) {
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
            _ => panic!(
                "Expected struct shape, got something else: {:?}",
                self.shape
            ),
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

    /// Asserts that every field has been initialized and forgets the PokeStruct.
    ///
    /// This method is only used when the origin is borrowed.
    /// If this method is not called, all fields will be freed when the PokeStruct is dropped.
    ///
    /// # Panics
    ///
    /// This function will panic if any field is not initialized.
    pub fn build_in_place(self) {
        // ensure all fields are initialized
        self.assert_all_fields_initialized();

        // prevent field drops when the PokeStruct is dropped
        std::mem::forget(self);
    }

    /// Builds a value of type `T` from the PokeStruct.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - Not all the fields have been initialized.
    /// - The generic type parameter T does not match the shape that this PokeStruct is building.
    pub fn build<T: Shapely>(self) -> T {
        self.assert_all_fields_initialized();
        self.assert_matching_shape::<T>();

        let result = unsafe {
            let ptr = self.data.as_mut_ptr() as *const T;
            std::ptr::read(ptr)
        };
        trace!("Built \x1b[1;33m{}\x1b[0m successfully", T::shape());
        std::mem::forget(self);
        result
    }

    /// Build that PokeStruct into a boxed completed shape.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - Not all the fields have been initialized.
    /// - The generic type parameter T does not match the shape that this PokeStruct is building.
    pub fn build_boxed<T: Shapely>(self) -> Box<T> {
        self.assert_all_fields_initialized();
        self.assert_matching_shape::<T>();

        let boxed = unsafe { Box::from_raw(self.data.as_mut_ptr() as *mut T) };
        std::mem::forget(self);
        boxed
    }

    /// Moves the contents of this `PokeStruct` into a target memory location.
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

    /// Gets a field, by name
    pub fn field_by_name<'s>(&'s mut self, name: &str) -> Result<crate::PokeValue<'s>, FieldError> {
        let index = self
            .fields
            .iter()
            .position(|f| f.name == name)
            .ok_or(FieldError::NoSuchStaticField)?;
        self.field(index)
    }

    /// Get a field writer for a field by index.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The shape doesn't represent a struct.
    /// - The index is out of bounds.
    pub fn field<'s>(&'s mut self, index: usize) -> Result<crate::PokeValue<'s>, FieldError> {
        if index >= self.fields.len() {
            return Err(FieldError::IndexOutOfBounds);
        }

        let field = &self.fields[index];

        // Get the field's address
        let field_addr = unsafe { self.data.field_uninit(field.offset) };
        let field_shape = field.shape;
        let field_vtable = field_shape.get().vtable();

        // Create a PokeValue for this field
        let poke_value = crate::PokeValue {
            data: field_addr,
            shape: field_shape.get(),
            vtable: field_vtable,
        };

        Ok(poke_value)
    }

    /// Sets a field's value by its index.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The index is out of bounds
    /// - The field shapes don't match
    pub fn set(&mut self, index: usize, value: crate::OpaqueConst) -> Result<(), FieldError> {
        if index >= self.fields.len() {
            return Err(FieldError::IndexOutOfBounds);
        }

        let field = &self.fields[index];

        let field_shape = field.shape.get();
        if field_shape != value.shape {
            return Err(FieldError::ShapeMismatch {
                expected: field_shape,
                got: value.shape,
            });
        }

        unsafe {
            std::ptr::copy_nonoverlapping(
                value.data.as_ptr(),
                self.data.field_uninit(field.offset).as_mut_ptr(),
                field_shape.layout.size(),
            );
            self.iset.set(index);
        }

        Ok(())
    }

    /// Sets a field's value by its name.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The field name doesn't exist
    /// - The field shapes don't match
    pub fn set_by_name(&mut self, name: &str, value: crate::OpaqueConst) -> Result<(), FieldError> {
        let index = self
            .fields
            .iter()
            .position(|f| f.name == name)
            .ok_or(FieldError::NoSuchStaticField)?;
        self.set(index, value)
    }

    /// Marks a field as initialized.
    pub unsafe fn mark_initialized(&mut self, index: usize) {
        self.iset.set(index);
    }
}

impl Drop for PokeStruct<'_> {
    fn drop(&mut self) {
        match self.shape.innards {
            crate::Innards::Struct { fields } => fields
                .iter()
                .enumerate()
                .filter_map(|(i, field)| {
                    if self.iset.has(i) {
                        Some((field, field.shape.get().vtable().drop_in_place?))
                    } else {
                        None
                    }
                })
                .for_each(|(field, drop_fn)| unsafe {
                    drop_fn(self.data.field_init(field.offset));
                }),
            _ => {}
        }
    }
}
