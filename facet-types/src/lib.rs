//! structs and vtable definitions used by Facet

// TODO: mark `non_exhaustive`, add `const fn` builder patterns

use core::fmt;
use std::alloc::Layout;

use facet_opaque::OpaqueUninit;
use typeid::ConstTypeId;

mod list;
pub use list::*;

mod map;
pub use map::*;

mod value;
pub use value::*;

/// Schema for reflection of a type
#[derive(Clone, Copy, Debug)]
pub struct Shape {
    /// Size, alignment
    pub layout: Layout,

    /// VTable for common operations. This is indirected because the vtable might
    /// have different functions implemented based on generic type parameters:
    /// HashMap<K, V> is not even constructible if `K` is not `Hash` + `Eq`.
    pub vtable: &'static ValueVTable,

    /// Details/contents of the value
    pub def: Def,
}

impl Shape {
    /// Checks if a shape has the given characteristic.
    pub const fn is(&'static self, characteristic: Characteristic) -> bool {
        match characteristic {
            // Marker traits
            Characteristic::Send => self.vtable.marker_traits.contains(MarkerTraits::SEND),
            Characteristic::Sync => self.vtable.marker_traits.contains(MarkerTraits::SYNC),
            Characteristic::Copy => self.vtable.marker_traits.contains(MarkerTraits::COPY),
            Characteristic::Eq => self.vtable.marker_traits.contains(MarkerTraits::EQ),

            // Functionality traits
            Characteristic::Clone => self.vtable.clone_into.is_some(),
            Characteristic::Debug => self.vtable.debug.is_some(),
            Characteristic::PartialEq => self.vtable.eq.is_some(),
            Characteristic::PartialOrd => self.vtable.partial_ord.is_some(),
            Characteristic::Ord => self.vtable.ord.is_some(),
            Characteristic::Hash => self.vtable.hash.is_some(),
            Characteristic::Default => self.vtable.default_in_place.is_some(),
        }
    }

    /// Check if this shape implements the Send trait
    pub const fn is_send(&'static self) -> bool {
        self.is(Characteristic::Send)
    }

    /// Check if this shape implements the Sync trait
    pub const fn is_sync(&'static self) -> bool {
        self.is(Characteristic::Sync)
    }

    /// Check if this shape implements the Copy trait
    pub const fn is_copy(&'static self) -> bool {
        self.is(Characteristic::Copy)
    }

    /// Check if this shape implements the Eq trait
    pub const fn is_eq(&'static self) -> bool {
        self.is(Characteristic::Eq)
    }

    /// Check if this shape implements the Clone trait
    pub const fn is_clone(&'static self) -> bool {
        self.is(Characteristic::Clone)
    }

    /// Check if this shape implements the Debug trait
    pub const fn is_debug(&'static self) -> bool {
        self.is(Characteristic::Debug)
    }

    /// Check if this shape implements the PartialEq trait
    pub const fn is_partial_eq(&'static self) -> bool {
        self.is(Characteristic::PartialEq)
    }

    /// Check if this shape implements the PartialOrd trait
    pub const fn is_partial_ord(&'static self) -> bool {
        self.is(Characteristic::PartialOrd)
    }

    /// Check if this shape implements the Ord trait
    pub const fn is_ord(&'static self) -> bool {
        self.is(Characteristic::Ord)
    }

    /// Check if this shape implements the Hash trait
    pub const fn is_hash(&'static self) -> bool {
        self.is(Characteristic::Hash)
    }

    /// Check if this shape implements the Default trait
    pub const fn is_default(&'static self) -> bool {
        self.is(Characteristic::Default)
    }

    /// Writes the name of this type to the given formatter
    pub fn write_type_name(&self, f: &mut fmt::Formatter<'_>, opts: TypeNameOpts) -> fmt::Result {
        (self.vtable.type_name)(f, opts)
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.def == other.def && self.layout == other.layout
    }
}

impl Eq for Shape {}

