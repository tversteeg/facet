/// MessagePack type tags
/// As defined in the MessagePack specification: https://github.com/msgpack/msgpack/blob/master/spec.md
/// Nil format - Represents nil/null values
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-nil
pub const MSGPACK_NIL: u8 = 0xc0;

/// Boolean format family - Represents true/false values
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-bool
pub const MSGPACK_FALSE: u8 = 0xc2;
pub const MSGPACK_TRUE: u8 = 0xc3;

/// Binary format family - Represents byte arrays
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-bin
pub const MSGPACK_BIN8: u8 = 0xc4;
pub const MSGPACK_BIN16: u8 = 0xc5;
pub const MSGPACK_BIN32: u8 = 0xc6;

/// Extension format family - Represents custom type information with byte arrays
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-ext
pub const MSGPACK_EXT8: u8 = 0xc7;
pub const MSGPACK_EXT16: u8 = 0xc8;
pub const MSGPACK_EXT32: u8 = 0xc9;

/// Float format family - Represents IEEE 754 floating point numbers
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-float
pub const MSGPACK_FLOAT32: u8 = 0xca;
pub const MSGPACK_FLOAT64: u8 = 0xcb;
/// Unsigned integer format family - Represents unsigned integers
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#int-format-family
pub const MSGPACK_UINT8: u8 = 0xcc;
pub const MSGPACK_UINT16: u8 = 0xcd;
pub const MSGPACK_UINT32: u8 = 0xce;
pub const MSGPACK_UINT64: u8 = 0xcf;

/// Signed integer format family - Represents signed integers
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#int-format-family
pub const MSGPACK_INT8: u8 = 0xd0;
pub const MSGPACK_INT16: u8 = 0xd1;
pub const MSGPACK_INT32: u8 = 0xd2;
pub const MSGPACK_INT64: u8 = 0xd3;

/// Fixed-size extension format family - Represents custom type information with fixed-size byte arrays
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-ext
pub const MSGPACK_FIXEXT1: u8 = 0xd4;
pub const MSGPACK_FIXEXT2: u8 = 0xd5;
pub const MSGPACK_FIXEXT4: u8 = 0xd6;
pub const MSGPACK_FIXEXT8: u8 = 0xd7;
pub const MSGPACK_FIXEXT16: u8 = 0xd8;

/// String format family - Represents UTF-8 string
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-str
pub const MSGPACK_STR8: u8 = 0xd9;
pub const MSGPACK_STR16: u8 = 0xda;
pub const MSGPACK_STR32: u8 = 0xdb;

/// Array format family - Represents arrays of arbitrary values
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-array
pub const MSGPACK_ARRAY16: u8 = 0xdc;
pub const MSGPACK_ARRAY32: u8 = 0xdd;

/// Map format family - Represents key-value maps
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-map
pub const MSGPACK_MAP16: u8 = 0xde;
pub const MSGPACK_MAP32: u8 = 0xdf;

/// Positive fixint format family - Represents positive integers from 0 to 127 in a single byte
/// The first bit is 0, and the remaining 7 bits store the value
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#int-format-family
pub const MSGPACK_POSFIXINT_MIN: u8 = 0x00;
pub const MSGPACK_POSFIXINT_MAX: u8 = 0x7f;

/// Negative fixint format family - Represents negative integers from -1 to -32 in a single byte
/// The first 3 bits are 111, and the remaining 5 bits store the absolute value minus 1
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#int-format-family
pub const MSGPACK_NEGFIXINT_MIN: i8 = -0x20;
pub const MSGPACK_NEGFIXINT_MAX: i8 = -0x01;

/// Fixstr format family - Represents strings up to 31 bytes in a compact format
/// The first 3 bits are 101, and the remaining 5 bits store the length
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-str
pub const MSGPACK_FIXSTR_MIN: u8 = 0xa0;
pub const MSGPACK_FIXSTR_MAX: u8 = 0xbf;

/// Fixarray format family - Represents arrays with up to 15 elements in a compact format
/// The first 4 bits are 1001, and the remaining 4 bits store the length
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-array
pub const MSGPACK_FIXARRAY_MIN: u8 = 0x90;
pub const MSGPACK_FIXARRAY_MAX: u8 = 0x9f;

/// Fixmap format family - Represents maps with up to 15 key-value pairs in a compact format
/// The first 4 bits are 1000, and the remaining 4 bits store the length
/// Ref: https://github.com/msgpack/msgpack/blob/master/spec.md#formats-map
pub const MSGPACK_FIXMAP_MIN: u8 = 0x80;
pub const MSGPACK_FIXMAP_MAX: u8 = 0x8f;
