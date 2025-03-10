//! Provides the core traits for thonk

/// Provides reflection so you can thonk about your types.
pub trait Thonk {
    /// Returns the thonk schema, as bytecode.
    fn thonk_schema() -> &'static [u8];
}

#[derive(Pod, Zeroable, Debug, Clone, Copy)]
pub enum RoughType {
    /// Associates keys with values
    Map,

    /// List of values (length known or not)
    Array,

    /// UTF-8 string
    String,

    /// Raw bytes
    Bytes,

    Bool,
    U64,
    I64,
    F64,
}
