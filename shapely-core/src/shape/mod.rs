use std::alloc::Layout;

use typeid::ConstTypeId;

use crate::{ListVTable, MapVTable, OpaqueUninit, Shapely, TypeNameOpts, ValueVTable};

mod pretty_print;

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
    /// Create a new shape for a type
    pub fn of_type<T: Shapely>() -> &'static Self {
        T::SHAPE
    }

    /// Create a new shape for a type
    pub fn of_val<T: Shapely>(_: &T) -> &'static Self {
        T::SHAPE
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
    pub fn is_type<Other: Shapely>(&'static self) -> bool {
        self == Other::SHAPE
    }

    /// Check if this shape is of the given type
    pub fn is_shape(&'static self, other: &'static Shape) -> bool {
        self == other
    }

    /// Assert that this shape is of the given type, panicking if it's not
    pub fn assert_type<Other: Shapely>(&'static self) {
        assert!(
            self.is_type::<Other>(),
            "Type mismatch: expected {:?}, found {:?}",
            ShapeDebug(Other::SHAPE),
            ShapeDebug(self)
        );
    }

    /// Assert that this shape is equal to the given shape, panicking if it's not
    pub fn assert_shape(&'static self, other: &'static Shape) {
        assert!(
            self.is_shape(other),
            "Shape mismatch: expected {:?}, found {:?}",
            ShapeDebug(other),
            ShapeDebug(self)
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

/// Debug wrapper implementation for Shape
pub struct ShapeDebug(pub &'static Shape);

impl std::fmt::Debug for ShapeDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.pretty_print_recursive(f)
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

/// Flags that can be applied to fields to modify their behavior
///
/// # Examples
///
/// ```rust
/// use shapely_core::FieldFlags;
///
/// // Create flags with the sensitive bit set
/// let flags = FieldFlags::SENSITIVE;
/// assert!(flags.contains(FieldFlags::SENSITIVE));
///
/// // Combine multiple flags using bitwise OR
/// let flags = FieldFlags::SENSITIVE | FieldFlags::EMPTY;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldFlags(u64);

impl FieldFlags {
    /// An empty set of flags
    pub const EMPTY: Self = Self(0);

    /// Flag indicating this field contains sensitive data that should not be displayed
    pub const SENSITIVE: Self = Self(1 << 0);

    /// Returns true if the given flag is set
    #[inline]
    pub fn contains(&self, flag: FieldFlags) -> bool {
        self.0 & flag.0 != 0
    }

    /// Sets the given flag and returns self for chaining
    #[inline]
    pub fn set_flag(&mut self, flag: FieldFlags) -> &mut Self {
        self.0 |= flag.0;
        self
    }

    /// Unsets the given flag and returns self for chaining
    #[inline]
    pub fn unset_flag(&mut self, flag: FieldFlags) -> &mut Self {
        self.0 &= !flag.0;
        self
    }

    /// Creates a new FieldFlags with the given flag set
    #[inline]
    pub const fn with_flag(flag: FieldFlags) -> Self {
        Self(flag.0)
    }
}

impl std::ops::BitOr for FieldFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for FieldFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
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
        if self.0 == 0 {
            return write!(f, "none");
        }

        // Define a vector of flag entries: (flag bit, name)
        let flags = [
            (Self::SENSITIVE.0, "sensitive"),
            // Future flags can be easily added here:
            // (Self::SOME_FLAG.0, "some_flag"),
            // (Self::ANOTHER_FLAG.0, "another_flag"),
        ];

        // Write all active flags with proper separators
        let mut is_first = true;
        for (bit, name) in flags {
            if self.0 & bit != 0 {
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
