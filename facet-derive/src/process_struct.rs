use super::*;

/// Processes a regular struct to implement Facet
///
/// Example input:
/// ```rust
/// struct Blah {
///     foo: u32,
///     bar: String,
/// }
/// ```
pub(crate) fn process_struct(parsed: Struct) -> proc_macro::TokenStream {
    let struct_name = parsed.name.to_string();

    // Generate field definitions
    let kind;
    let fields = match &parsed.kind {
        StructKind::Struct(body) => {
            kind = "facet::StructKind::Struct";
            body.content
                .0
                .iter()
                .map(|field| {
                    let field_name = field.value.name.to_string();
                    gen_struct_field(&field_name, &struct_name, &field.value.attributes)
                })
                .collect::<Vec<String>>()
        }
        StructKind::TupleStruct(body) => {
            kind = "facet::StructKind::TupleStruct";
            body.first
                .content
                .0
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let field_name = format!("{index}");
                    gen_struct_field(&field_name, &struct_name, &field.value.attributes)
                })
                .collect::<Vec<String>>()
        }
        StructKind::UnitStruct(_) => {
            kind = "facet::StructKind::Unit";
            vec![]
        }
    }
    .join(", ");

    let static_decl = generate_static_decl(&struct_name);
    let maybe_container_doc = build_maybe_doc(&parsed.attributes);

    // Generate the impl
    let output = format!(
        r#"
{static_decl}

#[automatically_derived]
unsafe impl facet::Facet for {struct_name} {{
    const SHAPE: &'static facet::Shape = &const {{
        static FIELDS: &[facet::Field] = &[{fields}];

        facet::Shape::builder()
            .id(facet::ConstTypeId::of::<{struct_name}>())
            .layout(core::alloc::Layout::new::<Self>())
            .vtable(facet::value_vtable!(
                {struct_name},
                |f, _opts| core::fmt::Write::write_str(f, "{struct_name}")
            ))
            .def(facet::Def::Struct(facet::StructDef::builder()
                .kind({kind})
                .fields(FIELDS)
                .build()))
            {maybe_container_doc}
            .build()
    }};
}}
        "#
    );

    output.into_token_stream().into()
}
