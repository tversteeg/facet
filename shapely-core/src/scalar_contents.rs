use crate::Scalar;
use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};

/// Represents the contents of a scalar value with a lifetime.
/// This allows safe access to the actual values stored in memory.
#[derive(Debug, PartialEq)]
pub enum ScalarContents<'a> {
    /// A string value
    String(Cow<'a, str>),

    /// A byte array
    Bytes(Cow<'a, [u8]>),

    /// An i8 value
    I8(i8),

    /// An i16 value
    I16(i16),

    /// An i32 value
    I32(i32),

    /// An i64 value
    I64(i64),

    /// An i128 value
    I128(i128),

    /// A u8 value
    U8(u8),

    /// A u16 value
    U16(u16),

    /// A u32 value
    U32(u32),

    /// A u64 value
    U64(u64),

    /// A u128 value
    U128(u128),

    /// An f32 value
    F32(f32),

    /// An f64 value
    F64(f64),

    /// A boolean value
    Boolean(bool),

    /// Nothing (unit type)
    Nothing,

    /// Unknown scalar type
    Unknown,
}

impl Display for ScalarContents<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ScalarContents::String(s) => write!(f, "\"{}\"", s.escape_debug()),
            ScalarContents::Bytes(b) => {
                write!(f, "b\"")?;
                for &byte in b.iter().take(64) {
                    write!(f, "\\x{:02x}", byte)?;
                }
                if b.len() > 64 {
                    write!(f, "...")?;
                }
                write!(f, "\"")
            }
            ScalarContents::I8(v) => write!(f, "{}", v),
            ScalarContents::I16(v) => write!(f, "{}", v),
            ScalarContents::I32(v) => write!(f, "{}", v),
            ScalarContents::I64(v) => write!(f, "{}", v),
            ScalarContents::I128(v) => write!(f, "{}", v),
            ScalarContents::U8(v) => write!(f, "{}", v),
            ScalarContents::U16(v) => write!(f, "{}", v),
            ScalarContents::U32(v) => write!(f, "{}", v),
            ScalarContents::U64(v) => write!(f, "{}", v),
            ScalarContents::U128(v) => write!(f, "{}", v),
            ScalarContents::F32(v) => write!(f, "{}", v),
            ScalarContents::F64(v) => write!(f, "{}", v),
            ScalarContents::Boolean(v) => write!(f, "{}", v),
            ScalarContents::Nothing => write!(f, "()"),
            ScalarContents::Unknown => write!(f, "<unknown>"),
        }
    }
}

impl Scalar {
    /// Get the contents of a scalar value from a memory location.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it reads from raw memory.
    /// The caller must ensure that:
    /// 1. The pointer points to a valid, initialized value of the correct type
    /// 2. The memory is properly aligned for the type
    /// 3. The memory is not mutated while the returned ScalarContents is in use
    pub unsafe fn get_contents<'a>(&self, ptr: *const u8) -> ScalarContents<'a> {
        // In Rust 2024, unsafe operations need to be in an unsafe block
        // even if they're in an unsafe function
        unsafe {
            match self {
                Scalar::String => {
                    // Cast the pointer to a reference to String
                    let string_ref = &*(ptr as *const String);
                    ScalarContents::String(Cow::Borrowed(string_ref.as_str()))
                }
                Scalar::Bytes => {
                    // Cast the pointer to a reference to Vec<u8>
                    let bytes_ref = &*(ptr as *const Vec<u8>);
                    ScalarContents::Bytes(Cow::Borrowed(bytes_ref.as_slice()))
                }
                Scalar::I8 => ScalarContents::I8(*(ptr as *const i8)),
                Scalar::I16 => ScalarContents::I16(*(ptr as *const i16)),
                Scalar::I32 => ScalarContents::I32(*(ptr as *const i32)),
                Scalar::I64 => ScalarContents::I64(*(ptr as *const i64)),
                Scalar::I128 => ScalarContents::I128(*(ptr as *const i128)),
                Scalar::U8 => ScalarContents::U8(*ptr),
                Scalar::U16 => ScalarContents::U16(*(ptr as *const u16)),
                Scalar::U32 => ScalarContents::U32(*(ptr as *const u32)),
                Scalar::U64 => ScalarContents::U64(*(ptr as *const u64)),
                Scalar::U128 => ScalarContents::U128(*(ptr as *const u128)),
                Scalar::F32 => ScalarContents::F32(*(ptr as *const f32)),
                Scalar::F64 => ScalarContents::F64(*(ptr as *const f64)),
                Scalar::Boolean => ScalarContents::Boolean(*(ptr as *const bool)),
                Scalar::Nothing => ScalarContents::Nothing,
                _ => ScalarContents::Unknown,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_contents_display() {
        assert_eq!(
            format!("{}", ScalarContents::String(Cow::Borrowed("hello"))),
            "\"hello\""
        );
        assert_eq!(format!("{}", ScalarContents::I32(42)), "42");
        assert_eq!(format!("{}", ScalarContents::Boolean(true)), "true");
        assert_eq!(format!("{}", ScalarContents::Nothing), "()");
    }
}
