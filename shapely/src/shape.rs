use std::{collections::HashSet, fmt::Formatter};

use nonmax::NonMaxU32;

use crate::{ShapeUninit, Slot};

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
            Innards::Map(map) => {
                for field in map.fields {
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
                if map.open_ended {
                    writeln!(
                        f,
                        "{:indent$}\x1b[1;31m(open-ended)\x1b[0m",
                        "",
                        indent = indent + Self::INDENT * 2
                    )?;
                }
            }
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

    pub fn slots(&self) -> Option<&'static dyn Slots> {
        match self.innards {
            Innards::Map(map_innards) => Some((map_innards.mk_slots)(*self)),
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
#[derive(Clone, Copy)]
pub struct MapInnards {
    /// Statically-known fields
    fields: &'static [MapField<'static>],

    /// Will allow setting fields outside of the ones listed in `fields`
    open_ended: bool,

    /// Slots for setting fields
    mk_slots: fn(Shape) -> &'static dyn Slots,
}

impl MapInnards {
    fn builder() -> MapInnardsBuilder {
        MapInnardsBuilder::default()
    }
}

#[derive(Default)]
pub struct MapInnardsBuilder {
    fields: Vec<MapField<'static>>,
    open_ended: bool,
    mk_slots: Option<fn(Shape) -> &'static dyn Slots>,
}

impl MapInnardsBuilder {
    pub fn field(mut self, field: MapField<'static>) -> Self {
        self.fields.push(field);
        self
    }

    pub fn open_ended(mut self, open_ended: bool) -> Self {
        self.open_ended = open_ended;
        self
    }

    pub fn mk_slots(mut self, mk_slots: fn(Shape) -> &'static dyn Slots) -> Self {
        self.mk_slots = Some(mk_slots);
        self
    }

    pub fn build(self) -> MapInnards {
        MapInnards {
            fields: self.fields.into_boxed_slice().leak(),
            open_ended: self.open_ended,
            mk_slots: self.mk_slots.expect("mk_slots is required"),
        }
    }
}

impl PartialEq for MapInnards {
    fn eq(&self, other: &Self) -> bool {
        self.fields == other.fields
            && self.open_ended == other.open_ended
            && std::ptr::eq(
                self.slots as *const dyn Slots,
                other.slots as *const dyn Slots,
            )
    }
}

impl Eq for MapInnards {}

impl std::hash::Hash for MapInnards {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.fields.hash(state);
        self.open_ended.hash(state);
        (self.slots as *const dyn Slots).hash(state);
    }
}

impl std::fmt::Debug for MapInnards {
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

/// Given the map's address, returns a FieldSlot for the requested field
pub trait Slots: Send + Sync {
    /// Returns a FieldSlot for a given field. If the map accommodates dynamically-added fields,
    /// this might, for example, insert an entry into a HashMap.
    ///
    /// Returns None if the field is not known and the data structure does not accommodate for arbitrary fields.
    fn slot<'a>(&'a mut self, map: &'a mut ShapeUninit, field: MapField<'_>) -> Option<Slot<'a>>;
}

/// Manipulator for struct-like types with known field offsets
pub struct StructManipulator {
    /// the overall shape of the struct
    pub shape: Shape,
}

impl Slots for StructManipulator {
    fn slot<'a>(&'a mut self, map: &'a mut ShapeUninit, field: MapField<'_>) -> Option<Slot<'a>> {
        if let Innards::Map(map_shape) = self.shape.innards {
            if let Some(field) = map_shape.fields.iter().find(|f| f.name == field.name) {
                if let Some(offset) = field.offset {
                    let field_addr =
                        unsafe { map.get_addr(&self.shape).add(offset.get() as usize) };
                    Some(Slot::for_struct_field(field_addr as *mut _))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            panic!(
                "Unexpected shape kind: expected Map, found {:?}",
                self.shape.innards
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
