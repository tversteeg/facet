use std::{alloc::Layout, any::TypeId, ptr::NonNull};

mod pretty_print;

mod opaque;
pub use opaque::*;

mod struct_shape;
pub use struct_shape::*;

mod enum_shape;
pub use enum_shape::*;

mod scalar_shape;
pub use scalar_shape::*;

mod list_shape;
pub use list_shape::*;

mod map_shape;
pub use map_shape::*;

mod peekpoke;
pub use peekpoke::*;

/// Schema for reflection of a type
#[derive(Clone, Copy)]
pub struct Shape {
    /// A descriptive name for the type, e.g. `u64`, or `Person`
    pub name: NameFn,

    /// The typeid of the underlying type
    pub typeid: TypeId,

    /// Size, alignment
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

/// Options for formatting the name of a type
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct NameOpts {
    /// as long as this is > 0, keep formatting the type parameters
    /// when it reaches 0, format type parameters as `...`
    /// if negative, all type parameters are formatted
    pub recurse_ttl: isize,
}

impl Default for NameOpts {
    fn default() -> Self {
        Self { recurse_ttl: -1 }
    }
}

impl NameOpts {
    /// Create a new `NameOpts` for which none of the type parameters are formatted
    pub fn none() -> Self {
        Self { recurse_ttl: 0 }
    }

    /// Create a new `NameOpts` for which only the direct children are formatted
    pub fn one() -> Self {
        Self { recurse_ttl: 1 }
    }

    /// Decrease the `recurse_ttl` — if it's != 0, returns options to pass when
    /// formatting children type parameters.
    ///
    /// If this returns `None` and you have type parameters, you should render a
    /// `…` (unicode ellipsis) character instead of your list of types.
    ///
    /// See the implementation for `Vec` for examples.
    pub fn for_children(&self) -> Option<Self> {
        if self.recurse_ttl > 0 {
            Some(Self {
                recurse_ttl: self.recurse_ttl - 1,
            })
        } else {
            None
        }
    }
}

/// A function that formats the name of a type.
///
/// This helps avoid allocations, and it takes options.
pub type NameFn = fn(f: &mut std::fmt::Formatter, opts: NameOpts) -> std::fmt::Result;

// Helper struct to format the name for display
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.name)(f, NameOpts::default())
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

    /// Returns a slice of statically known fields. Fields that are not in there might still be inserted if it's a dynamic collection.
    pub fn known_fields(&self) -> &'static [Field] {
        match self.innards {
            Innards::Struct { fields } => fields,
            _ => &[],
        }
    }

    /// Returns a reference to a field with the given name, if it exists
    pub fn field_by_name(&self, name: &str) -> Option<&Field> {
        // this is O(n), but shrug. maybe phf in the future? who knows.
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

    /// Returns a slice of enum variants, if this shape represents an enum
    pub fn variants(&self) -> &'static [Variant] {
        match self.innards {
            Innards::Enum { variants, repr: _ } => variants,
            _ => &[],
        }
    }

    /// Returns a reference to a variant with the given name, if it exists
    pub fn variant_by_name(&self, name: &str) -> Option<&Variant> {
        self.variants().iter().find(|variant| variant.name == name)
    }

    /// Returns a reference to a variant with the given index, if it exists
    pub fn variant_by_index(&self, index: usize) -> Result<&Variant, VariantError> {
        match self.innards {
            Innards::Enum { variants, repr: _ } => {
                variants.get(index).ok_or(VariantError::IndexOutOfBounds)
            }
            _ => Err(VariantError::NotAnEnum),
        }
    }

    /// Returns the enum representation, if this shape represents an enum
    pub fn enum_repr(&self) -> Option<EnumRepr> {
        match self.innards {
            Innards::Enum { variants: _, repr } => Some(repr),
            _ => None,
        }
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

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print_recursive(f)
    }
}

/// The shape of a schema: is it more map-shaped, array-shaped, scalar-shaped?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Innards {
    /// Struct with statically-known, named fields
    ///
    /// e.g. `struct Struct { field: u32 }`
    Struct {
        /// all fields, in declaration order (not necessarily in memory order)
        fields: &'static [Field],
    },

    /// Tuple-struct, with numbered fields
    ///
    /// e.g. `struct TupleStruct(u32, u32);`
    TupleStruct {
        /// all fields, in declaration order (not necessarily in memory order)
        fields: &'static [Field],
    },

    /// Tuple, with numbered fields
    ///
    /// e.g. `(u32, u32);`
    Tuple {
        /// all fields, in declaration order (not necessarily in memory order)
        fields: &'static [Field],
    },

    /// Map — keys are dynamic (and strings, sorry), values are homogeneous
    ///
    /// e.g. `Map<String, T>`
    Map {
        /// vtable for interacting with the map
        vtable: MapVTable,

        /// shape of the values in the map (keys must be String, sorry)
        value_shape: ShapeDesc,
    },

    /// Ordered list of heterogenous values, variable size
    ///
    /// e.g. `Vec<T>`
    List {
        /// vtable for interacting with the list
        vtable: ListVTable,

        /// shape of the items in the list
        item_shape: ShapeDesc,
    },

    /// Scalar — known base type
    ///
    /// e.g. `u32`, `String`, `bool`, `SocketAddr`, etc.
    Scalar { vtable: ScalarVTable },

    /// Enum with variants
    ///
    /// e.g. `enum Enum { Variant1, Variant2 }`
    Enum {
        /// all variants for this enum
        variants: &'static [Variant],

        /// representation of the enum
        repr: EnumRepr,
    },
}

/// A function that returns a shape. There should only be one of these per concrete type in a
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
