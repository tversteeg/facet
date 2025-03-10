//! Provides the core traits for shapely

#[cfg(feature = "derive")]
pub use shapely_derive::*;

use std::{collections::HashSet, fmt::Formatter};

mod builtin_impls;

/// Provides reflection so you can shapely about your types.
pub trait Schematic {
    /// Returns the shapely schema
    fn schema() -> Schema;
}

/// Schema for reflection of a type
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

impl Schema {
    const INDENT: usize = 2;

    /// Pretty-print this schema, recursively.
    pub fn pretty_print_recursive(&self, f: &mut Formatter) -> std::fmt::Result {
        self.pretty_print_recursive_internal(f, &mut HashSet::new(), 0)
    }

    fn pretty_print_recursive_internal(
        &self,
        f: &mut Formatter,
        printed_schemas: &mut HashSet<Schema>,
        indent: usize,
    ) -> std::fmt::Result {
        if !printed_schemas.insert(*self) {
            writeln!(
                f,
                "{:indent$}\x1b[1;33m{}\x1b[0m (\x1b[1;31malready printed\x1b[0m)",
                "",
                self.name,
                indent = indent
            )?;
            return Ok(());
        }

        writeln!(
            f,
            "{:indent$}\x1b[1;33m{}\x1b[0m (size: \x1b[1;34m{}\x1b[0m, align: \x1b[1;35m{}\x1b[0m)",
            "",
            self.name,
            self.size,
            self.align,
            indent = indent
        )?;

        match &self.shape {
            Shape::Map(map_shape) => {
                for field in map_shape.fields {
                    writeln!(
                        f,
                        "{:indent$}\x1b[1;32m{}\x1b[0m: ",
                        "",
                        field.name,
                        indent = indent + Self::INDENT
                    )?;
                    (field.schema)().pretty_print_recursive_internal(
                        f,
                        printed_schemas,
                        indent + Self::INDENT * 2,
                    )?;
                }
                if map_shape.open_ended {
                    writeln!(
                        f,
                        "{:indent$}\x1b[1;31m(open-ended)\x1b[0m",
                        "",
                        indent = indent + Self::INDENT * 2
                    )?;
                }
            }
            Shape::Array(elem_schema) => {
                write!(
                    f,
                    "{:indent$}\x1b[1;36mArray of:\x1b[0m ",
                    "",
                    indent = indent + Self::INDENT
                )?;
                elem_schema.pretty_print_recursive_internal(
                    f,
                    printed_schemas,
                    indent + Self::INDENT * 2,
                )?;
            }
            Shape::Transparent(inner_schema) => {
                write!(
                    f,
                    "{:indent$}\x1b[1;36mTransparent wrapper for:\x1b[0m ",
                    "",
                    indent = indent + Self::INDENT
                )?;
                inner_schema.pretty_print_recursive_internal(
                    f,
                    printed_schemas,
                    indent + Self::INDENT * 2,
                )?;
            }
            Shape::Scalar(scalar) => {
                writeln!(
                    f,
                    "{:indent$}\x1b[1;36mScalar:\x1b[0m \x1b[1;33m{:?}\x1b[0m",
                    "",
                    scalar,
                    indent = indent
                )?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print_recursive(f)
    }
}

/// The shape of a schema: is it more map-shaped, array-shaped, scalar-shaped?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub fields: &'static [MapField<'static>],

    /// Will allow setting fields outside of the ones listed in `fields`
    pub open_ended: bool,

    /// Setter for fields — we can't use field offsets
    pub manipulator: &'static dyn MapManipulator,
}

impl PartialEq for MapShape {
    fn eq(&self, other: &Self) -> bool {
        self.fields == other.fields
            && self.open_ended == other.open_ended
            && std::ptr::eq(
                self.manipulator as *const dyn MapManipulator,
                other.manipulator as *const dyn MapManipulator,
            )
    }
}

impl Eq for MapShape {}

impl std::hash::Hash for MapShape {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.fields.hash(state);
        self.open_ended.hash(state);
        (self.manipulator as *const dyn MapManipulator).hash(state);
    }
}

