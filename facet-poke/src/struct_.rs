use facet_trait::{FieldError, Opaque, OpaqueConst, OpaqueUninit, Shape, ShapeExt as _, StructDef};
use std::ptr::NonNull;

use super::{Guard, ISet, PokeValue};

/// Allows poking a struct (setting fields, etc.)
#[derive(Clone)]
pub struct PokeStruct<'mem> {
    data: OpaqueUninit<'mem>,
    shape: &'static Shape,
    def: StructDef,
    iset: ISet,
}

impl<'mem> PokeStruct<'mem> {
    #[inline(always)]
    /// Coerce back into a `PokeValue`
    pub fn into_value(self) -> PokeValue<'mem> {
        unsafe { PokeValue::new(self.data, self.shape) }
    }

    /// Shape getter
    #[inline(always)]
    pub fn shape(&self) -> &'static Shape {
        self.shape
    }
    /// Creates a new PokeStruct
    ///
    /// # Safety
    ///
    /// The `data`, `shape`, and `def` must match
    pub unsafe fn new(data: OpaqueUninit<'mem>, shape: &'static Shape, def: StructDef) -> Self {
        Self {
            data,
            iset: Default::default(),
            shape,
            def,
        }
    }

    /// Checks if all fields in the struct have been initialized.
    /// Panics if any field is not initialized, providing details about the uninitialized field.
    pub fn assert_all_fields_initialized(&self) {
        for (i, field) in self.def.fields.iter().enumerate() {
            if !self.iset.has(i) {
                panic!(
                    "Field '{}' was not initialized. Complete schema:\n{:?}",
                    field.name, self.shape
                );
            }
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
    pub fn build_in_place(self) -> Opaque<'mem> {
        // ensure all fields are initialized
        self.assert_all_fields_initialized();

        let data = unsafe { self.data.assume_init() };

        // prevent field drops when the PokeStruct is dropped
        std::mem::forget(self);

        data
    }

    /// Builds a value of type `T` from the PokeStruct, then deallocates the memory
    /// that this PokeStruct was pointing to.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - Not all the fields have been initialized.
    /// - The generic type parameter T does not match the shape that this PokeStruct is building.
    pub fn build<T: crate::Facet>(self, guard: Option<Guard>) -> T {
        let mut guard = guard;
        let this = self;
        // this changes drop order: guard must be dropped _after_ this.

        this.assert_all_fields_initialized();
        this.shape.assert_type::<T>();
        if let Some(guard) = &guard {
            guard.shape.assert_type::<T>();
        }

        let result = unsafe {
            let ptr = this.data.as_mut_ptr() as *const T;
            std::ptr::read(ptr)
        };
        guard.take(); // dealloc
        std::mem::forget(this);
        result
    }

    /// Build that PokeStruct into a boxed completed shape.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - Not all the fields have been initialized.
    /// - The generic type parameter T does not match the shape that this PokeStruct is building.
    pub fn build_boxed<T: crate::Facet>(self) -> Box<T> {
        self.assert_all_fields_initialized();
        self.shape.assert_type::<T>();

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
    pub unsafe fn move_into(self, target: NonNull<u8>, guard: Option<Guard>) {
        self.assert_all_fields_initialized();
        if let Some(guard) = &guard {
            guard.shape.assert_shape(self.shape);
        }

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
    pub fn field_by_name(&mut self, name: &str) -> Result<(usize, crate::Poke<'mem>), FieldError> {
        let index = self
            .def
            .fields
            .iter()
            .position(|f| f.name == name)
            .ok_or(FieldError::NoSuchStaticField)?;
        Ok((index, self.field(index)?))
    }

    /// Get a field writer for a field by index.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The shape doesn't represent a struct.
    /// - The index is out of bounds.
    pub fn field(&mut self, index: usize) -> Result<crate::Poke<'mem>, FieldError> {
        if index >= self.def.fields.len() {
            return Err(FieldError::IndexOutOfBounds);
        }

        let field = &self.def.fields[index];

        // Get the field's address
        let field_addr = unsafe { self.data.field_uninit(field.offset) };
        let field_shape = field.shape;

        let poke = unsafe { crate::Poke::unchecked_new(field_addr, field_shape) };
        Ok(poke)
    }

    /// Sets a field's value by its index.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The index is out of bounds
    /// - The field shapes don't match
    pub fn set(&mut self, index: usize, value: OpaqueConst) -> Result<(), FieldError> {
        if index >= self.def.fields.len() {
            return Err(FieldError::IndexOutOfBounds);
        }
        let field = &self.def.fields[index];
        let field_shape = field.shape;

        unsafe {
            std::ptr::copy_nonoverlapping(
                value.as_ptr(),
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
    pub fn set_by_name(&mut self, name: &str, value: OpaqueConst) -> Result<(), FieldError> {
        let index = self
            .def
            .fields
            .iter()
            .position(|f| f.name == name)
            .ok_or(FieldError::NoSuchStaticField)?;
        self.set(index, value)
    }

    /// Marks a field as initialized.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the field is initialized. Only call this after writing to
    /// an address gotten through [`Self::field`] or [`Self::field_by_name`].
    pub unsafe fn mark_initialized(&mut self, index: usize) {
        self.iset.set(index);
    }
}

impl Drop for PokeStruct<'_> {
    fn drop(&mut self) {
        self.def
            .fields
            .iter()
            .enumerate()
            .filter_map(|(i, field)| {
                if self.iset.has(i) {
                    Some((field, field.shape.vtable.drop_in_place?))
                } else {
                    None
                }
            })
            .for_each(|(field, drop_fn)| unsafe {
                drop_fn(self.data.field_init(field.offset));
            });
    }
}
