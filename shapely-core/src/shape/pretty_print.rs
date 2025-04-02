use crate::{FieldFlags, TypeNameOpts, VariantKind};

use super::{Def, EnumDef, ListDef, MapDef, Shape, StructDef};
use std::{collections::HashSet, fmt::Formatter};

const INDENT: usize = 2;

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
            (self.vtable.type_name)(f, TypeNameOpts::one())?;
            writeln!(f, "\x1b[0m (\x1b[1;31malready printed\x1b[0m)")?;
            return Ok(());
        }

        write!(f, "{:indent$}\x1b[1;33m", "", indent = indent)?;
        (self.vtable.type_name)(f, TypeNameOpts::default())?;
        writeln!(f, "\x1b[0m (\x1b[1;34m{}\x1b[0m bytes)", self.layout.size())?;

        match &self.def {
            Def::Scalar => {}
            Def::Struct(StructDef { fields })
            | Def::TupleStruct(StructDef { fields })
            | Def::Tuple(StructDef { fields }) => {
                let max_name_length = fields.iter().map(|f| f.name.len()).max().unwrap_or(0);
                for field in *fields {
                    write!(
                        f,
                        "{:indent$}\x1b[1;34m{:>4}\x1b[0m \x1b[1;32m{:<width$}\x1b[0m ",
                        "",
                        field.offset,
                        field.name,
                        indent = indent + INDENT,
                        width = max_name_length
                    )?;
                    if field.flags.contains(FieldFlags::SENSITIVE) {
                        write!(f, "(sensitive) ")?;
                    }
                    if let Def::Scalar = field.shape.def {
                        field.shape.pretty_print_recursive_internal(
                            f,
                            printed_schemas,
                            indent + INDENT * 2,
                        )?;
                    } else {
                        writeln!(f)?;
                        field.shape.pretty_print_recursive_internal(
                            f,
                            printed_schemas,
                            indent + INDENT * 2,
                        )?;
                    }
                }
            }
            Def::Map(MapDef { k: _, v, vtable: _ }) => {
                writeln!(
                    f,
                    "{:indent$}\x1b[1;36mHashMap with arbitrary keys and value shape:\x1b[0m",
                    "",
                    indent = indent + INDENT
                )?;
                v.pretty_print_recursive_internal(f, printed_schemas, indent + INDENT * 2)?;
            }
            Def::List(ListDef { t, vtable: _ }) => {
                write!(
                    f,
                    "{:indent$}\x1b[1;36mArray of:\x1b[0m ",
                    "",
                    indent = indent + INDENT
                )?;
                t.pretty_print_recursive_internal(f, printed_schemas, indent + INDENT * 2)?;
            }
            Def::Enum(EnumDef { variants, repr: _ }) => {
                writeln!(
                    f,
                    "{:indent$}\x1b[1;36mEnum with {} variants:\x1b[0m",
                    "",
                    variants.len(),
                    indent = indent + INDENT
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
                                indent = indent + INDENT * 2
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
                                indent = indent + INDENT * 2
                            )?;

                            // Print field details
                            for field in fields.iter() {
                                write!(
                                    f,
                                    "{:indent$}\x1b[1;34m{:>4}\x1b[0m \x1b[1;32m{}\x1b[0m ",
                                    "",
                                    field.offset,
                                    field.name,
                                    indent = indent + INDENT * 3
                                )?;
                                if field.flags.contains(FieldFlags::SENSITIVE) {
                                    write!(f, "(sensitive) ")?;
                                }
                                if let Def::Scalar = field.shape.def {
                                    field.shape.pretty_print_recursive_internal(
                                        f,
                                        printed_schemas,
                                        indent + INDENT * 4,
                                    )?;
                                } else {
                                    writeln!(f)?;
                                    field.shape.pretty_print_recursive_internal(
                                        f,
                                        printed_schemas,
                                        indent + INDENT * 4,
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
                                indent = indent + INDENT * 2
                            )?;

                            // Print field details
                            for field in fields.iter() {
                                write!(
                                    f,
                                    "{:indent$}\x1b[1;34m{:>4}\x1b[0m \x1b[1;32m{}\x1b[0m ",
                                    "",
                                    field.offset,
                                    field.name,
                                    indent = indent + INDENT * 3
                                )?;
                                if field.flags.contains(FieldFlags::SENSITIVE) {
                                    write!(f, "(sensitive) ")?;
                                }
                                if let Def::Scalar = field.shape.def {
                                    field.shape.pretty_print_recursive_internal(
                                        f,
                                        printed_schemas,
                                        indent + INDENT * 4,
                                    )?;
                                } else {
                                    writeln!(f)?;
                                    field.shape.pretty_print_recursive_internal(
                                        f,
                                        printed_schemas,
                                        indent + INDENT * 4,
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
