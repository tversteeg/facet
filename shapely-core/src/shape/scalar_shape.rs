/// A scalar type in Rust, representing a single value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Scalar {
    /// Valid utf-8
    String,

    /// Not valid utf-8 🤷
    Bytes,

    /// Signed 8-bit integer
    I8,
    /// Signed 16-bit integer
    I16,
    /// Signed 32-bit integer
    I32,
    /// Signed 64-bit integer
    I64,
    /// Signed 128-bit integer
    I128,

    /// Unsigned 8-bit integer
    U8,
    /// Unsigned 16-bit integer
    U16,
    /// Unsigned 32-bit integer
    U32,
    /// Unsigned 64-bit integer
    U64,
    /// Unsigned 128-bit integer
    U128,

    /// 32-bit floating point
    F32,
    /// 64-bit floating point
    F64,

    /// Boolean value (true/false)
    Boolean,

    /// An empty tuple, null, undefined, whatever you wish
    Nothing,
}
