use shapely::{Shape, ShapeDesc};
use std::io::{self, Write};

pub fn to_json<W: Write>(
    data: *mut u8,
    shape_desc: ShapeDesc,
    writer: &mut W,
    indent: bool,
) -> io::Result<()> {
    use shapely::{Innards, Scalar};

    fn serialize_value<W: Write>(
        data: *mut u8,
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
                Scalar::U64 => {
                    let value = unsafe { *(data as *const u64) };
                    write!(writer, "{value}")
                }
                // Add other scalar types as needed
                _ => write!(writer, "null"),
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
            // Add support for other shapes (Array, Transparent) as needed
            _ => write!(writer, "null"),
        }
    }

    serialize_value(data, shape_desc.get(), writer, indent, 0)
}

pub fn to_json_string(data: *mut u8, shape_desc: ShapeDesc, indent: bool) -> String {
    let mut buffer = Vec::new();
    to_json(data, shape_desc, &mut buffer, indent).unwrap();
    String::from_utf8(buffer).unwrap()
}
