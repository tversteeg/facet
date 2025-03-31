use super::Field;

/// Describes a variant of an enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variant {
    /// Name of the variant
    pub name: &'static str,

    /// Discriminant value (if available)
    pub discriminant: Option<i64>,

    /// Kind of variant (unit, tuple, or struct)
    pub kind: VariantKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VariantKind {
    /// Unit variant (e.g., `None` in Option)
    Unit,

    /// Tuple variant with unnamed fields (e.g., `Some(T)` in Option)
    Tuple { fields: &'static [Field] },

    /// Struct variant with named fields (e.g., `Struct { field: T }`)
    Struct { fields: &'static [Field] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VariantError {
    /// `variant_by_index` was called with an index that is out of bounds.
    IndexOutOfBounds,

    /// `variant_by_name` or `variant_by_index` was called on a non-enum type.
    NotAnEnum,

    /// `variant_by_name` was called with a name that doesn't match any variant.
    NoSuchVariant,
}

impl std::error::Error for VariantError {}

impl std::fmt::Display for VariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariantError::IndexOutOfBounds => write!(f, "Variant index out of bounds"),
            VariantError::NotAnEnum => write!(f, "Not an enum"),
            VariantError::NoSuchVariant => write!(f, "No such variant"),
        }
    }
}
