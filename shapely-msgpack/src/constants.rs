#[allow(dead_code)]
// MessagePack type tags
pub const MSGPACK_NIL: u8 = 0xc0;
pub const MSGPACK_FALSE: u8 = 0xc2;
pub const MSGPACK_TRUE: u8 = 0xc3;
pub const MSGPACK_BIN8: u8 = 0xc4;
pub const MSGPACK_BIN16: u8 = 0xc5;
pub const MSGPACK_BIN32: u8 = 0xc6;
pub const MSGPACK_EXT8: u8 = 0xc7;
pub const MSGPACK_EXT16: u8 = 0xc8;
pub const MSGPACK_EXT32: u8 = 0xc9;
pub const MSGPACK_FLOAT32: u8 = 0xca;
pub const MSGPACK_FLOAT64: u8 = 0xcb;
pub const MSGPACK_UINT8: u8 = 0xcc;
pub const MSGPACK_UINT16: u8 = 0xcd;
pub const MSGPACK_UINT32: u8 = 0xce;
pub const MSGPACK_UINT64: u8 = 0xcf;
pub const MSGPACK_INT8: u8 = 0xd0;
pub const MSGPACK_INT16: u8 = 0xd1;
pub const MSGPACK_INT32: u8 = 0xd2;
pub const MSGPACK_INT64: u8 = 0xd3;
pub const MSGPACK_FIXEXT1: u8 = 0xd4;
pub const MSGPACK_FIXEXT2: u8 = 0xd5;
pub const MSGPACK_FIXEXT4: u8 = 0xd6;
pub const MSGPACK_FIXEXT8: u8 = 0xd7;
pub const MSGPACK_FIXEXT16: u8 = 0xd8;
pub const MSGPACK_STR8: u8 = 0xd9;
pub const MSGPACK_STR16: u8 = 0xda;
pub const MSGPACK_STR32: u8 = 0xdb;
pub const MSGPACK_ARRAY16: u8 = 0xdc;
pub const MSGPACK_ARRAY32: u8 = 0xdd;
pub const MSGPACK_MAP16: u8 = 0xde;
pub const MSGPACK_MAP32: u8 = 0xdf;

// Fixint
pub const MSGPACK_POSFIXINT_MIN: u8 = 0x00;
pub const MSGPACK_POSFIXINT_MAX: u8 = 0x7f;

// Negative fixint
pub const MSGPACK_NEGFIXINT_MIN: i8 = -0x20;
pub const MSGPACK_NEGFIXINT_MAX: i8 = -0x01;

// Fixstr
pub const MSGPACK_FIXSTR_MIN: u8 = 0xa0;
pub const MSGPACK_FIXSTR_MAX: u8 = 0xbf;

// Fixarray
pub const MSGPACK_FIXARRAY_MIN: u8 = 0x90;
pub const MSGPACK_FIXARRAY_MAX: u8 = 0x9f;

// Fixmap
pub const MSGPACK_FIXMAP_MIN: u8 = 0x80;
pub const MSGPACK_FIXMAP_MAX: u8 = 0x8f;