impl std::fmt::Debug for MapShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapShape")
            .field("fields", &self.fields)
            .field("open_ended", &self.open_ended)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapField<'s> {
    /// key for the map field
    pub name: &'s str,

    /// schema of the inner type
    pub schema: fn() -> Schema,
}

/// Given the map's address, calls on_field_addr with the address of the requested field
pub trait MapManipulator: Send + Sync {
    /// Returns the address of a given field. If the map accomodates dynamically-added fields,
    /// this might for example, insert an entry into a HashMap.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `map_addr` is a valid, properly aligned pointer to an instance of the map type.
    /// - `field` corresponds to an existing field in the map's schema.
    /// - Any modifications made via `write_field` maintain the field's type invariants.
    /// - The data pointed to by `map_addr` remains valid for the duration of the `write_field` callback.
    /// - The address provided to `write_field` is not used after the callback returns.
    /// - The callback must fully initialize the field at the provided address.
    unsafe fn set_field_raw(
        &self,
        map_addr: *mut u8,
        field: MapField<'_>,
        write_field: &mut dyn FnMut(*mut u8),
    ) -> SetFieldOutcome;
}

impl dyn MapManipulator {
    /// Still unsafe, but a little less?
    ///
    /// # Safety
    ///
    ///   - map must be a valid pointer to a map of the correct type.
    ///   - slot must be a valid field for the map.
    ///   - value must be of the correct type
    pub unsafe fn set_field<TField>(
        &self,
        map: *mut u8,
        slot: MapField,
        value: TField,
    ) -> SetFieldOutcome {
        let mut value = Some(value);
        unsafe {
            self.set_field_raw(map, slot, &mut |field_ptr| {
                *(field_ptr as *mut TField) = value.take().unwrap();
            })
        }
    }
}

/// Manipulator for struct-like types with known field offsets
pub struct StructManipulator {
    pub fields: &'static [(MapField<'static>, usize)],
}

impl MapManipulator for StructManipulator {
    unsafe fn set_field_raw(
        &self,
        map_addr: *mut u8,
        field: MapField<'_>,
        write_field: &mut dyn FnMut(*mut u8),
    ) -> SetFieldOutcome {
        if let Some((_, offset)) = self.fields.iter().find(|(f, _)| f.name == field.name) {
            unsafe {
                write_field(map_addr.add(*offset));
            }
            SetFieldOutcome::Accepted
        } else {
            SetFieldOutcome::Rejected
        }
    }
}

/// The outcome of trying to set a field on a map
#[derive(Debug, Clone, Copy)]
pub enum SetFieldOutcome {
    /// The field was successfully set
    Accepted,

    /// The field was rejected (unknown field set in a struct, for example)
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    use crate::{Schematic, Shape, StructManipulator};

    #[derive(Debug, PartialEq, Eq)]
    struct FooBar {
        foo: u64,
        bar: String,
    }

    impl Schematic for FooBar {
        fn schema() -> crate::Schema {
            use crate::{MapField, MapShape, Schema, Shape};

            static FOO_FIELD: MapField = MapField {
                name: "foo",
                schema: <u64 as Schematic>::schema,
            };
            static BAR_FIELD: MapField = MapField {
                name: "bar",
                schema: <String as Schematic>::schema,
            };
            static SCHEMA: Schema = Schema {
                name: "FooBar",
                size: std::mem::size_of::<FooBar>(),
                align: std::mem::align_of::<FooBar>(),
                shape: Shape::Map(MapShape {
                    fields: &[FOO_FIELD, BAR_FIELD],
                    open_ended: false,
                    manipulator: &StructManipulator {
                        fields: &[
                            (FOO_FIELD, std::mem::offset_of!(FooBar, foo)),
                            (BAR_FIELD, std::mem::offset_of!(FooBar, bar)),
                        ],
                    },
                }),
                display: None,
                debug: None,
                set_to_default: None,
            };
            SCHEMA
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

        if let Shape::Map(sh) = &schema.shape {
            for field in sh.fields {
                unsafe {
                    match field.name {
                        "foo" => {
                            sh.manipulator.set_field(ptr, *field, 42u64);
                        }
                        "bar" => {
                            sh.manipulator
                                .set_field(ptr, *field, String::from("Hello, World!"));
                        }
                        _ => panic!("Unknown field: {}", field.name),
                    }
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
