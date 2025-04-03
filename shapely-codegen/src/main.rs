use std::fmt::Write;

fn main() {
    // Check if current directory has a Cargo.toml with [workspace]
    let cargo_toml_path = std::env::current_dir().unwrap().join("Cargo.toml");
    let cargo_toml_content =
        std::fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
    if !cargo_toml_content.contains("[workspace]") {
        panic!(
            "Cargo.toml does not contain [workspace] (you must run codegen from the workspace root)"
        );
    }

    let mut output = String::new();
    let _ = codegen_tuple_impls(&mut output);
    let path = "shapely-core/src/impls/tuples_impls.rs";
    std::fs::write(path, output).expect("Failed to write file");

    // Run rustfmt on the generated file
    let status = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .arg(path)
        .status()
        .expect("Failed to execute rustfmt");

    if !status.success() {
        eprintln!("rustfmt failed with exit code: {}", status);
    }
}

fn codegen_tuple_impls(w: &mut dyn Write) -> std::fmt::Result {
    writeln!(w, "use std::alloc::Layout;")?;
    writeln!(w)?;
    writeln!(
        w,
        "use crate::{{Field, FieldFlags, Def, StructDef, Shape, Shapely, TypeNameOpts, value_vtable}};"
    )?;

    // Generate implementations for tuples of size 1 to 12
    for n in 1..=12 {
        let type_params = (0..n)
            .map(|i| format!("T{}", i))
            .collect::<Vec<_>>()
            .join(", ");

        let where_clause = (0..n)
            .map(|i| format!("T{}: Shapely", i))
            .collect::<Vec<_>>()
            .join(", ");

        let type_name_fn = format!(
            r#"fn type_name<{type_params}>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
            where {where_clause}
            {{
                if let Some(opts) = opts.for_children() {{
                    write!(f, "(")?;
                    {}
                    write!(f, ")")
                }} else {{
                    write!(f, "(⋯)")
                }}
            }}"#,
            (0..n)
                .map(|i| {
                    let prefix = if i > 0 { "write!(f, \", \")?; " } else { "" };
                    format!("{}(T{}::SHAPE.vtable.type_name)(f, opts)?;", prefix, i)
                })
                .collect::<Vec<_>>()
                .join("\n                    ")
        );

        let tuple = if n == 1 {
            "(T0,)".to_string()
        } else {
            format!("({})", type_params)
        };

        let field_macro = format!(
            r#"
        macro_rules! field {{
            ($idx:tt, $ty:ty) => {{
                Field {{
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!({tuple}, $idx),
                    flags: FieldFlags::EMPTY,
                }}
            }}
        }}
        "#
        );

        let fields = format!(
            "&const {{ [ {} ] }}",
            (0..n)
                .map(|i| format!("field!({}, T{})", i, i))
                .collect::<Vec<_>>()
                .join(",")
        );

        writeln!(
            w,
            r#"
            impl<{type_params}> Shapely for {tuple} where {where_clause}
            {{
                const SHAPE: &'static Shape = &const {{
                    use std::fmt;

                    {type_name_fn}

                    {field_macro}

                    Shape {{
                        layout: Layout::new::<{tuple}>(),
                        vtable: value_vtable!({tuple}, type_name::<{type_params}> as _),
                        def: Def::Tuple(StructDef {{
                            fields: {fields}
                        }}),
                    }}
                }};
            }}"#
        )?;
    }
    Ok(())
}
