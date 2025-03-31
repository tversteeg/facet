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
    std::fs::write("shapely-core/src/tuples_impls.rs", output).expect("Failed to write file");

    // Run rustfmt on the generated file
    let status = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .arg("shapely-core/src/tuples_impls.rs")
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
        "use crate::{{Field, FieldFlags, Innards, Shape, ShapeDesc, Shapely, mini_typeid}};"
    )?;

    // Generate implementations for tuples of size 1 to 12
    for n in 1..=12 {
        let type_params = (0..n)
            .map(|i| format!("T{}", i))
            .collect::<Vec<_>>()
            .join(", ");

        let mut name_format = "write!(f, \"(\")?;".to_string();
        for i in 0..n {
            name_format += &format!("\n                (T{}::shape().name)(f)?;", i);
            if i < n - 1 {
                name_format += "\n                write!(f, \",\")?;";
            }
        }
        name_format += "\n                write!(f, \",)\")";

        let where_clause = (0..n)
            .map(|i| format!("T{}: Shapely", i))
            .collect::<Vec<_>>()
            .join(", ");

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
                    shape: ShapeDesc(<$ty>::shape),
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
            "
            impl<{type_params}> Shapely for {tuple} where {where_clause}
            {{
                fn shape() -> Shape {{
                    {field_macro}

                    Shape {{
                        name: |f| {{
                            {name_format}
                        }},
                        typeid: mini_typeid::of::<Self>(),
                        layout: Layout::new::<{tuple}>(),
                        innards: Innards::Tuple {{
                            fields: {fields}
                        }},
                        set_to_default: None,
                        drop_in_place: Some(|addr: *mut u8| unsafe {{
                            std::ptr::drop_in_place(addr as *mut {tuple});
                        }}),
                    }}
                }}
            }}"
        )?;
    }
    Ok(())
}
