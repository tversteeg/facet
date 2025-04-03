use shapely::Peek;
use std::fmt::Write as _;
use std::io::{self, Write};

/// Serializes any Shapely type to JSON
pub fn to_json<W: Write>(peek: Peek<'_>, writer: &mut W, indent: bool) -> io::Result<()> {
    fn serialize_value<W: Write>(
        peek: Peek<'_>,
        writer: &mut W,
        indent: bool,
        level: usize,
    ) -> io::Result<()> {
        match peek {
            Peek::Scalar(pv) => {
                // For scalar values, use debug or display based on type
                if let Some(debug) = pv.debug() {
                    let mut buf = String::new();
                    write!(&mut buf, "{:?}", debug).map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "failed to format value")
                    })?;

                    // Handle special cases for JSON formatting
                    let s = if buf.starts_with('"') {
                        // String values are already quoted
                        buf
                    } else if buf == "true" || buf == "false" {
                        buf
                    } else if buf == "()" {
                        "null".to_string()
                    } else {
                        buf
                    };
                    write!(writer, "{}", s)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "value must implement Debug",
                    ))
                }
            }
            Peek::Struct(ps) => {
                write!(writer, "{{")?;
                if indent {
                    writeln!(writer)?;
                }

                let mut first = true;
                let mut i = 0;
                while let Some(field) = ps.field_at(i) {
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
                    write!(writer, "\"{}\":", field.name())?;
                    if indent {
                        write!(writer, " ")?;
                    }

                    serialize_value(field.peek(), writer, indent, level + 1)?;
                    i += 1;
                }

                if !first && indent {
                    writeln!(writer)?;
                    write!(writer, "{:indent$}", "", indent = level * 2)?;
                }
                write!(writer, "}}")
            }
            Peek::List(pl) => {
                write!(writer, "[")?;
                if indent {
                    writeln!(writer)?;
                }

                let mut first = true;
                let mut i = 0;
                while let Some(item) = pl.next_item() {
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

                    serialize_value(item, writer, indent, level + 1)?;
                    i += 1;
                }

                if !first && indent {
                    writeln!(writer)?;
                    write!(writer, "{:indent$}", "", indent = level * 2)?;
                }
                write!(writer, "]")
            }
            Peek::Map(pm) => {
                write!(writer, "{{")?;
                if indent {
                    writeln!(writer)?;
                }

                let mut first = true;
                let mut i = 0;
                while let Some((key, value)) = pm.next_entry() {
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

                    // Handle key serialization using debug
                    if let Some(debug) = key.as_value().debug() {
                        let mut buf = String::new();
                        write!(&mut buf, "{:?}", debug).map_err(|_| {
                            io::Error::new(io::ErrorKind::InvalidData, "failed to format key")
                        })?;
                        // Remove surrounding quotes if present
                        let key_str = if buf.starts_with('"') && buf.ends_with('"') {
                            &buf[1..buf.len() - 1]
                        } else {
                            &buf
                        };
                        write!(writer, "\"{}\":", key_str)?;
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Map key must implement Debug",
                        ));
                    };

                    if indent {
                        write!(writer, " ")?;
                    }

                    serialize_value(value, writer, indent, level + 1)?;
                    i += 1;
                }

                if !first && indent {
                    writeln!(writer)?;
                    write!(writer, "{:indent$}", "", indent = level * 2)?;
                }

                write!(writer, "}}")
            }
        }
    }

    serialize_value(peek, writer, indent, 0)
}

/// Serializes any Shapely type to JSON and returns it as a String
pub fn to_json_string(peek: Peek<'_>, indent: bool) -> String {
    let mut buffer = Vec::new();
    to_json(peek, &mut buffer, indent).unwrap();
    String::from_utf8(buffer).unwrap()
}