impl std::hash::Hash for Shape {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.def.hash(state);
        self.layout.hash(state);
    }
}

impl Shape {
    /// Check if this shape is of the given type
    pub fn is_shape(&'static self, other: &'static Shape) -> bool {
        self == other
    }

    /// Assert that this shape is equal to the given shape, panicking if it's not
    pub fn assert_shape(&'static self, other: &'static Shape) {
        assert!(
            self.is_shape(other),
            "Shape mismatch: expected {other}, found {self}",
        );
    }
}

// Helper struct to format the name for display
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.vtable.type_name)(f, TypeNameOpts::default())
    }
}

impl Shape {
    /// Heap-allocate a value of this shape
    #[inline]
    pub fn allocate(&self) -> OpaqueUninit<'static> {
        OpaqueUninit::new(unsafe { std::alloc::alloc(self.layout) })
    }
}

/// Errors encountered when calling `field_by_index` or `field_by_name`
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FieldError {
    /// `field_by_index` was called on a dynamic collection, that has no
    /// static fields. a map doesn't have a "first field", it can only
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

/// Common fields for struct-like types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct StructDef {
    /// all fields, in declaration order (not necessarily in memory order)
    pub fields: &'static [Field],
}

/// Describes a field in a struct or tuple
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Field {
    /// key for the struct field (for tuples and tuple-structs, this is the 0-based index)
    pub name: &'static str,

    /// schema of the inner type
    pub shape: &'static Shape,

    /// offset of the field in the struct (obtained through `std::mem::offset_of`)
    pub offset: usize,

    /// flags for the field (e.g. sensitive, etc.)
    pub flags: FieldFlags,
}

bitflags::bitflags! {
    /// Flags that can be applied to fields to modify their behavior
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FieldFlags: u64 {
        /// An empty set of flags
        const EMPTY = 0;

        /// Flag indicating this field contains sensitive data that should not be displayed
        const SENSITIVE = 1 << 0;
    }
}

impl Default for FieldFlags {
    #[inline(always)]
    fn default() -> Self {
        Self::EMPTY
    }
}

impl std::fmt::Display for FieldFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "none");
        }

        // Define a vector of flag entries: (flag, name)
        let flags = [
            (FieldFlags::SENSITIVE, "sensitive"),
            // Future flags can be easily added here:
            // (FieldFlags::SOME_FLAG, "some_flag"),
            // (FieldFlags::ANOTHER_FLAG, "another_flag"),
        ];

        // Write all active flags with proper separators
        let mut is_first = true;
        for (flag, name) in flags {
            if self.contains(flag) {
                if !is_first {
                    write!(f, ", ")?;
                }
                is_first = false;
                write!(f, "{}", name)?;
            }
        }

        Ok(())
    }
}

/// Fields for map types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MapDef {
    /// vtable for interacting with the map
    pub vtable: &'static MapVTable,
    /// shape of the keys in the map
    pub k: &'static Shape,
    /// shape of the values in the map
    pub v: &'static Shape,
}

/// Fields for list types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ListDef {
    /// vtable for interacting with the list
    pub vtable: &'static ListVTable,
    /// shape of the items in the list
    pub t: &'static Shape,
}

/// Fields for enum types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct EnumDef {
    /// representation of the enum (u8, u16, etc.)
    pub repr: EnumRepr,
    /// all variants for this enum
    pub variants: &'static [Variant],
}

/// Describes a variant of an enum
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Variant {
    /// Name of the variant
    pub name: &'static str,

    /// Discriminant value (if available)
    pub discriminant: Option<i64>,

    /// Kind of variant (unit, tuple, or struct)
    pub kind: VariantKind,
}

