//! Provides the core traits for thonk

mod builtin_impls;

/// Provides reflection so you can thonk about your types.
pub trait Schematic {
    /// Returns the thonk schema
    fn schema() -> &'static Schema;
}

/// Schema for reflection of a type
pub struct Schema {
    /// A descriptive name for the schema, e.g. `u64`, or `Person`
    pub name: &'static str,

    /// Size of one such value, in bytes
    pub size: usize,

    /// Alignment of the value, in bytes
    pub align: usize,

    /// Shape of the value
    pub shape: Shape,

    /// Display impl, if any
    pub display: Option<FmtFunction>,

    /// Debug impl, if any
    pub debug: Option<FmtFunction>,

    /// Set the value at a given address to the default value
    pub set_to_default: Option<fn(*mut u8)>,
}

impl std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schema")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("align", &self.align)
            .field("shape", &self.shape)
            .field("write_display", &self.display.is_some())
            .field("write_debug", &self.debug.is_some())
            .finish()
    }
}

/// The shape of a schema: is it more map-shaped, array-shaped, scalar-shaped?
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    /// Associates keys with values
    Map(MapShape),

    /// Ordered list of heterogenous values, variable size
    Array(&'static Schema),

    // todo: tuples: Ordered list of non-heterogenous values, fixed-size
    /// Transparent — forwards to another known schema
    Transparent(&'static Schema),

    /// Scalar — known based type
    Scalar(Scalar),
}

/// The shape of a map: works for structs, but also HashMap<String, String> for example
#[derive(Clone, Copy)]
pub struct MapShape {
    /// Statically-known fields
    pub fields: &'static [MapField],

    /// Will allow setting fields outside of the ones listed in `fields`
    pub open_ended: bool,

    /// Setter for fields — we can't use field offsets
    pub manipulator: &'static dyn MapManipulator,
}

impl std::fmt::Debug for MapShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapShape")
            .field("fields", &self.fields)
            .field("open_ended", &self.open_ended)
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MapField {
    /// key for the map field
    pub name: &'static str,

    /// schema of the inner type
    pub schema: fn() -> &'static Schema,
}

pub enum MapFieldSlot<'s> {
    // a statically-known field
    Static(&'s MapField),

    // a field we found out while deserializing
    Dynamic {
        name: &'s str,
        schema: &'static Schema,
    },
}

impl<'a> From<&'a MapField> for MapFieldSlot<'a> {
    fn from(field: &'a MapField) -> Self {
        MapFieldSlot::Static(field)
    }
}

impl<'s> MapFieldSlot<'s> {
    fn name(&self) -> &str {
        match self {
            MapFieldSlot::Static(field) => field.name,
            MapFieldSlot::Dynamic { name, .. } => name,
        }
    }
}

/// Given the map's address, calls on_field_addr with the address of the requested field
pub trait MapManipulator: Send + Sync + 'static {
    /// Returns the address of a given field. If the map accomodates dynamically-added fields,
    /// this might for example, insert an entry into a HashMap.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `map_addr` is a valid, properly aligned pointer to an instance of the map type.
    /// - `field` corresponds to an existing field in the map's schema.
    /// - Any modifications made via `on_addr` maintain the field's type invariants.
    /// - The data pointed to by `map_addr` remains valid for the duration of the `on_addr` callback.
    /// - The address provided to `on_addr` is not used after the callback returns.
    unsafe fn set_field_raw(
        &self,
        map_addr: *mut u8,
        field: MapFieldSlot,
        on_addr: &mut dyn FnMut(*mut u8),
    ) -> SetFieldOutcome;
}

/// The outcome of trying to set a field on a map
#[derive(Debug, Clone, Copy)]
pub enum SetFieldOutcome {
    /// The field was successfully set
    Accepted,

    /// The field was rejected (unknown field set in a struct, for example)
    Rejected,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Scalar {
    // Valid utf-8
    String,

    // Not valid utf-8 🤷
    Bytes,

    I8,
    I16,
    I32,
    I64,
    I128,

    U8,
    U16,
    U32,
    U64,
    U128,

    F32,
    F64,

    Boolean,
}

/// A function that writes a field to a formatter
pub type FmtFunction = fn(addr: *const u8, &mut std::fmt::Formatter) -> std::fmt::Result;

#[cfg(test)]
mod tests {
    use crate::{MapFieldSlot, Schematic, SetFieldOutcome, Shape};

    #[derive(Debug, PartialEq, Eq)]
    struct FooBar {
        foo: u64,
        bar: String,
    }

    impl Schematic for FooBar {
        fn schema() -> &'static crate::Schema {
            use crate::{MapField, MapManipulator, MapShape, Schema, Shape};
            struct FooBarManipulator;

            impl MapManipulator for FooBarManipulator {
                unsafe fn set_field_raw(
                    &self,
                    map_addr: *mut u8,
                    field: MapFieldSlot,
                    on_addr: &mut dyn FnMut(*mut u8),
                ) -> SetFieldOutcome {
                    unsafe {
                        let foo_bar = &mut *(map_addr as *mut FooBar);
                        match field.name() {
                            "foo" => on_addr(&mut foo_bar.foo as *mut u64 as _),
                            "bar" => on_addr(&mut foo_bar.bar as *mut String as _),
                            _ => return SetFieldOutcome::Rejected,
                        }
                        SetFieldOutcome::Accepted
                    }
                }
            }

            static SCHEMA: Schema = Schema {
                name: "FooBar",
                size: std::mem::size_of::<FooBar>(),
                align: std::mem::align_of::<FooBar>(),
                shape: Shape::Map(MapShape {
                    fields: &[
                        MapField {
                            name: "foo",
                            schema: <u64 as Schematic>::schema,
                        },
                        MapField {
                            name: "bar",
                            schema: <String as Schematic>::schema,
                        },
                    ],
                    open_ended: false,
                    manipulator: &FooBarManipulator,
                }),
                display: None,
                debug: None,
                set_to_default: None,
            };
            &SCHEMA
        }
    }

    #[test]
    fn build_foobar_through_reflection() {
        let schema = FooBar::schema();
        let layout = std::alloc::Layout::from_size_align(schema.size, schema.align).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        // Ensure proper cleanup
        struct Cleanup<'a>(&'a std::alloc::Layout, *mut u8);
        impl Drop for Cleanup<'_> {
            fn drop(&mut self) {
                unsafe {
                    std::alloc::dealloc(self.1, *self.0);
                }
            }
        }
        let _cleanup = Cleanup(&layout, ptr);

        // Use ptr for further operations...
        if let Shape::Map(map_shape) = &schema.shape {
            for field in map_shape.fields {
                unsafe {
                    map_shape
                        .manipulator
                        .set_field_raw(ptr, field.into(), &mut |field_ptr| match field.name {
                            "foo" => {
                                *(field_ptr as *mut u64) = 42;
                            }
                            "bar" => {
                                let string_ptr = field_ptr as *mut String;
                                std::ptr::write(string_ptr, String::from("Hello, World!"));
                            }
                            _ => panic!("Unknown field: {}", field.name),
                        });
                }
            }
        }

        // Verify the fields were set correctly
        let foo_bar = unsafe { &*(ptr as *const FooBar) };
        assert_eq!(foo_bar.foo, 42);
        assert_eq!(foo_bar.bar, "Hello, World!");

        assert_eq!(
            &FooBar {
                foo: 42,
                bar: "Hello, World!".to_string()
            },
            foo_bar
        )
    }
}
