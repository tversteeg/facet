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
            (self.vtable().type_name)(f, TypeNameOpts::one())?;
            writeln!(f, "\x1b[0m (\x1b[1;31malready printed\x1b[0m)")?;
            return Ok(());
        }

        write!(f, "{:indent$}\x1b[1;33m", "", indent = indent)?;
        (self.vtable().type_name)(f, TypeNameOpts::default())?;
        writeln!(f, "\x1b[0m (\x1b[1;34m{}\x1b[0m bytes)", self.layout.size())?;

        match &self.innards {
            Innards::Struct { fields }
            | Innards::TupleStruct { fields }
            | Innards::Tuple { fields } => {
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
                    if field.flags.contains(super::struct_::FieldFlags::SENSITIVE) {
                        write!(f, "(sensitive) ")?;
                    }
                    if let Innards::Scalar = field.shape.get().innards {
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
            Innards::Map { k: _, v, vtable: _ } => {
                writeln!(
                    f,
                    "{:indent$}\x1b[1;36mHashMap with arbitrary keys and value shape:\x1b[0m",
                    "",
                    indent = indent + Self::INDENT
                )?;
                v.get().pretty_print_recursive_internal(
                    f,
                    printed_schemas,
                    indent + Self::INDENT * 2,
                )?;
            }
            Innards::List { t, vtable: _ } => {
                write!(
                    f,
                    "{:indent$}\x1b[1;36mArray of:\x1b[0m ",
                    "",
                    indent = indent + Self::INDENT
                )?;
                t.get().pretty_print_recursive_internal(
                    f,
                    printed_schemas,
                    indent + Self::INDENT * 2,
                )?;
            }
            Innards::Scalar => {
                // let's not duplicate `u64 => U64` for example
            }
            Innards::Enum { variants, repr: _ } => {
                writeln!(
                    f,
                    "{:indent$}\x1b[1;36mEnum with {} variants:\x1b[0m",
                    "",
                    variants.len(),
                    indent = indent + Self::INDENT
                )?;

                for variant in *variants {
                    match &variant.kind {
                        VariantKind::Unit => {
                            let disc_str = if let Some(disc) = variant.discriminant {
                                format!(" = {}", disc)
                            } else {
                                String::new()
                            };
                            writeln!(
                                f,
                                "{:indent$}\x1b[1;32m{}{}\x1b[0m",
                                "",
                                variant.name,
                                disc_str,
                                indent = indent + Self::INDENT * 2
                            )?;
                        }
                        VariantKind::Tuple { fields } => {
                            writeln!(
                                f,
                                "{:indent$}\x1b[1;32m{}\x1b[0m({})",
                                "",
                                variant.name,
                                (0..fields.len())
                                    .map(|_| "_")
                                    .collect::<Vec<_>>()
                                    .join(", "),
                                indent = indent + Self::INDENT * 2
                            )?;

                            // Print field details
                            for field in *fields {
                                write!(
                                    f,
                                    "{:indent$}\x1b[1;34m{:>4}\x1b[0m \x1b[1;32m{}\x1b[0m ",
                                    "",
                                    field.offset,
                                    field.name,
                                    indent = indent + Self::INDENT * 3
                                )?;
                                if field.flags.contains(super::struct_::FieldFlags::SENSITIVE) {
                                    write!(f, "(sensitive) ")?;
                                }
                                if let Innards::Scalar = field.shape.get().innards {
                                    field.shape.get().pretty_print_recursive_internal(
                                        f,
                                        printed_schemas,
                                        indent + Self::INDENT * 4,
                                    )?;
                                } else {
                                    writeln!(f)?;
                                    field.shape.get().pretty_print_recursive_internal(
                                        f,
                                        printed_schemas,
                                        indent + Self::INDENT * 4,
                                    )?;
                                }
                            }
                        }
                        VariantKind::Struct { fields } => {
                            writeln!(
                                f,
                                "{:indent$}\x1b[1;32m{}\x1b[0m {{ {} }}",
                                "",
                                variant.name,
                                fields.iter().map(|f| f.name).collect::<Vec<_>>().join(", "),
                                indent = indent + Self::INDENT * 2
                            )?;

                            // Print field details
                            for field in *fields {
                                write!(
                                    f,
                                    "{:indent$}\x1b[1;34m{:>4}\x1b[0m \x1b[1;32m{}\x1b[0m ",
                                    "",
                                    field.offset,
                                    field.name,
                                    indent = indent + Self::INDENT * 3
                                )?;
                                if field.flags.contains(super::struct_::FieldFlags::SENSITIVE) {
                                    write!(f, "(sensitive) ")?;
                                }
                                if let Innards::Scalar = field.shape.get().innards {
                                    field.shape.get().pretty_print_recursive_internal(
                                        f,
                                        printed_schemas,
                                        indent + Self::INDENT * 4,
                                    )?;
                                } else {
                                    writeln!(f)?;
                                    field.shape.get().pretty_print_recursive_internal(
                                        f,
                                        printed_schemas,
                                        indent + Self::INDENT * 4,
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
