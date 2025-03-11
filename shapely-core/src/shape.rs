use std::{alloc::Layout, any::TypeId, collections::HashSet, fmt::Formatter, ptr::NonNull};

/// Schema for reflection of a type
#[derive(Clone, Copy)]
pub struct Shape {
    /// A descriptive name for the type, e.g. `u64`, or `Person`
    pub name: NameFn,

    /// The typeid of the underlying type
    pub typeid: TypeId,

    /// Size & alignment
    pub layout: Layout,

    /// Details/contents of the value
    pub innards: Innards,

    /// Set the value at a given address to the default value for this type
    pub set_to_default: Option<SetToDefaultFn>,

    /// Drop the value at a given address
    ///
    /// # Safety
    ///
    /// This function should be called only for initialized values.
    /// It's the caller's responsibility to ensure the address points to a valid value.
    pub drop_in_place: Option<DropFn>,
}

pub type NameFn = fn(shape: &Shape, f: &mut std::fmt::Formatter) -> std::fmt::Result;

// Helper struct to format the name for display
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.name)(self, f)
    }
}

/// A function that sets a value to its default at a specific memory address
pub type SetToDefaultFn = unsafe fn(*mut u8);

/// A function that drops a value at a specific memory address
pub type DropFn = unsafe fn(*mut u8);

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.typeid == other.typeid
    }
}

impl Eq for Shape {}

impl std::hash::Hash for Shape {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.typeid.hash(state);
    }
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
            write!(
                f,
                "{:indent$}\x1b[1;33m",
                "",
                indent = indent
            )?;
            (self.name)(self, f)?;
            writeln!(
                f,
                "\x1b[0m (\x1b[1;31malready printed\x1b[0m)"
            )?;
            return Ok(());
        }

        write!(
            f,
            "{:indent$}\x1b[1;33m",
            "",
            indent = indent
        )?;
        (self.name)(self, f)?;
        writeln!(
            f,
            "\x1b[0m (size: \x1b[1;34m{}\x1b[0m, align: \x1b[1;35m{}\x1b[0m)",
            self.layout.size(),
            self.layout.align()
        )?;

        match &self.innards {
            Innards::Struct { fields } => {
                for field in *fields {
                    writeln!(
                        f,
                        "{:indent$}\x1b[1;32m{}\x1b[0m: ",
                        "",
                        field.name,
                        indent = indent + Self::INDENT
                    )?;
                    field.shape.get().pretty_print_recursive_internal(
                        f,
                        printed_schemas,
                        indent + Self::INDENT * 2,
                    )?;
                }
            }
            Innards::HashMap { value_shape } => {
                writeln!(
                    f,
                    "{:indent$}\x1b[1;36mHashMap with arbitrary keys and value shape:\x1b[0m",
                    "",
                    indent = indent + Self::INDENT
                )?;
                value_shape.get().pretty_print_recursive_internal(
                    f,
                    printed_schemas,
                    indent + Self::INDENT * 2,
                )?;
            }
            Innards::Array(elem_schema) => {
                write!(
                    f,
                    "{:indent$}\x1b[1;36mArray of:\x1b[0m ",
                    "",
                    indent = indent + Self::INDENT
                )?;
                elem_schema.get().pretty_print_recursive_internal(
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
                inner_schema.get().pretty_print_recursive_internal(
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

    /// Returns a slice of statically known fields. Fields that are not in there might still be inserted if it's a dynamic collection.
    pub fn known_fields(&self) -> &'static [Field] {
        match self.innards {
            Innards::Struct { fields } => fields,
            _ => &[],
        }
    }

    /// Returns a reference to a field with the given name, if it exists
    pub fn field_by_name(&self, name: &str) -> Option<&Field> {
        self.known_fields().iter().find(|field| field.name == name)
    }

    /// Returns a reference to a field with the given index, if it exists
    pub fn field_by_index(&self, index: usize) -> Result<&Field, FieldError> {
        match self.innards {
            Innards::Struct { fields } => fields.get(index).ok_or(FieldError::IndexOutOfBounds),
            _ => Err(FieldError::NotAStruct),
        }
    }

    /// Returns a dangling pointer for this shape.
    ///
    /// This is useful for zero-sized types (ZSTs) which don't need actual memory allocation,
    /// but still need a properly aligned "some address".
    ///
    /// # Safety
    ///
    /// This function returns a dangling pointer. It should only be used in contexts where
    /// a non-null pointer is required but no actual memory access will occur, such as for ZSTs.
    pub fn dangling(&self) -> NonNull<u8> {
        let dang = NonNull::dangling();
        let offset = dang.align_offset(self.layout.align());
        unsafe { dang.byte_add(offset) }
    }
}

/// Errors encountered when calling `field_by_index` or `field_by_name`
#[derive(Debug)]
pub enum FieldError {
    /// `field_by_index` was called on a dynamic collection, that has no
    /// static fields. a HashMap doesn't have a "first field", it can only
    /// associate by keys.
    NoStaticFields,

    /// `field_by_name` was called on a struct, and there is no static field
    /// with the given key.
    NoSuchStaticField,

    /// `field_by_index` was called on a fixed-size collection (like a tuple,
    /// a struct, or a fixed-size array) and the index was out of bounds.
    IndexOutOfBounds,

    /// `field_by_index` or `field_by_name` was called on a non-struct type.
    NotAStruct,
}

impl std::error::Error for FieldError {}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldError::NoStaticFields => write!(f, "No static fields available"),
            FieldError::NoSuchStaticField => write!(f, "No such static field"),
            FieldError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            FieldError::NotAStruct => write!(f, "Not a struct"),
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
    /// Struct with statically-known fields
    Struct { fields: &'static [Field] },

    /// HashMap — keys are dynamic, values are homogeneous
    HashMap { value_shape: ShapeDesc },

    /// Ordered list of heterogenous values, variable size
    Array(ShapeDesc),

    /// Transparent — forwards to another known schema
    Transparent(ShapeDesc),

    /// Scalar — known based type
    Scalar(Scalar),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Field {
    /// key for the map field
    pub name: &'static str,

    /// schema of the inner type
    pub shape: ShapeDesc,

    /// offset of the field in the map, if known.
    ///
    /// For example, when deserializing a self-descriptive format like JSON, we're going to get
    /// some map fields with dynamically discovered field names, and they're not going to have
    /// an offset.
    ///
    /// However, when deserializing formats that are non-self descriptive and working from an
    /// existing shape, then their map fields are probably going to have offsets, especially if
    /// they're using derived macros.
    pub offset: usize,
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

    /// An empty tuple, null, undefined, whatever you wish
    Nothing,
}

/// A function that returns a shape. There should only be one of these per concrete type in a
/// program. This enables optimizations.
#[derive(Clone, Copy)]
pub struct ShapeDesc(pub fn() -> Shape);

impl From<fn() -> Shape> for ShapeDesc {
    fn from(f: fn() -> Shape) -> Self {
        Self(f)
    }
}

impl ShapeDesc {
    /// Build the inner shape
    #[inline(always)]
    pub fn get(&self) -> Shape {
        (self.0)()
    }
}

impl PartialEq for ShapeDesc {
    fn eq(&self, other: &Self) -> bool {
        if std::ptr::eq(self.0 as *const (), other.0 as *const ()) {
            true
        } else {
            let self_shape = self.0();
            let other_shape = other.0();
            self_shape == other_shape
        }
    }
}

impl Eq for ShapeDesc {}

impl std::hash::Hash for ShapeDesc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the function pointer
        (self.0 as *const ()).hash(state);
    }
}

impl std::fmt::Debug for ShapeDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
