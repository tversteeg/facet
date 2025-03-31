use shapely::{Shape, ShapeDesc};
use std::io::{self, Write};

/// Serializes any Shapely type to JSON
pub fn to_json<W: Write>(
    data: *mut u8,
    shape_desc: ShapeDesc,
    writer: &mut W,
    indent: bool,
) -> io::Result<()> {
    use shapely::{Innards, Scalar};

    fn serialize_value<W: Write>(
        data: *const u8,
        shape: Shape,
        writer: &mut W,
        indent: bool,
        level: usize,
    ) -> io::Result<()> {
        match &shape.innards {
            Innards::Scalar(scalar) => match scalar {
                Scalar::String => {
                    let s = unsafe { &*(data as *const String) };
                    write!(writer, "\"{}\"", s.replace('"', "\\\""))
                }
                Scalar::Bytes => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "cowardly refusing to inject binary data straight into your JSON",
                )),
                Scalar::I8 => {
                    let value = unsafe { *(data as *const i8) };
                    write!(writer, "{value}")
                }
                Scalar::I16 => {
                    let value = unsafe { *(data as *const i16) };
                    write!(writer, "{value}")
                }
                Scalar::I32 => {
                    let value = unsafe { *(data as *const i32) };
                    write!(writer, "{value}")
                }
                Scalar::I64 => {
                    let value = unsafe { *(data as *const i64) };
                    write!(writer, "{value}")
                }
                Scalar::I128 => {
                    let value = unsafe { *(data as *const i128) };
                    write!(writer, "{value}")
                }
                Scalar::U8 => {
                    let value = unsafe { *data };
                    write!(writer, "{value}")
                }
                Scalar::U16 => {
                    let value = unsafe { *(data as *const u16) };
                    write!(writer, "{value}")
                }
                Scalar::U32 => {
                    let value = unsafe { *(data as *const u32) };
                    write!(writer, "{value}")
                }
                Scalar::U64 => {
                    let value = unsafe { *(data as *const u64) };
                    write!(writer, "{value}")
                }
                Scalar::U128 => {
                    let value = unsafe { *(data as *const u128) };
                    write!(writer, "{value}")
                }
                Scalar::F32 => {
                    let value = unsafe { *(data as *const f32) };
                    write!(writer, "{value}")
                }
                Scalar::F64 => {
                    let value = unsafe { *(data as *const f64) };
                    write!(writer, "{value}")
                }
                Scalar::Boolean => {
                    let value = unsafe { *(data as *const bool) };
                    write!(writer, "{}", if value { "true" } else { "false" })
                }
                Scalar::Nothing => {
                    write!(writer, "null")
                }
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "unsupported scalar type encountered",
                )),
            },
            Innards::Struct { fields } => {
                write!(writer, "{{")?;
                if indent {
                    writeln!(writer)?;
                }
                for (i, field) in fields.iter().enumerate() {
                    if indent {
                        write!(writer, "{:indent$}", "", indent = (level + 1) * 2)?;
                    }
                    write!(writer, "\"{}\":", field.name)?;
                    if indent {
                        write!(writer, " ")?;
                    }
                    let field_data = unsafe { data.add(field.offset) };
                    serialize_value(field_data, field.shape.get(), writer, indent, level + 1)?;
                    if i < fields.len() - 1 {
                        write!(writer, ",")?;
                    }
                    if indent {
                        writeln!(writer)?;
                    }
                }
                if indent {
                    write!(writer, "{:indent$}", "", indent = level * 2)?;
                }
                write!(writer, "}}")
            }
            Innards::List { vtable, item_shape } => {
                write!(writer, "[")?;
                if indent {
                    writeln!(writer)?;
                }

                unsafe {
                    let len = (vtable.len)(data);

                    for i in 0..len {
                        if indent {
                            write!(writer, "{:indent$}", "", indent = (level + 1) * 2)?;
                        }

                        let item_ptr = (vtable.get_item_ptr)(data, i);
                        serialize_value(item_ptr, item_shape.get(), writer, indent, level + 1)?;

                        if i < len - 1 {
                            write!(writer, ",")?;
                        }
                        if indent {
                            writeln!(writer)?;
                        }
                    }

                    if indent && len > 0 {
                        write!(writer, "{:indent$}", "", indent = level * 2)?;
                    }
                }
                write!(writer, "]")
            }
            Innards::Map {
                vtable,
                value_shape,
            } => {
                write!(writer, "{{")?;
                if indent {
                    writeln!(writer)?;
                }

                unsafe {
                    // Get an iterator over the HashMap
                    let iter_ptr = (vtable.iter)(data);

                    // Keep track of whether we need to write a comma
                    let mut first = true;

                    // Iterate over the key-value pairs
                    while let Some((key_ptr, value_ptr)) = (vtable.iter_vtable.next)(iter_ptr) {
                        if !first {
                            write!(writer, ",")?;
                            if indent {
                                writeln!(writer)?;
                            }
                        }
                        first = false;

                        if indent {
                            write!(writer, "{:indent$}", "", indent = (level + 1) * 2)?;
                        }

                        // Serialize the key as a string
                        let key = &(*key_ptr);
                        write!(writer, "\"{}\":", key.replace('"', "\\\""))?;

                        if indent {
                            write!(writer, " ")?;
                        }

                        // Serialize the value
                        serialize_value(value_ptr, value_shape.get(), writer, indent, level + 1)?;
                    }

                    // Deallocate the iterator
                    (vtable.iter_vtable.dealloc)(iter_ptr);

                    if !first && indent {
                        writeln!(writer)?;
                        write!(writer, "{:indent$}", "", indent = level * 2)?;
                    }
                }

                write!(writer, "}}")
            }
            // Add support for other shapes (Array, Transparent) as needed
            _ => write!(writer, "null"),
        }
    }

    serialize_value(data, shape_desc.get(), writer, indent, 0)
}

/// Serializes any Shapely type to JSON and returns it as a String
pub fn to_json_string(data: *mut u8, shape_desc: ShapeDesc, indent: bool) -> String {
    let mut buffer = Vec::new();
    to_json(data, shape_desc, &mut buffer, indent).unwrap();
    String::from_utf8(buffer).unwrap()
}