/// Represents the different kinds of variants that can exist in a Rust enum
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VariantKind {
    /// Unit variant (e.g., `None` in Option)
    Unit,

    /// Tuple variant with unnamed fields (e.g., `Some(T)` in Option)
    Tuple {
        /// List of fields contained in the tuple variant
        fields: &'static [Field],
    },

    /// Struct variant with named fields (e.g., `Struct { field: T }`)
    Struct {
        /// List of fields contained in the struct variant
        fields: &'static [Field],
    },
}

/// All possible representations for Rust enums
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EnumRepr {
    /// Default representation (compiler-dependent)
    Default,
    /// u8 representation (#[repr(u8)])
    U8,
    /// u16 representation (#[repr(u16)])
    U16,
    /// u32 representation (#[repr(u32)])
    U32,
    /// u64 representation (#[repr(u64)])
    U64,
    /// usize representation (#[repr(usize)])
    USize,
    /// i8 representation (#[repr(i8)])
    I8,
    /// i16 representation (#[repr(i16)])
    I16,
    /// i32 representation (#[repr(i32)])
    I32,
    /// i64 representation (#[repr(i64)])
    I64,
    /// isize representation (#[repr(isize)])
    ISize,
}

impl Default for EnumRepr {
    fn default() -> Self {
        Self::Default
    }
}

/// Definition for scalar types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ScalarDef {
    /// The TypeId of the scalar type
    pub type_id: ConstTypeId,
}

impl ScalarDef {
    /// Create a new ScalarDef with the given TypeId
    pub const fn of<T>() -> Self {
        Self {
            type_id: ConstTypeId::of::<T>(),
        }
    }
}

/// The definition of a shape: is it more like a struct, a map, a list?
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Def {
    /// Scalar — those don't have a def, they're not composed of other things.
    /// You can interact with them through [`ValueVTable`].
    ///
    /// e.g. `u32`, `String`, `bool`, `SocketAddr`, etc.
    Scalar(ScalarDef),

    /// Struct with statically-known, named fields
    ///
    /// e.g. `struct Struct { field: u32 }`
    Struct(StructDef),

    /// Tuple-struct, with numbered fields
    ///
    /// e.g. `struct TupleStruct(u32, u32);`
    TupleStruct(StructDef),

    /// Tuple, with numbered fields
    ///
    /// e.g. `(u32, u32);`
    Tuple(StructDef),

    /// Map — keys are dynamic (and strings, sorry), values are homogeneous
    ///
    /// e.g. `Map<String, T>`
    Map(MapDef),

    /// Ordered list of heterogenous values, variable size
    ///
    /// e.g. `Vec<T>`
    List(ListDef),

    /// Enum with variants
    ///
    /// e.g. `enum Enum { Variant1, Variant2 }`
    Enum(EnumDef),
}

/// A characteristic a shape can have
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Characteristic {
    // Marker traits
    /// Implements Send
    Send,

    /// Implements Sync
    Sync,

    /// Implements Copy
    Copy,

    /// Implements Eq
    Eq,

    // Functionality traits
    /// Implements Clone
    Clone,

    /// Implements Debug
    Debug,

    /// Implements PartialEq
    PartialEq,

    /// Implements PartialOrd
    PartialOrd,

    /// Implements Ord
    Ord,

    /// Implements Hash
    Hash,

    /// Implements Default
    Default,
}

impl Characteristic {
    /// Checks if all shapes have the given characteristic.
    pub const fn all(self, shapes: &'static [&'static Shape]) -> bool {
        let mut i = 0;
        while i < shapes.len() {
            if !shapes[i].is(self) {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Checks if any shape has the given characteristic.
    pub const fn any(self, shapes: &'static [&'static Shape]) -> bool {
        let mut i = 0;
        while i < shapes.len() {
            if shapes[i].is(self) {
                return true;
            }
            i += 1;
        }
        false
    }

    /// Checks if none of the shapes have the given characteristic.
    pub const fn none(self, shapes: &'static [&'static Shape]) -> bool {
        let mut i = 0;
        while i < shapes.len() {
            if shapes[i].is(self) {
                return false;
            }
            i += 1;
        }
        true
    }
}
