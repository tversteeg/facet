use super::{Innards, Shape};
use std::{collections::HashSet, fmt::Formatter};

impl Shape {
    /// Pretty-print this shape, recursively.
    pub fn pretty_print_recursive(&self, f: &mut Formatter) -> std::fmt::Result {
        self.pretty_print_recursive_internal(f, &mut HashSet::new(), 0)
    }

    fn pretty_print_recursive_internal(
        &self,
        f: &mut Formatter,
        printed_schemas: &mut HashSet<Shape>,
        indent: usize,
    ) -> std::fmt::Result {
        if !printed_schemas.insert(*self) {
            write!(f, "{:indent$}\x1b[1;33m", "", indent = indent)?;
            (self.name)(f)?;
            writeln!(f, "\x1b[0m (\x1b[1;31malready printed\x1b[0m)")?;
            return Ok(());
        }

        write!(f, "{:indent$}\x1b[1;33m", "", indent = indent)?;
        (self.name)(f)?;
        writeln!(f, "\x1b[0m (\x1b[1;34m{}\x1b[0m bytes)", self.layout.size())?;

        match &self.innards {
            Innards::Struct { fields } => {
                let max_name_length = fields.iter().map(|f| f.name.len()).max().unwrap_or(0);
                for field in *fields {
                    write!(
                        f,
                        "{:indent$}\x1b[1;34m{:>4}\x1b[0m \x1b[1;32m{:<width$}\x1b[0m ",
                        "",
                        field.offset,
                        field.name,
                        indent = indent + Self::INDENT,
                        width = max_name_length
                    )?;
                    if field.flags.is_sensitive() {
                        write!(f, "(sensitive) ")?;
                    }
                    if let Innards::Scalar(_) = field.shape.get().innards {
                        field.shape.get().pretty_print_recursive_internal(
                            f,
                            printed_schemas,
                            indent + Self::INDENT * 2,
                        )?;
                    } else {
                        writeln!(f)?;
                        field.shape.get().pretty_print_recursive_internal(
                            f,
                            printed_schemas,
                            indent + Self::INDENT * 2,
                        )?;
                    }
                }
            }
            Innards::HashMap { value_shape } => {
                writeln!(
                    f,
                    "{:indent$}\x1b[1;36mHashMap with arbitrary keys and value shape:\x1b[0m",
                    "",
                    indent = indent + Self::INDENT
                )?;
                value_shape.get().pretty_print_recursive_internal(
                    f,
                    printed_schemas,
                    indent + Self::INDENT * 2,
                )?;
            }
            Innards::Array(elem_schema) => {
                write!(
                    f,
                    "{:indent$}\x1b[1;36mArray of:\x1b[0m ",
                    "",
                    indent = indent + Self::INDENT
                )?;
                elem_schema.get().pretty_print_recursive_internal(
                    f,
                    printed_schemas,
                    indent + Self::INDENT * 2,
                )?;
            }
            Innards::Transparent(inner_schema) => {
                write!(
                    f,
                    "{:indent$}\x1b[1;36mTransparent wrapper for:\x1b[0m ",
                    "",
                    indent = indent + Self::INDENT
                )?;
                inner_schema.get().pretty_print_recursive_internal(
                    f,
                    printed_schemas,
                    indent + Self::INDENT * 2,
                )?;
            }
            Innards::Scalar(_scalar) => {
                // let's not duplicate `u64 => U64` for example
            }
        }

        Ok(())
    }
}
