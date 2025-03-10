use std::{collections::HashSet, fmt::Formatter};

use nonmax::NonMaxU32;

use crate::slot::{AllSlots, SlotsKind};

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
    pub innards: Innards,

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

        match &self.innards {
            Innards::Map(map) => match map {
                MapInnards::Struct { fields } => {
                    for field in *fields {
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
                }
                MapInnards::HashMap { value_shape } => {
                    writeln!(
                        f,
                        "{:indent$}\x1b[1;36mHashMap with arbitrary keys and value shape:\x1b[0m",
                        "",
                        indent = indent + Self::INDENT
                    )?;
                    value_shape().pretty_print_recursive_internal(
                        f,
                        printed_schemas,
                        indent + Self::INDENT * 2,
                    )?;
                }
            },
            Innards::Array(elem_schema) => {
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
            Innards::Transparent(inner_schema) => {
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
            Innards::Scalar(scalar) => {
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

    pub fn slots(&self) -> Option<AllSlots> {
        match self.innards {
            Innards::Map(map) => Some(
                match map {
                    MapInnards::Struct { .. } => SlotsKind::Struct,
                    MapInnards::HashMap { value_shape } => SlotsKind::HashMap { value_shape },
                }
                .to_slots(*self),
            ),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print_recursive(f)
    }
}

/// The shape of a schema: is it more map-shaped, array-shaped, scalar-shaped?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Innards {
    /// Associates keys with values
    Map(MapInnards),

    /// Ordered list of heterogenous values, variable size
    Array(&'static Shape),

    /// Transparent — forwards to another known schema
    Transparent(&'static Shape),

    /// Scalar — known based type
    Scalar(Scalar),
}

/// The shape of a map: works for structs, but also HashMap<String, String> for example
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapInnards {
    /// Struct with statically-known fields
    Struct {
        fields: &'static [MapField<'static>],
    },
    /// HashMap — keys are dynamic, values are homogeneous
    HashMap { value_shape: fn() -> Shape },
}

impl MapInnards {
    /// Creates a new MapInnards for a struct with the given fields
    pub fn for_struct(fields: &'static [MapField<'static>]) -> Self {
        MapInnards::Struct { fields }
    }

    /// Creates a new MapInnards for a HashMap
    pub fn for_hashmap(value_shape: fn() -> Shape) -> Self {
        MapInnards::HashMap { value_shape }
    }

    /// Returns a reference to the fields of this map
    pub fn static_fields(&self) -> &'static [MapField<'static>] {
        match self {
            MapInnards::Struct { fields } => fields,
            MapInnards::HashMap { .. } => &[],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapField<'s> {
    /// key for the map field
    pub name: &'s str,

    /// schema of the inner type
    pub schema: fn() -> Shape,

    /// offset of the field in the map, if known.
    ///
    /// For example, when deserializing a self-descriptive format like JSON, we're going to get
    /// some map fields with dynamically discovered field names, and they're not going to have
    /// an offset.
    ///
    /// However, when deserializing formats that are non-self descriptive and working from an
    /// existing shape, then their map fields are probably going to have offsets, especially if
    /// they're using derived macros.
    pub offset: Option<NonMaxU32>,
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
