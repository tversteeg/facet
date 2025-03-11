use std::collections::HashMap;
use std::ptr::NonNull;

use crate::{trace, InitMark, ShapeDesc, Shapely};

/// Where to write the value
enum Destination<'s> {
    /// Writes directly to some address. If it's already initialized,
    /// the old value is dropped in place.
    ///
    /// If the shape is a ZST, ptr will be dangling.
    Ptr {
        ptr: NonNull<u8>,
        init_mark: InitMark<'s>,
    },

    /// Inserts into a HashMap<String, V>
    HashMap { map: NonNull<u8>, key: String },
}

/// Allows writing into a struct field or inserting into a hash map.
pub struct Slot<'s> {
    /// where to write the value
    dest: Destination<'s>,

    /// shape of the field we're writing — used for validation
    shape: ShapeDesc,
}

impl<'s> Slot<'s> {
    /// Create a new slot for writing into a struct field — not just `foo.bar`, but also
    /// `foo.2` for tuples, `foo.0` for newtype wrappers, etc.
    #[inline(always)]
    pub fn for_ptr(ptr: NonNull<u8>, shape: ShapeDesc, init_mark: InitMark<'s>) -> Self {
        Self {
            dest: Destination::Ptr { ptr, init_mark },
            shape,
        }
    }

    /// Create a new slot for writing into a HashMap. This is a different kind of slot because
    /// the field _has_ to be allocated on the heap first and _then_ inserted into the hashmap.
    #[inline(always)]
    pub fn for_hash_map(map: NonNull<u8>, key: String, shape: ShapeDesc) -> Self {
        Self {
            dest: Destination::HashMap { map, key },
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
        trace!(
            "Filling slot with value of type: \x1b[33m{}\x1b[0m",
            std::any::type_name::<T>()
        );

        match self.dest {
            Destination::Ptr { ptr, mut init_mark } => {
                if init_mark.get() {
                    trace!("Field already initialized, dropping existing value");
                    if let Some(drop_fn) = self.shape.get().drop_in_place {
                        // Safety: The `drop_fn` is guaranteed to be a valid function pointer
                        // for dropping the value at `ptr`. We've already checked that the
                        // shape matches, and we're only calling this if the field is initialized.
                        // The `ptr` is valid because it's part of the `Destination::Ptr` variant.
                        unsafe {
                            drop_fn(ptr.as_ptr());
                        }
                    }
                }

                trace!("Filling struct field at address: \x1b[33m{:?}\x1b[0m", ptr);
                unsafe { std::ptr::write(ptr.as_ptr() as *mut T, value) };
                init_mark.set();
            }
            Destination::HashMap { map, key } => {
                let map = unsafe { &mut *(map.as_ptr() as *mut HashMap<String, T>) };
                trace!("Inserting value into HashMap with key: \x1b[33m{key}\x1b[0m");
                map.insert(key, value);
            }
        }
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
            match self.dest {
                Destination::Ptr { ptr: field_addr } => {
                    let size = self.shape.get().layout.size();
                    if self.init_field_slot.is_init() {
                        if let Some(drop_fn) = self.shape.get().drop_in_place {
                            drop_fn(
                                field_addr
                                    .map(|p| p.as_ptr())
                                    .unwrap_or(std::ptr::null_mut()),
                            );
                        }
                    }
                    if let Some(field_addr) = field_addr {
                        let field_addr = field_addr.as_ptr();
                        trace!(
                            "Filling struct field: src=\x1b[33m{:?}\x1b[0m, dst=\x1b[33m{:?}\x1b[0m, size=\x1b[33m{}\x1b[0m bytes",
                            partial.addr.unwrap().as_ptr(),
                            field_addr,
                            size
                        );
                        partial.move_into(field_addr);
                    } else {
                        trace!("Skipping write for ZST field");
                        drop(partial)
                    }
                }
                Destination::HashMap { map: _, ref key } => {
                    trace!(
                        "Filling HashMap entry: key=\x1b[33m{}\x1b[0m, src=\x1b[33m{:?}\x1b[0m, size=\x1b[33m{}\x1b[0m bytes",
                        key,
                        partial.addr.unwrap().as_ptr(),
                        self.shape.get().layout.size()
                    );
                    // TODO: Implement for HashMap
                    // I guess we need another field in the vtable?
                    panic!("fill_from_partial not implemented for HashMap");
                }
            }
        }
        self.init_field_slot.mark_as_init();
    }
}
