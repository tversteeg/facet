//! Provides the core traits for thonk

/// Provides reflection so you can thonk about your types.
pub trait Schematic {
    /// Returns the thonk schema
    fn schema() -> &'static Schema;
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    /// Associates keys with values
    Map,

    /// List of values (length known or not)
    Array,

    /// Scalar — must only have one field. Think transparent enums, etc.
    Scalar,
}

pub enum BuiltinFieldKind {
    String,
    Bytes,
    U64,
    I64,
    F64,
    Boolean,
}

pub struct Schema {
    pub name: &'static str,
    pub shape: Shape,
    pub fields: &'static [Field],
}

pub enum FieldKind {
    Builtin(BuiltinFieldKind),
    Nested(&'static Schema),
}

/// Describes a field in a schema
pub struct Field {
    pub offset: usize,
    pub kind: FieldKind,
}

impl Schematic for u64 {
    fn schema() -> &'static Schema {
        static SCHEMA: Schema = Schema {
            name: "u64",
            shape: Shape::Scalar,
            fields: &[Field {
                offset: 0,
                kind: FieldKind::Builtin(BuiltinFieldKind::U64),
            }],
        };
        &SCHEMA
    }
}
