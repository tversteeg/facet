use std::ptr::NonNull;

use crate::{InitMark, ShapeDesc, Shapely, trace};

/// Allows writing into a struct field.
pub struct Slot<'s> {
    /// Pointer to where the value will be written. If it's already initialized,
    /// the old value is dropped in place.
    ///
    /// If the shape is a ZST, ptr will be dangling.
    ptr: NonNull<u8>,

    /// Tracks whether the field is initialized
    init_mark: InitMark<'s>,

    /// shape of the field we're writing — used for validation
    shape: ShapeDesc,
}

impl<'s> Slot<'s> {
    /// Create a new slot for writing into a struct field — not just `foo.bar`, but also
    /// `foo.2` for tuples, `foo.0` for newtype wrappers, etc.
    #[inline(always)]
    pub fn for_ptr(ptr: NonNull<u8>, shape: ShapeDesc, init_mark: InitMark<'s>) -> Self {
        Self {
            ptr,
            init_mark,
            shape,
        }
    }

    /// Fills the slot with a value of a concrete type. This performs a type check and panics if the
    /// type is incompatible with the slot's shape.
    ///
    /// If the slot is already initialized, the old value is dropped.
    pub fn fill<T: Shapely>(mut self, value: T) {
        // should we provide fill_unchecked?
        if self.shape != T::shape_desc() {
            panic!(
                "Attempted to fill a field with an incompatible shape.\n\
                Expected shape: \x1b[33m{:?}\x1b[0m\n\
                Actual shape: \x1b[33m{:?}\x1b[0m\n\
                This is undefined behavior and we're refusing to proceed.",
                self.shape.get(),
                T::shape()
            );
        }

        if self.init_mark.get() {
            trace!("Field already initialized, dropping existing value");
            if let Some(drop_fn) = self.shape.get().drop_in_place {
                // Safety: The `drop_fn` is guaranteed to be a valid function pointer
                // for dropping the value at `ptr`. We've already checked that the
                // shape matches, and we're only calling this if the field is initialized.
                // The `ptr` is valid because it points to initialized memory.
                unsafe {
                    drop_fn(self.ptr.as_ptr());
                }
            }
        }

        trace!(
            "Filling struct field at address: \x1b[33m{:?}\x1b[0m with type: \x1b[33m{}\x1b[0m",
            self.ptr,
            T::shape()
        );
        unsafe { std::ptr::write(self.ptr.as_ptr() as *mut T, value) };
        self.init_mark.set();
    }

    pub fn fill_from_partial(mut self, partial: crate::Partial<'_>) {
        if self.shape != partial.shape() {
            panic!(
                "Attempted to fill a field with an incompatible shape.\n\
                Expected shape: {:?}\n\
                Actual shape: {:?}\n\
                This is undefined behavior and we're refusing to proceed.",
                self.shape.get(),
                partial.shape().get()
            );
        }

        unsafe {
            if self.init_mark.get() {
                if let Some(drop_fn) = self.shape.get().drop_in_place {
                    drop_fn(self.ptr.as_ptr());
                }
            }
            partial.move_into(self.ptr);
            self.init_mark.set();
        }
    }

    pub fn shape(&self) -> ShapeDesc {
        self.shape
    }
}
