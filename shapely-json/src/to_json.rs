#![allow(unreachable_code)]

use shapely_poke::Peek;
use shapely_trait::ShapeExt as _;
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
            Peek::Value(pv) => {
                if pv.shape().is_type::<()>() {
                    write!(writer, "null")
                } else if pv.shape().is_type::<bool>() {
                    let value = unsafe { pv.data().as_ref::<bool>() };
                    write!(writer, "{}", value)
                } else if pv.shape().is_type::<u64>() {
                    let value = unsafe { pv.data().as_ref::<u64>() };
                    write!(writer, "{}", value)
                } else if pv.shape().is_type::<String>() {
                    let value = unsafe { pv.data().as_ref::<String>() };
                    write!(writer, "\"{}\"", value.escape_debug())
                } else {
                    // For other types, we'll use a placeholder
                    write!(writer, "\"<unsupported type>\"")
                }
            }
            Peek::Struct(ps) => {
                write!(writer, "{{")?;
                if indent {
                    writeln!(writer)?;
                }

                let mut first = true;
                for field in ps.fields() {
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
                    write!(writer, "\"{}\":", field.0)?;
                    if indent {
                        write!(writer, " ")?;
                    }

                    serialize_value(Peek::Value(field.1), writer, indent, level + 1)?;
                }

                if !first && indent {
                    writeln!(writer)?;
                    write!(writer, "{:indent$}", "", indent = level * 2)?;
                }
                write!(writer, "}}")
            }
            Peek::List(_pl) => {
                todo!("list");

                write!(writer, "[")?;
                if indent {
                    writeln!(writer)?;
                }

                let mut first = true;
                let mut _index = 0;
                // while let Some(item) = pl.item_at(index) {
                //     if !first {
                //         write!(writer, ",")?;
                //         if indent {
                //             writeln!(writer)?;
                //         }
                //     }
                //     first = false;

                //     if indent {
                //         write!(writer, "{:indent$}", "", indent = (level + 1) * 2)?;
                //     }

                //     serialize_value(item, writer, indent, level + 1)?;
                //     index += 1;
                // }

                if !first && indent {
                    writeln!(writer)?;
                    write!(writer, "{:indent$}", "", indent = level * 2)?;
                }
                write!(writer, "]")
            }
            Peek::Map(_pm) => {
                todo!("map");

                write!(writer, "{{")?;
                if indent {
                    writeln!(writer)?;
                }

                let mut first = true;
                let mut _index = 0;
                // while let Some((key, value)) = pm.entry_at(index) {
                //     if !first {
                //         write!(writer, ",")?;
                //         if indent {
                //             writeln!(writer)?;
                //         }
                //     }
                //     first = false;

                //     if indent {
                //         write!(writer, "{:indent$}", "", indent = (level + 1) * 2)?;
                //     }

                //     // Handle key serialization
                //     serialize_value(Peek::Scalar(key), writer, false, 0)?;
                //     write!(writer, ":")?;

                //     if indent {
                //         write!(writer, " ")?;
                //     }

                //     serialize_value(value, writer, indent, level + 1)?;
                //     index += 1;
                // }

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
