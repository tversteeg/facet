use std::{collections::HashSet, fmt::Formatter};

use crate::{FieldSlot, ShapeUninit};

/// Schema for reflection of a type
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shape {
    /// A descriptive name for the type, e.g. `u64`, or `Person`
    pub name: &'static str,

    /// Size of one such value, in bytes
    pub size: usize,

    /// Alignment of the value, in bytes
    pub align: usize,

    /// Details/contents of the value
    pub shape: ShapeKind,

    /// Display impl, if any
    pub display: Option<FmtFunction>,

    /// Debug impl, if any
    pub debug: Option<FmtFunction>,

    /// Set the value at a given address to the default value for this type
    pub set_to_default: Option<fn(*mut u8)>,
}

impl Shape {
    const INDENT: usize = 2;

    /// Pretty-print this shape, recursively.
    pub fn pretty_print_recursive(&self, f: &mut Formatter) -> std::fmt::Result {
        self.pretty_print_recursive_internal(f, &mut HashSet::new(), 0)
    }

    fn pretty_print_recursive_internal(
        &self,
        f: &mut Formatter,
        printed_schemas: &mut HashSet<Shape>,
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
            ShapeKind::Map(map_shape) => {
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
            ShapeKind::Array(elem_schema) => {
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
            ShapeKind::Transparent(inner_schema) => {
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
            ShapeKind::Scalar(scalar) => {
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

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print_recursive(f)
    }
}

/// The shape of a schema: is it more map-shaped, array-shaped, scalar-shaped?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeKind {
    /// Associates keys with values
    Map(MapShape),

    /// Ordered list of heterogenous values, variable size
    Array(&'static Shape),

    /// Transparent — forwards to another known schema
    Transparent(&'static Shape),

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
    pub schema: fn() -> Shape,
}

/// Given the map's address, returns a FieldSlot for the requested field
pub trait MapManipulator: Send + Sync {
    /// Returns a FieldSlot for a given field. If the map accommodates dynamically-added fields,
    /// this might, for example, insert an entry into a HashMap.
    ///
    /// Returns None if the field is rejected or doesn't exist.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `map_addr` is a valid, properly aligned pointer to an instance of the map type.
    /// - `field` corresponds to an existing field in the map's schema.
    /// - Any modifications made via the returned FieldSlot maintain the field's type invariants.
    /// - The data pointed to by `map_addr` remains valid for the lifetime of the returned FieldSlot.
    unsafe fn get_field_slot<'a>(
        &self,
        map_addr: &mut ShapeUninit,
        field: MapField<'_>,
    ) -> Option<FieldSlot<'a>>;
}

/// Manipulator for struct-like types with known field offsets
pub struct StructManipulator {
    /// the overall shape of the struct
    pub shape: Shape,

    /// field offsets
    pub field_offsets: &'static [u32],
}

impl MapManipulator for StructManipulator {
    unsafe fn get_field_slot<'a>(
        &self,
        map_addr: &mut ShapeUninit,
        field: MapField<'_>,
    ) -> Option<FieldSlot<'a>> {
        if let ShapeKind::Map(map_shape) = self.shape.shape {
            if let Some((index, _)) = map_shape
                .fields
                .iter()
                .enumerate()
                .find(|(_, f)| f.name == field.name)
            {
                let offset = self.field_offsets[index] as usize;
                let field_addr = unsafe { map_addr.get_addr(&self.shape).add(offset) };
                Some(FieldSlot::new(field_addr as *mut _))
            } else {
                None
            }
        } else {
            panic!(
                "Unexpected shape kind: expected Map, found {:?}",
                self.shape.shape
            );
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
