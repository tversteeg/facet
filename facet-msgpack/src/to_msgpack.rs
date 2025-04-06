use facet_peek::Peek;
use facet_trait::{Facet, ShapeExt as _};
use log::trace;
use std::io::{self, Write};

/// Serializes any Facet type to MessagePack bytes
pub fn to_vec<T: Facet>(value: &T) -> Vec<u8> {
    let mut buffer = Vec::new();
    let peek = Peek::new(value);
    to_writer(peek, &mut buffer).unwrap();
    buffer
}

/// Serializes any Facet type to a writer in MessagePack format
pub fn to_writer<W: Write>(peek: Peek<'_>, writer: &mut W) -> io::Result<()> {
    fn serialize_value<W: Write>(peek: Peek<'_>, writer: &mut W) -> io::Result<()> {
        match peek {
            Peek::Value(pv) => {
                trace!("Serializing scalar");
                if pv.shape().is_type::<String>() {
                    let value = unsafe { pv.data().as_ref::<String>() };
                    write_str(writer, value)
                } else if pv.shape().is_type::<u64>() {
                    let value = unsafe { pv.data().as_ref::<u64>() };
                    write_u64(writer, *value)
                } else {
                    todo!("Unsupported scalar type: {}", pv.shape())
                }
            }
            Peek::Struct(ps) => {
                trace!("Serializing struct");

                // Write map header
                let fields: Vec<_> = ps.fields().collect();
                write_map_len(writer, fields.len())?;

                // Write fields
                for (name, field_peek) in fields {
                    write_str(writer, name)?;
                    serialize_value(Peek::Value(field_peek), writer)?;
                }
                Ok(())
            }
            _ => {
                todo!("Unsupported type: {:?}", peek)
            }
        }
    }

    serialize_value(peek, writer)
}

fn write_str<W: Write>(writer: &mut W, s: &str) -> io::Result<()> {
    let bytes = s.as_bytes();
    let len = bytes.len();

    match len {
        0..=31 => {
            // fixstr
            writer.write_all(&[(0xa0 | len as u8)])?;
        }
        32..=255 => {
            // str8
            writer.write_all(&[0xd9, len as u8])?;
        }
        256..=65535 => {
            // str16
            writer.write_all(&[0xda])?;
            writer.write_all(&(len as u16).to_be_bytes())?;
        }
        _ => {
            // str32
            writer.write_all(&[0xdb])?;
            writer.write_all(&(len as u32).to_be_bytes())?;
        }
    }
    writer.write_all(bytes)
}

fn write_u64<W: Write>(writer: &mut W, n: u64) -> io::Result<()> {
    match n {
        0..=127 => {
            // positive fixint
            writer.write_all(&[n as u8])
        }
        128..=255 => {
            // uint8
            writer.write_all(&[0xcc, n as u8])
        }
        256..=65535 => {
            // uint16
            writer.write_all(&[0xcd])?;
            writer.write_all(&(n as u16).to_be_bytes())
        }
        65536..=4294967295 => {
            // uint32
            writer.write_all(&[0xce])?;
            writer.write_all(&(n as u32).to_be_bytes())
        }
        _ => {
            // uint64
            writer.write_all(&[0xcf])?;
            writer.write_all(&n.to_be_bytes())
        }
    }
}

fn write_map_len<W: Write>(writer: &mut W, len: usize) -> io::Result<()> {
    match len {
        0..=15 => {
            // fixmap
            writer.write_all(&[(0x80 | len as u8)])
        }
        16..=65535 => {
            // map16
            writer.write_all(&[0xde])?;
            writer.write_all(&(len as u16).to_be_bytes())
        }
        _ => {
            // map32
            writer.write_all(&[0xdf])?;
            writer.write_all(&(len as u32).to_be_bytes())
        }
    }
}
